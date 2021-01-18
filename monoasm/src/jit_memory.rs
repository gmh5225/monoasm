//--------------------
//
// JIT module runtime
//
//--------------------

use crate::*;
use monoasm_inst::{util, Mode, Reg};
use region::{protect, Protection};
use std::alloc::{alloc, Layout};

/// Memory manager.
pub struct JitMemory {
    /// Pointer to the heap.
    contents: *mut u8,
    /// Current position
    counter: Pos,
    /// Current label id.
    label_count: usize,
    /// Relocation information.
    reloc: Relocations,
}

impl Index<Pos> for JitMemory {
    type Output = u8;

    fn index(&self, index: Pos) -> &u8 {
        if index.0 >= PAGE_SIZE {
            panic!("Page size overflow")
        }
        unsafe { &*self.contents.offset(index.0 as isize) }
    }
}

impl IndexMut<Pos> for JitMemory {
    fn index_mut(&mut self, index: Pos) -> &mut u8 {
        if index.0 >= PAGE_SIZE {
            panic!("Page size overflow")
        }
        unsafe { &mut *self.contents.offset(index.0 as isize) }
    }
}

impl JitMemory {
    /// Create new JitMemory.
    ///
    /// This function try to allocate heap memory of 4KB for JIT assemble.
    ///
    /// ### panic
    /// Panic if Layout::from_size_align() or region::protect() returned Err.
    pub fn new() -> JitMemory {
        let size = 4096;
        let layout = Layout::from_size_align(size, PAGE_SIZE).expect("Bad Layout.");
        let contents = unsafe { alloc(layout) };

        unsafe {
            protect(contents, size, Protection::READ_WRITE_EXECUTE).expect("Mprotect failed.");
        }
        let mut res = JitMemory {
            contents,
            counter: Pos(0),
            label_count: 0,
            reloc: Relocations::new(),
        };
        res.emitb(0xc3);
        res.counter = Pos(0);
        res
    }

    fn _p(&self) {
        for i in 0..self.counter.0 {
            print!("{:>02x} ", self[Pos(i)]);
        }
        println!();
    }

    /// Get top address of the heap-allocated memory.
    pub fn get_mem_addr(&self) -> u64 {
        self.contents as u64
    }

    /// Create new label and returns `DestLabel`.
    pub fn label(&mut self) -> DestLabel {
        let label = self.label_count;
        self.label_count += 1;
        self.reloc.push(Reloc::new());
        DestLabel(label)
    }

    /// Bind the current location to `DestLabel`.
    pub fn bind_label(&mut self, label: DestLabel) {
        self.reloc[label].loc = Some(self.counter);
    }

    /// Save relocaton slot for `DestLabel`.
    pub fn save_reloc(&mut self, dest: DestLabel, size: u8) {
        self.reloc[dest].disp.push((size, self.counter));
    }

    /// Resolve all relocations and return the top addresss of generated machine code as a function pointer.
    pub fn finalize<T, U>(&mut self) -> fn(T) -> U {
        let mut relocs: Vec<(Pos, i32)> = vec![];
        for rel in self.reloc.iter_mut() {
            let pos = rel.loc.expect("Reloc not determined.");
            for (size, dest) in &mut rel.disp {
                let disp = pos.0 as i64 - dest.0 as i64 - *size as i64;
                if i32::min_value() as i64 > disp || disp > i32::max_value() as i64 {
                    panic!("Relocation overflow");
                }
                relocs.push((*dest, disp as i32));
            }
        }
        for (dest, disp) in relocs {
            self.write32(dest, disp);
        }
        unsafe { mem::transmute(self.contents) }
    }

    /// Emit a byte.
    pub fn emitb(&mut self, val: u8) {
        let c = self.counter;
        self[c] = val;
        self.counter = c + 1;
    }

    /// Emit a word.
    pub fn emitw(&mut self, val: u16) {
        let c = self.counter;
        self[c] = val as u8;
        self[c + 1] = (val >> 8) as u8;
        self.counter = c + 2;
    }

    /// Emit a long word.
    pub fn emitl(&mut self, val: u32) {
        let c = self.counter;
        self[c] = val as u8;
        self[c + 1] = (val >> 8) as u8;
        self[c + 2] = (val >> 16) as u8;
        self[c + 3] = (val >> 24) as u8;
        self.counter = c + 4;
    }

