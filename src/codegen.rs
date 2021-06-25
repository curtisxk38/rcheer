use crate::ast::{Binary, BinaryOp, Expr, If, Literal, Unary, UnaryOp};

pub struct CodeGenerator {
    bb_label_counter: i32
}

impl CodeGenerator {
    pub fn new() -> CodeGenerator {
        CodeGenerator {bb_label_counter: 0}
    }

    pub fn gen_code(&mut self, ast: Expr) -> String {
        let mut program = String::new();
        
        let preamble = "\t.file	\"test.c\"\n\
        \t.text\n\
        \t.globl\tmain\n\
        \t.type\tmain, @function\n\
        \tmain:\n\
        \t.LFB0:\n";

        let postamble = "\t\tpopq %rax\n\
        \t\tret\n\
        \t.LFE0:\n\
        \t    .size	main, .-main\n\
        \t    .ident	\"GCC: (Ubuntu 9.3.0-17ubuntu1~20.04) 9.3.0\"\n\
        \t    .section	.note.GNU-stack,\"\",@progbits\n\
        \t    .section	.note.gnu.property,\"a\"\n\
        \t    .align 8\n\
        \t    .long	 1f - 0f\n\
        \t    .long	 4f - 1f\n\
        \t    .long	 5\n\
        \t0:\n\
        \t    .string	 \"GNU\"\n\
        \t1:\n\
        \t    .align 8\n\
        \t    .long	 0xc0000002\n\
        \t    .long	 3f - 2f\n\
        \t2:\n\
        \t    .long	 0x3\n\
        \t3:\n\
        \t    .align 8\n\
        \t4:\n\
        ";

        self.visit_expr(&ast, &mut program);

        format!("{}{}{}", preamble, program, postamble) 
    }

    fn visit_expr(&mut self, node: &Expr, program: &mut String) {
        match node {
            Expr::Binary(binary) => {self.visit_binary(binary, program)}
            Expr::Literal(literal) => {self.visit_literal(literal, program)}
            Expr::Unary(unary) => {self.visit_unary(unary, program)}
            Expr::Grouping(grouping) => {self.visit_expr(grouping.expr.as_ref(), program)}
            Expr::If(if_expr) => {self.visit_if_expr(if_expr, program)},
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
            format!("\
            \t\tpopq %rdx\n\
            \t\tpopq %rax\n\
            \t\t{} %rdx, %rax\n\
            \t\tpushq %rax\n\
            ", op_instr).as_str()
        )
    }

    fn binary_compare(&mut self, jump_instr: &str, program: &mut String) {
        program.push_str(
            format!("\
            \t\tpopq %rdx\n\
            \t\tpopq %rax\n\
            \t\tcmpq %rdx, %rax\n\
            \t\t{jump_instr} CMP_TRUE{label}\n\
            \tCMP_FALSE{label}:\n\
            \t\tpushq $0\n\
            \t\tjmp CMP_DONE{label}\n\
            \tCMP_TRUE{label}:\n\
            \t\tpushq $1\n\
            \tCMP_DONE{label}:\n\
            ", jump_instr=jump_instr, label=self.bb_label_counter).as_str()
        );
        self.bb_label_counter += 1;
    }

    fn visit_unary(&mut self, node: &Unary, program: &mut String) {
        self.visit_expr(node.right.as_ref(), program);
        program.push_str(
            format!("\
            \t\tpopq %rax\n\
            \t\t{}\n\
            \t\tpushq %rax\n\
            ",
            match node.operation {
                UnaryOp::Minus => "imulq $-1, %rax"
            }
            ).as_str()
        );
        
    }

    fn visit_literal(&mut self, node: &Literal, program: &mut String) {
        program.push_str(
            format!("\t\tpushq ${}\n", node.token.lexeme).as_str()
        )
    }

    fn visit_if_expr(&mut self, node: &If, program: &mut String) {
        todo!()
    }
}