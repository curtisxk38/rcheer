use crate::ast::{Binary, BinaryOp, Expr, Literal};

pub fn gen_code(ast: Expr) -> String {
    let mut program = String::new();
    
    let preamble = r#"	.file	"test.c"
	.text
	.globl	main
	.type	main, @function
    main:
    .LFB0:"#;

    let postamble = r#"
    popq %rax
    ret
    .LFE0:
        .size	main, .-main
        .ident	"GCC: (Ubuntu 9.3.0-17ubuntu1~20.04) 9.3.0"
        .section	.note.GNU-stack,"",@progbits
        .section	.note.gnu.property,"a"
        .align 8
        .long	 1f - 0f
        .long	 4f - 1f
        .long	 5
    0:
        .string	 "GNU"
    1:
        .align 8
        .long	 0xc0000002
        .long	 3f - 2f
    2:
        .long	 0x3
    3:
        .align 8
    4:
    
    "#;

    visit_expr(&ast, &mut program);

    format!("{}{}{}", preamble, program, postamble) 
}

fn visit_expr(node: &Expr, program: &mut String) {
    match node {
        Expr::Binary(binary) => {visit_binary(binary, program)}
        Expr::Literal(literal) => {visit_literal(literal, program)}
    }
}

fn visit_binary(node: &Binary, program: &mut String) {
    visit_expr(node.left.as_ref(), program);
    visit_expr(node.right.as_ref(), program);
    let op_instr = match node.operation {
        BinaryOp::Add => "addq"
    };
    program.push_str(
        format!("popq %rax
        popq %rdx
        {} %rdx, %rax
        pushq %rax", op_instr).as_str()
    )
}

fn visit_literal(node: &Literal, program: &mut String) {
    program.push_str(
        format!("
        pushq ${}   
        ", node.token.lexeme).as_str()
    )
}