    /// Write 32bit data `val` on `loc`.
    fn write32(&mut self, loc: Pos, val: i32) {
        let val = val as u32;
        self[loc] = val as u8;
        self[loc + 1] = (val >> 8) as u8;
        self[loc + 2] = (val >> 16) as u8;
        self[loc + 3] = (val >> 24) as u8;
    }

    /// Emit a quad word.
    pub fn emitq(&mut self, val: u64) {
        self.emitl(val as u32);
        self.emitl((val >> 32) as u32);
    }
}

#[allow(dead_code)]
impl JitMemory {
    /// Encoding: Opcode + rd  
    /// REX.W Op+ rd
    pub fn enc_o(&mut self, op: u8, reg: Reg) {
        self.rexw(Reg::none(), reg, Reg::none());
        self.op_with_rd(op, reg);
    }

    /// Encoding: MI  
    /// ModRM:r/m
    pub fn enc_mi(&mut self, op: u8, rm_op: Or) {
        self.enc_mr(op, Reg::none(), rm_op)
    }

    /// Encoding: MR or RM
    /// MR-> ModRM:r/m(w) ModRM:reg(r)
    /// RM-> ModRM:reg(r) ModRM:r/m(w)
    pub fn enc_mr(&mut self, op: u8, reg: Reg, rm_op: Or) {
        let (mode, rm, disp) = op_to_rm(rm_op);
        self.enc_mr_main(op, reg, mode, rm);
        // TODO: If mode == Ind and r/m == 5, becomes [rip + disp32].
        self.imm_to_ts(disp, mode);
    }

    fn enc_mr_main(&mut self, op: u8, reg: Reg, mode: Mode, rm: Reg) {
        if mode != Mode::Reg && (rm == Reg::Rsp || rm == Reg::R12) {
            // TODO: If mode != Reg and r/m == 4 (rsp/r12), use SIB.
            // Currently, only Mode::Ind is supported.
            assert!(mode == Mode::Ind);
            // set index to 4 when [rm] is to be used.
            let index = Reg::Rsp; // magic number
            let scale = 0;
            self.rexw(reg, rm, index);
            self.emitb(op);
            self.modrm(reg, mode, rm);
            self.sib(scale, index, rm as u8);
        } else {
            self.rexw(reg, rm, Reg::none());
            self.emitb(op);
            self.modrm(reg, mode, rm);
        }
    }

    fn modrm_digit(&mut self, digit: u8, mode: Mode, rm: Reg) {
        self.emitb(util::modrm_digit(digit, mode, rm));
    }

    fn modrm(&mut self, reg: Reg, mode: Mode, rm: Reg) {
        self.emitb(util::modrm(reg, mode, rm));
    }

    fn rexw(&mut self, reg: Reg, base: Reg, index: Reg) {
        self.emitb(util::rexw(reg, base, index));
    }

    fn rex(&mut self, reg: Reg, base: Reg, index: Reg) {
        if let Some(rex_prefix) = util::rex(reg, base, index) {
            self.emitb(rex_prefix);
        }
    }

    fn op_with_rd(&mut self, op: u8, r: Reg) {
        self.emitb(util::op_with_rd(op, r));
    }

    fn sib(&mut self, scale: u8, index: Reg, base: u8) {
        self.emitb(util::sib(scale, index, base));
    }

    fn imm_to_ts(&mut self, imm: Option<i32>, mode: Mode) {
        match imm {
            Some(imm) => match mode {
                Mode::InD8 => self.emitb(imm as i8 as u8),
                Mode::InD32 => self.emitl(imm as u32),
                _ => unreachable!(),
            },
            None => {}
        }
    }
}

fn op_to_rm(op: Or) -> (Mode, Reg, Option<i32>) {
    match op {
        Or::Reg(r) => (Mode::Reg, r, None),
        Or::Ind(r) => (Mode::Ind, r, None),
        Or::IndD8(r, d) => (Mode::InD8, r, Some(d as i32)),
        Or::IndD32(r, d) => (Mode::InD32, r, Some(d)),
        rm_op => unreachable!("as_rm():{:?}", rm_op),
    }
}