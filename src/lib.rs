
mod scan;
mod token;
mod parse;
mod ast;
mod codegen;
mod typechecker;

pub enum CompileResult {
    Program(String),
    ParseError(parse::ParseError),
    ScanError(String)
}

pub fn compile(program: &str) -> CompileResult {
    match scan::scan(&program) {
        scan::ScanResult::Tokens(tokens) => {
            match parse::parse(&tokens) {
                parse::ParseResult::AST(ast) => {
                    let asm = codegen::gen_code(ast);
                    CompileResult::Program(asm)
                }
                parse::ParseResult::Error(error) => {
                    return CompileResult::ParseError(error);
                }
            }
        }
        scan::ScanResult::Error(error) => {
            return CompileResult::ScanError(error);
        }
    }
}