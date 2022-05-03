  extern crate monoasm;
  extern crate monoasm_macro;
  use std::io::Write;

  use monoasm::*;
  use monoasm_macro::monoasm;

  #[test]
  fn subq() {
      let mut jit: JitMemory = JitMemory::new();
      monoasm!(
          jit,
	subq rax, rax;
	subq rax, rcx;
	subq rax, rdx;
	subq rax, rbx;
	subq rax, rsp;
	subq rax, rbp;
	subq rax, rsi;
	subq rax, rdi;
	subq rax, r8;
	subq rax, r9;
	subq rax, r10;
	subq rax, r11;
	subq rax, r12;
	subq rax, r13;
	subq rax, r14;
	subq rax, r15;

	subq rcx, rax;
	subq rcx, rcx;
	subq rcx, rdx;
	subq rcx, rbx;
	subq rcx, rsp;
	subq rcx, rbp;
	subq rcx, rsi;
	subq rcx, rdi;
	subq rcx, r8;
	subq rcx, r9;
	subq rcx, r10;
	subq rcx, r11;
	subq rcx, r12;
	subq rcx, r13;
	subq rcx, r14;
	subq rcx, r15;

	subq rdx, rax;
	subq rdx, rcx;
	subq rdx, rdx;
	subq rdx, rbx;
	subq rdx, rsp;
	subq rdx, rbp;
	subq rdx, rsi;
	subq rdx, rdi;
	subq rdx, r8;
	subq rdx, r9;
	subq rdx, r10;
	subq rdx, r11;
	subq rdx, r12;
	subq rdx, r13;
	subq rdx, r14;
	subq rdx, r15;

	subq rbx, rax;
	subq rbx, rcx;
	subq rbx, rdx;
	subq rbx, rbx;
	subq rbx, rsp;
	subq rbx, rbp;
	subq rbx, rsi;
	subq rbx, rdi;
	subq rbx, r8;
	subq rbx, r9;
	subq rbx, r10;
	subq rbx, r11;
	subq rbx, r12;
	subq rbx, r13;
	subq rbx, r14;
	subq rbx, r15;

	subq rsp, rax;
	subq rsp, rcx;
	subq rsp, rdx;
	subq rsp, rbx;
	subq rsp, rsp;
	subq rsp, rbp;
	subq rsp, rsi;
	subq rsp, rdi;
	subq rsp, r8;
	subq rsp, r9;
	subq rsp, r10;
	subq rsp, r11;
	subq rsp, r12;
	subq rsp, r13;
	subq rsp, r14;
	subq rsp, r15;

	subq rbp, rax;
	subq rbp, rcx;
	subq rbp, rdx;
	subq rbp, rbx;
	subq rbp, rsp;
	subq rbp, rbp;
	subq rbp, rsi;
	subq rbp, rdi;
	subq rbp, r8;
	subq rbp, r9;
	subq rbp, r10;
	subq rbp, r11;
	subq rbp, r12;
	subq rbp, r13;
	subq rbp, r14;
	subq rbp, r15;

	subq rsi, rax;
	subq rsi, rcx;
	subq rsi, rdx;
	subq rsi, rbx;
	subq rsi, rsp;
	subq rsi, rbp;
	subq rsi, rsi;
	subq rsi, rdi;
	subq rsi, r8;
	subq rsi, r9;
	subq rsi, r10;
	subq rsi, r11;
	subq rsi, r12;
	subq rsi, r13;
	subq rsi, r14;
	subq rsi, r15;

	subq rdi, rax;
	subq rdi, rcx;
	subq rdi, rdx;
	subq rdi, rbx;
	subq rdi, rsp;
	subq rdi, rbp;
	subq rdi, rsi;
	subq rdi, rdi;
	subq rdi, r8;
	subq rdi, r9;
	subq rdi, r10;
	subq rdi, r11;
	subq rdi, r12;
	subq rdi, r13;
	subq rdi, r14;
	subq rdi, r15;

	subq r8, rax;
	subq r8, rcx;
	subq r8, rdx;
	subq r8, rbx;
	subq r8, rsp;
	subq r8, rbp;
	subq r8, rsi;
	subq r8, rdi;
	subq r8, r8;
	subq r8, r9;
	subq r8, r10;
	subq r8, r11;
	subq r8, r12;
	subq r8, r13;
	subq r8, r14;
	subq r8, r15;

	subq r9, rax;
	subq r9, rcx;
	subq r9, rdx;
	subq r9, rbx;
	subq r9, rsp;
	subq r9, rbp;
	subq r9, rsi;
	subq r9, rdi;
	subq r9, r8;
	subq r9, r9;
	subq r9, r10;
	subq r9, r11;
	subq r9, r12;
	subq r9, r13;
	subq r9, r14;
	subq r9, r15;

	subq r10, rax;
	subq r10, rcx;
	subq r10, rdx;
	subq r10, rbx;
	subq r10, rsp;
	subq r10, rbp;
	subq r10, rsi;
	subq r10, rdi;
	subq r10, r8;
	subq r10, r9;
	subq r10, r10;
	subq r10, r11;
	subq r10, r12;
	subq r10, r13;
	subq r10, r14;
	subq r10, r15;

	subq r11, rax;
	subq r11, rcx;
	subq r11, rdx;
	subq r11, rbx;
	subq r11, rsp;
	subq r11, rbp;
	subq r11, rsi;
	subq r11, rdi;
	subq r11, r8;
	subq r11, r9;
	subq r11, r10;
	subq r11, r11;
	subq r11, r12;
	subq r11, r13;
	subq r11, r14;
	subq r11, r15;

	subq r12, rax;
	subq r12, rcx;
	subq r12, rdx;
	subq r12, rbx;
	subq r12, rsp;
	subq r12, rbp;
	subq r12, rsi;
	subq r12, rdi;
	subq r12, r8;
	subq r12, r9;
	subq r12, r10;
	subq r12, r11;
	subq r12, r12;
	subq r12, r13;
	subq r12, r14;
	subq r12, r15;

	subq r13, rax;
	subq r13, rcx;
	subq r13, rdx;
	subq r13, rbx;
	subq r13, rsp;
	subq r13, rbp;
	subq r13, rsi;
	subq r13, rdi;
	subq r13, r8;
	subq r13, r9;
	subq r13, r10;
	subq r13, r11;
	subq r13, r12;
	subq r13, r13;
	subq r13, r14;
	subq r13, r15;

	subq r14, rax;
	subq r14, rcx;
	subq r14, rdx;
	subq r14, rbx;
	subq r14, rsp;
	subq r14, rbp;
	subq r14, rsi;
	subq r14, rdi;
	subq r14, r8;
	subq r14, r9;
	subq r14, r10;
	subq r14, r11;
	subq r14, r12;
	subq r14, r13;
	subq r14, r14;
	subq r14, r15;

	subq r15, rax;
	subq r15, rcx;
	subq r15, rdx;
	subq r15, rbx;
	subq r15, rsp;
	subq r15, rbp;
	subq r15, rsi;
	subq r15, rdi;
	subq r15, r8;
	subq r15, r9;
	subq r15, r10;
	subq r15, r11;
	subq r15, r12;
	subq r15, r13;
	subq r15, r14;
	subq r15, r15;


	subq rax, 16;
	subq [rax], 16;
	subq [rax + 16], 16;
	subq rcx, 16;
	subq [rcx], 16;
	subq [rcx + 16], 16;
	subq rdx, 16;
	subq [rdx], 16;
	subq [rdx + 16], 16;
	subq rbx, 16;
	subq [rbx], 16;
	subq [rbx + 16], 16;
	subq rsp, 16;
	subq [rsp], 16;
	subq [rsp + 16], 16;
	subq rbp, 16;
	subq [rbp], 16;
	subq [rbp + 16], 16;
	subq rsi, 16;
	subq [rsi], 16;
	subq [rsi + 16], 16;
	subq rdi, 16;
	subq [rdi], 16;
	subq [rdi + 16], 16;
	subq r8, 16;
	subq [r8], 16;
	subq [r8 + 16], 16;
	subq r9, 16;
	subq [r9], 16;
	subq [r9 + 16], 16;
	subq r10, 16;
	subq [r10], 16;
	subq [r10 + 16], 16;
	subq r11, 16;
	subq [r11], 16;
	subq [r11 + 16], 16;
	subq r12, 16;
	subq [r12], 16;
	subq [r12 + 16], 16;
	subq r13, 16;
	subq [r13], 16;
	subq [r13 + 16], 16;
	subq r14, 16;
	subq [r14], 16;
	subq [r14 + 16], 16;
	subq r15, 16;
	subq [r15], 16;
	subq [r15 + 16], 16;

	subq [rax], rax;
	subq [rax + 16], rax;
	subq [rax], rcx;
	subq [rax + 16], rcx;
	subq [rax], rdx;
	subq [rax + 16], rdx;
	subq [rax], rbx;
	subq [rax + 16], rbx;
	subq [rax], rsp;
	subq [rax + 16], rsp;
	subq [rax], rbp;
	subq [rax + 16], rbp;
	subq [rax], rsi;
	subq [rax + 16], rsi;
	subq [rax], rdi;
	subq [rax + 16], rdi;
	subq [rax], r8;
	subq [rax + 16], r8;
	subq [rax], r9;
	subq [rax + 16], r9;
	subq [rax], r10;
	subq [rax + 16], r10;
	subq [rax], r11;
	subq [rax + 16], r11;
	subq [rax], r12;
	subq [rax + 16], r12;
	subq [rax], r13;
	subq [rax + 16], r13;
	subq [rax], r14;
	subq [rax + 16], r14;
	subq [rax], r15;
	subq [rax + 16], r15;

	subq [rcx], rax;
	subq [rcx + 16], rax;
	subq [rcx], rcx;
	subq [rcx + 16], rcx;
	subq [rcx], rdx;
	subq [rcx + 16], rdx;
	subq [rcx], rbx;
	subq [rcx + 16], rbx;
	subq [rcx], rsp;
	subq [rcx + 16], rsp;
	subq [rcx], rbp;
	subq [rcx + 16], rbp;
	subq [rcx], rsi;
	subq [rcx + 16], rsi;
	subq [rcx], rdi;
	subq [rcx + 16], rdi;
	subq [rcx], r8;
	subq [rcx + 16], r8;
	subq [rcx], r9;
	subq [rcx + 16], r9;
	subq [rcx], r10;
	subq [rcx + 16], r10;
	subq [rcx], r11;
	subq [rcx + 16], r11;
	subq [rcx], r12;
	subq [rcx + 16], r12;
	subq [rcx], r13;
	subq [rcx + 16], r13;
	subq [rcx], r14;
	subq [rcx + 16], r14;
	subq [rcx], r15;
	subq [rcx + 16], r15;

	subq [rdx], rax;
	subq [rdx + 16], rax;
	subq [rdx], rcx;
	subq [rdx + 16], rcx;
	subq [rdx], rdx;
	subq [rdx + 16], rdx;
	subq [rdx], rbx;
	subq [rdx + 16], rbx;
	subq [rdx], rsp;
	subq [rdx + 16], rsp;
	subq [rdx], rbp;
	subq [rdx + 16], rbp;
	subq [rdx], rsi;
	subq [rdx + 16], rsi;
	subq [rdx], rdi;
	subq [rdx + 16], rdi;
	subq [rdx], r8;
	subq [rdx + 16], r8;
	subq [rdx], r9;
	subq [rdx + 16], r9;
	subq [rdx], r10;
	subq [rdx + 16], r10;
	subq [rdx], r11;
	subq [rdx + 16], r11;
	subq [rdx], r12;
	subq [rdx + 16], r12;
	subq [rdx], r13;
	subq [rdx + 16], r13;
	subq [rdx], r14;
	subq [rdx + 16], r14;
	subq [rdx], r15;
	subq [rdx + 16], r15;

	subq [rbx], rax;
	subq [rbx + 16], rax;
	subq [rbx], rcx;
	subq [rbx + 16], rcx;
	subq [rbx], rdx;
	subq [rbx + 16], rdx;
	subq [rbx], rbx;
	subq [rbx + 16], rbx;
	subq [rbx], rsp;
	subq [rbx + 16], rsp;
	subq [rbx], rbp;
	subq [rbx + 16], rbp;
	subq [rbx], rsi;
	subq [rbx + 16], rsi;
	subq [rbx], rdi;
	subq [rbx + 16], rdi;
	subq [rbx], r8;
	subq [rbx + 16], r8;
	subq [rbx], r9;
	subq [rbx + 16], r9;
	subq [rbx], r10;
	subq [rbx + 16], r10;
	subq [rbx], r11;
	subq [rbx + 16], r11;
	subq [rbx], r12;
	subq [rbx + 16], r12;
	subq [rbx], r13;
	subq [rbx + 16], r13;
	subq [rbx], r14;
	subq [rbx + 16], r14;
	subq [rbx], r15;
	subq [rbx + 16], r15;

	subq [rsp], rax;
	subq [rsp + 16], rax;
	subq [rsp], rcx;
	subq [rsp + 16], rcx;
	subq [rsp], rdx;
	subq [rsp + 16], rdx;
	subq [rsp], rbx;
	subq [rsp + 16], rbx;
	subq [rsp], rsp;
	subq [rsp + 16], rsp;
	subq [rsp], rbp;
	subq [rsp + 16], rbp;
	subq [rsp], rsi;
	subq [rsp + 16], rsi;
	subq [rsp], rdi;
	subq [rsp + 16], rdi;
	subq [rsp], r8;
	subq [rsp + 16], r8;
	subq [rsp], r9;
	subq [rsp + 16], r9;
	subq [rsp], r10;
	subq [rsp + 16], r10;
	subq [rsp], r11;
	subq [rsp + 16], r11;
	subq [rsp], r12;
	subq [rsp + 16], r12;
	subq [rsp], r13;
	subq [rsp + 16], r13;
	subq [rsp], r14;
	subq [rsp + 16], r14;
	subq [rsp], r15;
	subq [rsp + 16], r15;

	subq [rbp], rax;
	subq [rbp + 16], rax;
	subq [rbp], rcx;
	subq [rbp + 16], rcx;
	subq [rbp], rdx;
	subq [rbp + 16], rdx;
	subq [rbp], rbx;
	subq [rbp + 16], rbx;
	subq [rbp], rsp;
	subq [rbp + 16], rsp;
	subq [rbp], rbp;
	subq [rbp + 16], rbp;
	subq [rbp], rsi;
	subq [rbp + 16], rsi;
	subq [rbp], rdi;
	subq [rbp + 16], rdi;
	subq [rbp], r8;
	subq [rbp + 16], r8;
	subq [rbp], r9;
	subq [rbp + 16], r9;
	subq [rbp], r10;
	subq [rbp + 16], r10;
	subq [rbp], r11;
	subq [rbp + 16], r11;
	subq [rbp], r12;
	subq [rbp + 16], r12;
	subq [rbp], r13;
	subq [rbp + 16], r13;
	subq [rbp], r14;
	subq [rbp + 16], r14;
	subq [rbp], r15;
	subq [rbp + 16], r15;

	subq [rsi], rax;
	subq [rsi + 16], rax;
	subq [rsi], rcx;
	subq [rsi + 16], rcx;
	subq [rsi], rdx;
	subq [rsi + 16], rdx;
	subq [rsi], rbx;
	subq [rsi + 16], rbx;
	subq [rsi], rsp;
	subq [rsi + 16], rsp;
	subq [rsi], rbp;
	subq [rsi + 16], rbp;
	subq [rsi], rsi;
	subq [rsi + 16], rsi;
	subq [rsi], rdi;
	subq [rsi + 16], rdi;
	subq [rsi], r8;
	subq [rsi + 16], r8;
	subq [rsi], r9;
	subq [rsi + 16], r9;
	subq [rsi], r10;
	subq [rsi + 16], r10;
	subq [rsi], r11;
	subq [rsi + 16], r11;
	subq [rsi], r12;
	subq [rsi + 16], r12;
	subq [rsi], r13;
	subq [rsi + 16], r13;
	subq [rsi], r14;
	subq [rsi + 16], r14;
	subq [rsi], r15;
	subq [rsi + 16], r15;

	subq [rdi], rax;
	subq [rdi + 16], rax;
	subq [rdi], rcx;
	subq [rdi + 16], rcx;
	subq [rdi], rdx;
	subq [rdi + 16], rdx;
	subq [rdi], rbx;
	subq [rdi + 16], rbx;
	subq [rdi], rsp;
	subq [rdi + 16], rsp;
	subq [rdi], rbp;
	subq [rdi + 16], rbp;
	subq [rdi], rsi;
	subq [rdi + 16], rsi;
	subq [rdi], rdi;
	subq [rdi + 16], rdi;
	subq [rdi], r8;
	subq [rdi + 16], r8;
	subq [rdi], r9;
	subq [rdi + 16], r9;
	subq [rdi], r10;
	subq [rdi + 16], r10;
	subq [rdi], r11;
	subq [rdi + 16], r11;
	subq [rdi], r12;
	subq [rdi + 16], r12;
	subq [rdi], r13;
	subq [rdi + 16], r13;
	subq [rdi], r14;
	subq [rdi + 16], r14;
	subq [rdi], r15;
	subq [rdi + 16], r15;

	subq [r8], rax;
	subq [r8 + 16], rax;
	subq [r8], rcx;
	subq [r8 + 16], rcx;
	subq [r8], rdx;
	subq [r8 + 16], rdx;
	subq [r8], rbx;
	subq [r8 + 16], rbx;
	subq [r8], rsp;
	subq [r8 + 16], rsp;
	subq [r8], rbp;
	subq [r8 + 16], rbp;
	subq [r8], rsi;
	subq [r8 + 16], rsi;
	subq [r8], rdi;
	subq [r8 + 16], rdi;
	subq [r8], r8;
	subq [r8 + 16], r8;
	subq [r8], r9;
	subq [r8 + 16], r9;
	subq [r8], r10;
	subq [r8 + 16], r10;
	subq [r8], r11;
	subq [r8 + 16], r11;
	subq [r8], r12;
	subq [r8 + 16], r12;
	subq [r8], r13;
	subq [r8 + 16], r13;
	subq [r8], r14;
	subq [r8 + 16], r14;
	subq [r8], r15;
	subq [r8 + 16], r15;

	subq [r9], rax;
	subq [r9 + 16], rax;
	subq [r9], rcx;
	subq [r9 + 16], rcx;
	subq [r9], rdx;
	subq [r9 + 16], rdx;
	subq [r9], rbx;
	subq [r9 + 16], rbx;
	subq [r9], rsp;
	subq [r9 + 16], rsp;
	subq [r9], rbp;
	subq [r9 + 16], rbp;
	subq [r9], rsi;
	subq [r9 + 16], rsi;
	subq [r9], rdi;
	subq [r9 + 16], rdi;
	subq [r9], r8;
	subq [r9 + 16], r8;
	subq [r9], r9;
	subq [r9 + 16], r9;
	subq [r9], r10;
	subq [r9 + 16], r10;
	subq [r9], r11;
	subq [r9 + 16], r11;
	subq [r9], r12;
	subq [r9 + 16], r12;
	subq [r9], r13;
	subq [r9 + 16], r13;
	subq [r9], r14;
	subq [r9 + 16], r14;
	subq [r9], r15;
	subq [r9 + 16], r15;

	subq [r10], rax;
	subq [r10 + 16], rax;
	subq [r10], rcx;
	subq [r10 + 16], rcx;
	subq [r10], rdx;
	subq [r10 + 16], rdx;
	subq [r10], rbx;
	subq [r10 + 16], rbx;
	subq [r10], rsp;
	subq [r10 + 16], rsp;
	subq [r10], rbp;
	subq [r10 + 16], rbp;
	subq [r10], rsi;
	subq [r10 + 16], rsi;
	subq [r10], rdi;
	subq [r10 + 16], rdi;
	subq [r10], r8;
	subq [r10 + 16], r8;
	subq [r10], r9;
	subq [r10 + 16], r9;
	subq [r10], r10;
	subq [r10 + 16], r10;
	subq [r10], r11;
	subq [r10 + 16], r11;
	subq [r10], r12;
	subq [r10 + 16], r12;
	subq [r10], r13;
	subq [r10 + 16], r13;
	subq [r10], r14;
	subq [r10 + 16], r14;
	subq [r10], r15;
	subq [r10 + 16], r15;

	subq [r11], rax;
	subq [r11 + 16], rax;
	subq [r11], rcx;
	subq [r11 + 16], rcx;
	subq [r11], rdx;
	subq [r11 + 16], rdx;
	subq [r11], rbx;
	subq [r11 + 16], rbx;
	subq [r11], rsp;
	subq [r11 + 16], rsp;
	subq [r11], rbp;
	subq [r11 + 16], rbp;
	subq [r11], rsi;
	subq [r11 + 16], rsi;
	subq [r11], rdi;
	subq [r11 + 16], rdi;
	subq [r11], r8;
	subq [r11 + 16], r8;
	subq [r11], r9;
	subq [r11 + 16], r9;
	subq [r11], r10;
	subq [r11 + 16], r10;
	subq [r11], r11;
	subq [r11 + 16], r11;
	subq [r11], r12;
	subq [r11 + 16], r12;
	subq [r11], r13;
	subq [r11 + 16], r13;
	subq [r11], r14;
	subq [r11 + 16], r14;
	subq [r11], r15;
	subq [r11 + 16], r15;

	subq [r12], rax;
	subq [r12 + 16], rax;
	subq [r12], rcx;
	subq [r12 + 16], rcx;
	subq [r12], rdx;
	subq [r12 + 16], rdx;
	subq [r12], rbx;
	subq [r12 + 16], rbx;
	subq [r12], rsp;
	subq [r12 + 16], rsp;
	subq [r12], rbp;
	subq [r12 + 16], rbp;
	subq [r12], rsi;
	subq [r12 + 16], rsi;
	subq [r12], rdi;
	subq [r12 + 16], rdi;
	subq [r12], r8;
	subq [r12 + 16], r8;
	subq [r12], r9;
	subq [r12 + 16], r9;
	subq [r12], r10;
	subq [r12 + 16], r10;
	subq [r12], r11;
	subq [r12 + 16], r11;
	subq [r12], r12;
	subq [r12 + 16], r12;
	subq [r12], r13;
	subq [r12 + 16], r13;
	subq [r12], r14;
	subq [r12 + 16], r14;
	subq [r12], r15;
	subq [r12 + 16], r15;

	subq [r13], rax;
	subq [r13 + 16], rax;
	subq [r13], rcx;
	subq [r13 + 16], rcx;
	subq [r13], rdx;
	subq [r13 + 16], rdx;
	subq [r13], rbx;
	subq [r13 + 16], rbx;
	subq [r13], rsp;
	subq [r13 + 16], rsp;
	subq [r13], rbp;
	subq [r13 + 16], rbp;
	subq [r13], rsi;
	subq [r13 + 16], rsi;
	subq [r13], rdi;
	subq [r13 + 16], rdi;
	subq [r13], r8;
	subq [r13 + 16], r8;
	subq [r13], r9;
	subq [r13 + 16], r9;
	subq [r13], r10;
	subq [r13 + 16], r10;
	subq [r13], r11;
	subq [r13 + 16], r11;
	subq [r13], r12;
	subq [r13 + 16], r12;
	subq [r13], r13;
	subq [r13 + 16], r13;
	subq [r13], r14;
	subq [r13 + 16], r14;
	subq [r13], r15;
	subq [r13 + 16], r15;

	subq [r14], rax;
	subq [r14 + 16], rax;
	subq [r14], rcx;
	subq [r14 + 16], rcx;
	subq [r14], rdx;
	subq [r14 + 16], rdx;
	subq [r14], rbx;
	subq [r14 + 16], rbx;
	subq [r14], rsp;
	subq [r14 + 16], rsp;
	subq [r14], rbp;
	subq [r14 + 16], rbp;
	subq [r14], rsi;
	subq [r14 + 16], rsi;
	subq [r14], rdi;
	subq [r14 + 16], rdi;
	subq [r14], r8;
	subq [r14 + 16], r8;
	subq [r14], r9;
	subq [r14 + 16], r9;
	subq [r14], r10;
	subq [r14 + 16], r10;
	subq [r14], r11;
	subq [r14 + 16], r11;
	subq [r14], r12;
	subq [r14 + 16], r12;
	subq [r14], r13;
	subq [r14 + 16], r13;
	subq [r14], r14;
	subq [r14 + 16], r14;
	subq [r14], r15;
	subq [r14 + 16], r15;

	subq [r15], rax;
	subq [r15 + 16], rax;
	subq [r15], rcx;
	subq [r15 + 16], rcx;
	subq [r15], rdx;
	subq [r15 + 16], rdx;
	subq [r15], rbx;
	subq [r15 + 16], rbx;
	subq [r15], rsp;
	subq [r15 + 16], rsp;
	subq [r15], rbp;
	subq [r15 + 16], rbp;
	subq [r15], rsi;
	subq [r15 + 16], rsi;
	subq [r15], rdi;
	subq [r15 + 16], rdi;
	subq [r15], r8;
	subq [r15 + 16], r8;
	subq [r15], r9;
	subq [r15 + 16], r9;
	subq [r15], r10;
	subq [r15 + 16], r10;
	subq [r15], r11;
	subq [r15 + 16], r11;
	subq [r15], r12;
	subq [r15 + 16], r12;
	subq [r15], r13;
	subq [r15 + 16], r13;
	subq [r15], r14;
	subq [r15 + 16], r14;
	subq [r15], r15;
	subq [r15 + 16], r15;


	subq rax, [rax];
	subq rax, [rax + 16];
	subq rax, [rcx];
	subq rax, [rcx + 16];
	subq rax, [rdx];
	subq rax, [rdx + 16];
	subq rax, [rbx];
	subq rax, [rbx + 16];
	subq rax, [rsp];
	subq rax, [rsp + 16];
	subq rax, [rbp];
	subq rax, [rbp + 16];
	subq rax, [rsi];
	subq rax, [rsi + 16];
	subq rax, [rdi];
	subq rax, [rdi + 16];
	subq rax, [r8];
	subq rax, [r8 + 16];
	subq rax, [r9];
	subq rax, [r9 + 16];
	subq rax, [r10];
	subq rax, [r10 + 16];
	subq rax, [r11];
	subq rax, [r11 + 16];
	subq rax, [r12];
	subq rax, [r12 + 16];
	subq rax, [r13];
	subq rax, [r13 + 16];
	subq rax, [r14];
	subq rax, [r14 + 16];
	subq rax, [r15];
	subq rax, [r15 + 16];

	subq rcx, [rax];
	subq rcx, [rax + 16];
	subq rcx, [rcx];
	subq rcx, [rcx + 16];
	subq rcx, [rdx];
	subq rcx, [rdx + 16];
	subq rcx, [rbx];
	subq rcx, [rbx + 16];
	subq rcx, [rsp];
	subq rcx, [rsp + 16];
	subq rcx, [rbp];
	subq rcx, [rbp + 16];
	subq rcx, [rsi];
	subq rcx, [rsi + 16];
	subq rcx, [rdi];
	subq rcx, [rdi + 16];
	subq rcx, [r8];
	subq rcx, [r8 + 16];
	subq rcx, [r9];
	subq rcx, [r9 + 16];
	subq rcx, [r10];
	subq rcx, [r10 + 16];
	subq rcx, [r11];
	subq rcx, [r11 + 16];
	subq rcx, [r12];
	subq rcx, [r12 + 16];
	subq rcx, [r13];
	subq rcx, [r13 + 16];
	subq rcx, [r14];
	subq rcx, [r14 + 16];
	subq rcx, [r15];
	subq rcx, [r15 + 16];

	subq rdx, [rax];
	subq rdx, [rax + 16];
	subq rdx, [rcx];
	subq rdx, [rcx + 16];
	subq rdx, [rdx];
	subq rdx, [rdx + 16];
	subq rdx, [rbx];
	subq rdx, [rbx + 16];
	subq rdx, [rsp];
	subq rdx, [rsp + 16];
	subq rdx, [rbp];
	subq rdx, [rbp + 16];
	subq rdx, [rsi];
	subq rdx, [rsi + 16];
	subq rdx, [rdi];
	subq rdx, [rdi + 16];
	subq rdx, [r8];
	subq rdx, [r8 + 16];
	subq rdx, [r9];
	subq rdx, [r9 + 16];
	subq rdx, [r10];
	subq rdx, [r10 + 16];
	subq rdx, [r11];
	subq rdx, [r11 + 16];
	subq rdx, [r12];
	subq rdx, [r12 + 16];
	subq rdx, [r13];
	subq rdx, [r13 + 16];
	subq rdx, [r14];
	subq rdx, [r14 + 16];
	subq rdx, [r15];
	subq rdx, [r15 + 16];

	subq rbx, [rax];
	subq rbx, [rax + 16];
	subq rbx, [rcx];
	subq rbx, [rcx + 16];
	subq rbx, [rdx];
	subq rbx, [rdx + 16];
	subq rbx, [rbx];
	subq rbx, [rbx + 16];
	subq rbx, [rsp];
	subq rbx, [rsp + 16];
	subq rbx, [rbp];
	subq rbx, [rbp + 16];
	subq rbx, [rsi];
	subq rbx, [rsi + 16];
	subq rbx, [rdi];
	subq rbx, [rdi + 16];
	subq rbx, [r8];
	subq rbx, [r8 + 16];
	subq rbx, [r9];
	subq rbx, [r9 + 16];
	subq rbx, [r10];
	subq rbx, [r10 + 16];
	subq rbx, [r11];
	subq rbx, [r11 + 16];
	subq rbx, [r12];
	subq rbx, [r12 + 16];
	subq rbx, [r13];
	subq rbx, [r13 + 16];
	subq rbx, [r14];
	subq rbx, [r14 + 16];
	subq rbx, [r15];
	subq rbx, [r15 + 16];

	subq rsp, [rax];
	subq rsp, [rax + 16];
	subq rsp, [rcx];
	subq rsp, [rcx + 16];
	subq rsp, [rdx];
	subq rsp, [rdx + 16];
	subq rsp, [rbx];
	subq rsp, [rbx + 16];
	subq rsp, [rsp];
	subq rsp, [rsp + 16];
	subq rsp, [rbp];
	subq rsp, [rbp + 16];
	subq rsp, [rsi];
	subq rsp, [rsi + 16];
	subq rsp, [rdi];
	subq rsp, [rdi + 16];
	subq rsp, [r8];
	subq rsp, [r8 + 16];
	subq rsp, [r9];
	subq rsp, [r9 + 16];
	subq rsp, [r10];
	subq rsp, [r10 + 16];
	subq rsp, [r11];
	subq rsp, [r11 + 16];
	subq rsp, [r12];
	subq rsp, [r12 + 16];
	subq rsp, [r13];
	subq rsp, [r13 + 16];
	subq rsp, [r14];
	subq rsp, [r14 + 16];
	subq rsp, [r15];
	subq rsp, [r15 + 16];

	subq rbp, [rax];
	subq rbp, [rax + 16];
	subq rbp, [rcx];
	subq rbp, [rcx + 16];
	subq rbp, [rdx];
	subq rbp, [rdx + 16];
	subq rbp, [rbx];
	subq rbp, [rbx + 16];
	subq rbp, [rsp];
	subq rbp, [rsp + 16];
	subq rbp, [rbp];
	subq rbp, [rbp + 16];
	subq rbp, [rsi];
	subq rbp, [rsi + 16];
	subq rbp, [rdi];
	subq rbp, [rdi + 16];
	subq rbp, [r8];
	subq rbp, [r8 + 16];
	subq rbp, [r9];
	subq rbp, [r9 + 16];
	subq rbp, [r10];
	subq rbp, [r10 + 16];
	subq rbp, [r11];
	subq rbp, [r11 + 16];
	subq rbp, [r12];
	subq rbp, [r12 + 16];
	subq rbp, [r13];
	subq rbp, [r13 + 16];
	subq rbp, [r14];
	subq rbp, [r14 + 16];
	subq rbp, [r15];
	subq rbp, [r15 + 16];

	subq rsi, [rax];
	subq rsi, [rax + 16];
	subq rsi, [rcx];
	subq rsi, [rcx + 16];
	subq rsi, [rdx];
	subq rsi, [rdx + 16];
	subq rsi, [rbx];
	subq rsi, [rbx + 16];
	subq rsi, [rsp];
	subq rsi, [rsp + 16];
	subq rsi, [rbp];
	subq rsi, [rbp + 16];
	subq rsi, [rsi];
	subq rsi, [rsi + 16];
	subq rsi, [rdi];
	subq rsi, [rdi + 16];
	subq rsi, [r8];
	subq rsi, [r8 + 16];
	subq rsi, [r9];
	subq rsi, [r9 + 16];
	subq rsi, [r10];
	subq rsi, [r10 + 16];
	subq rsi, [r11];
	subq rsi, [r11 + 16];
	subq rsi, [r12];
	subq rsi, [r12 + 16];
	subq rsi, [r13];
	subq rsi, [r13 + 16];
	subq rsi, [r14];
	subq rsi, [r14 + 16];
	subq rsi, [r15];
	subq rsi, [r15 + 16];

	subq rdi, [rax];
	subq rdi, [rax + 16];
	subq rdi, [rcx];
	subq rdi, [rcx + 16];
	subq rdi, [rdx];
	subq rdi, [rdx + 16];
	subq rdi, [rbx];
	subq rdi, [rbx + 16];
	subq rdi, [rsp];
	subq rdi, [rsp + 16];
	subq rdi, [rbp];
	subq rdi, [rbp + 16];
	subq rdi, [rsi];
	subq rdi, [rsi + 16];
	subq rdi, [rdi];
	subq rdi, [rdi + 16];
	subq rdi, [r8];
	subq rdi, [r8 + 16];
	subq rdi, [r9];
	subq rdi, [r9 + 16];
	subq rdi, [r10];
	subq rdi, [r10 + 16];
	subq rdi, [r11];
	subq rdi, [r11 + 16];
	subq rdi, [r12];
	subq rdi, [r12 + 16];
	subq rdi, [r13];
	subq rdi, [r13 + 16];
	subq rdi, [r14];
	subq rdi, [r14 + 16];
	subq rdi, [r15];
	subq rdi, [r15 + 16];

	subq r8, [rax];
	subq r8, [rax + 16];
	subq r8, [rcx];
	subq r8, [rcx + 16];
	subq r8, [rdx];
	subq r8, [rdx + 16];
	subq r8, [rbx];
	subq r8, [rbx + 16];
	subq r8, [rsp];
	subq r8, [rsp + 16];
	subq r8, [rbp];
	subq r8, [rbp + 16];
	subq r8, [rsi];
	subq r8, [rsi + 16];
	subq r8, [rdi];
	subq r8, [rdi + 16];
	subq r8, [r8];
	subq r8, [r8 + 16];
	subq r8, [r9];
	subq r8, [r9 + 16];
	subq r8, [r10];
	subq r8, [r10 + 16];
	subq r8, [r11];
	subq r8, [r11 + 16];
	subq r8, [r12];
	subq r8, [r12 + 16];
	subq r8, [r13];
	subq r8, [r13 + 16];
	subq r8, [r14];
	subq r8, [r14 + 16];
	subq r8, [r15];
	subq r8, [r15 + 16];

	subq r9, [rax];
	subq r9, [rax + 16];
	subq r9, [rcx];
	subq r9, [rcx + 16];
	subq r9, [rdx];
	subq r9, [rdx + 16];
	subq r9, [rbx];
	subq r9, [rbx + 16];
	subq r9, [rsp];
	subq r9, [rsp + 16];
	subq r9, [rbp];
	subq r9, [rbp + 16];
	subq r9, [rsi];
	subq r9, [rsi + 16];
	subq r9, [rdi];
	subq r9, [rdi + 16];
	subq r9, [r8];
	subq r9, [r8 + 16];
	subq r9, [r9];
	subq r9, [r9 + 16];
	subq r9, [r10];
	subq r9, [r10 + 16];
	subq r9, [r11];
	subq r9, [r11 + 16];
	subq r9, [r12];
	subq r9, [r12 + 16];
	subq r9, [r13];
	subq r9, [r13 + 16];
	subq r9, [r14];
	subq r9, [r14 + 16];
	subq r9, [r15];
	subq r9, [r15 + 16];

	subq r10, [rax];
	subq r10, [rax + 16];
	subq r10, [rcx];
	subq r10, [rcx + 16];
	subq r10, [rdx];
	subq r10, [rdx + 16];
	subq r10, [rbx];
	subq r10, [rbx + 16];
	subq r10, [rsp];
	subq r10, [rsp + 16];
	subq r10, [rbp];
	subq r10, [rbp + 16];
	subq r10, [rsi];
	subq r10, [rsi + 16];
	subq r10, [rdi];
	subq r10, [rdi + 16];
	subq r10, [r8];
	subq r10, [r8 + 16];
	subq r10, [r9];
	subq r10, [r9 + 16];
	subq r10, [r10];
	subq r10, [r10 + 16];
	subq r10, [r11];
	subq r10, [r11 + 16];
	subq r10, [r12];
	subq r10, [r12 + 16];
	subq r10, [r13];
	subq r10, [r13 + 16];
	subq r10, [r14];
	subq r10, [r14 + 16];
	subq r10, [r15];
	subq r10, [r15 + 16];

	subq r11, [rax];
	subq r11, [rax + 16];
	subq r11, [rcx];
	subq r11, [rcx + 16];
	subq r11, [rdx];
	subq r11, [rdx + 16];
	subq r11, [rbx];
	subq r11, [rbx + 16];
	subq r11, [rsp];
	subq r11, [rsp + 16];
	subq r11, [rbp];
	subq r11, [rbp + 16];
	subq r11, [rsi];
	subq r11, [rsi + 16];
	subq r11, [rdi];
	subq r11, [rdi + 16];
	subq r11, [r8];
	subq r11, [r8 + 16];
	subq r11, [r9];
	subq r11, [r9 + 16];
	subq r11, [r10];
	subq r11, [r10 + 16];
	subq r11, [r11];
	subq r11, [r11 + 16];
	subq r11, [r12];
	subq r11, [r12 + 16];
	subq r11, [r13];
	subq r11, [r13 + 16];
	subq r11, [r14];
	subq r11, [r14 + 16];
	subq r11, [r15];
	subq r11, [r15 + 16];

	subq r12, [rax];
	subq r12, [rax + 16];
	subq r12, [rcx];
	subq r12, [rcx + 16];
	subq r12, [rdx];
	subq r12, [rdx + 16];
	subq r12, [rbx];
	subq r12, [rbx + 16];
	subq r12, [rsp];
	subq r12, [rsp + 16];
	subq r12, [rbp];
	subq r12, [rbp + 16];
	subq r12, [rsi];
	subq r12, [rsi + 16];
	subq r12, [rdi];
	subq r12, [rdi + 16];
	subq r12, [r8];
	subq r12, [r8 + 16];
	subq r12, [r9];
	subq r12, [r9 + 16];
	subq r12, [r10];
	subq r12, [r10 + 16];
	subq r12, [r11];
	subq r12, [r11 + 16];
	subq r12, [r12];
	subq r12, [r12 + 16];
	subq r12, [r13];
	subq r12, [r13 + 16];
	subq r12, [r14];
	subq r12, [r14 + 16];
	subq r12, [r15];
	subq r12, [r15 + 16];

	subq r13, [rax];
	subq r13, [rax + 16];
	subq r13, [rcx];
	subq r13, [rcx + 16];
	subq r13, [rdx];
	subq r13, [rdx + 16];
	subq r13, [rbx];
	subq r13, [rbx + 16];
	subq r13, [rsp];
	subq r13, [rsp + 16];
	subq r13, [rbp];
	subq r13, [rbp + 16];
	subq r13, [rsi];
	subq r13, [rsi + 16];
	subq r13, [rdi];
	subq r13, [rdi + 16];
	subq r13, [r8];
	subq r13, [r8 + 16];
	subq r13, [r9];
	subq r13, [r9 + 16];
	subq r13, [r10];
	subq r13, [r10 + 16];
	subq r13, [r11];
	subq r13, [r11 + 16];
	subq r13, [r12];
	subq r13, [r12 + 16];
	subq r13, [r13];
	subq r13, [r13 + 16];
	subq r13, [r14];
	subq r13, [r14 + 16];
	subq r13, [r15];
	subq r13, [r15 + 16];

	subq r14, [rax];
	subq r14, [rax + 16];
	subq r14, [rcx];
	subq r14, [rcx + 16];
	subq r14, [rdx];
	subq r14, [rdx + 16];
	subq r14, [rbx];
	subq r14, [rbx + 16];
	subq r14, [rsp];
	subq r14, [rsp + 16];
	subq r14, [rbp];
	subq r14, [rbp + 16];
	subq r14, [rsi];
	subq r14, [rsi + 16];
	subq r14, [rdi];
	subq r14, [rdi + 16];
	subq r14, [r8];
	subq r14, [r8 + 16];
	subq r14, [r9];
	subq r14, [r9 + 16];
	subq r14, [r10];
	subq r14, [r10 + 16];
	subq r14, [r11];
	subq r14, [r11 + 16];
	subq r14, [r12];
	subq r14, [r12 + 16];
	subq r14, [r13];
	subq r14, [r13 + 16];
	subq r14, [r14];
	subq r14, [r14 + 16];
	subq r14, [r15];
	subq r14, [r15 + 16];

	subq r15, [rax];
	subq r15, [rax + 16];
	subq r15, [rcx];
	subq r15, [rcx + 16];
	subq r15, [rdx];
	subq r15, [rdx + 16];
	subq r15, [rbx];
	subq r15, [rbx + 16];
	subq r15, [rsp];
	subq r15, [rsp + 16];
	subq r15, [rbp];
	subq r15, [rbp + 16];
	subq r15, [rsi];
	subq r15, [rsi + 16];
	subq r15, [rdi];
	subq r15, [rdi + 16];
	subq r15, [r8];
	subq r15, [r8 + 16];
	subq r15, [r9];
	subq r15, [r9 + 16];
	subq r15, [r10];
	subq r15, [r10 + 16];
	subq r15, [r11];
	subq r15, [r11 + 16];
	subq r15, [r12];
	subq r15, [r12 + 16];
	subq r15, [r13];
	subq r15, [r13 + 16];
	subq r15, [r14];
	subq r15, [r14 + 16];
	subq r15, [r15];
	subq r15, [r15 + 16];


      );
      jit.finalize();
      let mut buf = std::fs::File::create("tests/subq_monoasm.bin").unwrap();
      buf.write_all(jit.as_slice()).unwrap();
  }