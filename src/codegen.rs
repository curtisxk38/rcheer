use crate::ast::{Binary, BinaryOp, Expr, Literal, Unary, UnaryOp};

pub struct CodeGenerator {
    bb_label_counter: i32
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {bb_label_counter: 0}
    }

    pub fn gen_code(&mut self, ast: Expr) -> String {
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

        self.visit_expr(&ast, &mut program);

        format!("{}{}{}", preamble, program, postamble) 
    }

    fn visit_expr(&mut self, node: &Expr, program: &mut String) {
        match node {
            Expr::Binary(binary) => {self.visit_binary(binary, program)}
            Expr::Literal(literal) => {self.visit_literal(literal, program)}
            Expr::Unary(unary) => {self.visit_unary(unary, program)}
            Expr::Grouping(grouping) => {self.visit_expr(grouping.expr.as_ref(), program)}
        }
    }

    fn visit_binary(&mut self, node: &Binary, program: &mut String) {
        self.visit_expr(node.left.as_ref(), program);
        self.visit_expr(node.right.as_ref(), program);
        match node.operation {
            BinaryOp::Add => self.binary_arithmetic("addq", program),
            BinaryOp::Minus => self.binary_arithmetic("subq", program),
            BinaryOp::Times => self.binary_arithmetic("imulq", program),
            BinaryOp::Less => self.binary_compare("jl", program),
            BinaryOp::BangEqual => self.binary_compare("jne", program),
            BinaryOp::EqualEqual => self.binary_compare("je", program),
            BinaryOp::Greater => self.binary_compare("jg", program),
            BinaryOp::GreaterEqual => self.binary_compare("jge", program),
            BinaryOp::LessEqual => self.binary_compare("jle", program),
        };   
    }

    fn binary_arithmetic(&mut self, op_instr: &str, program: &mut String) {
        program.push_str(
            format!("
            popq %rdx
            popq %rax
            {} %rdx, %rax
            pushq %rax", op_instr).as_str()
        )
    }

    fn binary_compare(&mut self, jump_instr: &str, program: &mut String) {
        program.push_str(
            format!("
            popq %rdx
            popq %rax
            cmpq %rdx, %rax
            {jump_instr} CMP_TRUE{label}
            CMP_FALSE{label}:
            pushq $0
            jmp CMP_DONE{label}
            CMP_TRUE{label}:
            pushq $1
            CMP_DONE{label}:
            ", jump_instr=jump_instr, label=self.bb_label_counter).as_str()
        );
        self.bb_label_counter += 1;
    }

    fn visit_unary(&mut self, node: &Unary, program: &mut String) {
        self.visit_expr(node.right.as_ref(), program);
        program.push_str(
            format!("
            popq %rax
            {}
            pushq %rax
            ",
            match node.operation {
                UnaryOp::Minus => "imulq $-1, %rax"
            }
            ).as_str()
        );
        
    }

    fn visit_literal(&mut self, node: &Literal, program: &mut String) {
        program.push_str(
            format!("
            pushq ${}   
            ", node.token.lexeme).as_str()
        )
    }
}