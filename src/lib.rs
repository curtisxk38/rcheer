use typechecker::TypeError;


mod scan;
mod token;
mod parse;
mod ast;
mod codegen;
mod typechecker;

pub enum CompileResult {
    Program(String),
    ParseError(parse::ParseError),
    TypeError(Vec<TypeError>),
    ScanError(String)
}

pub fn compile(program: &str) -> CompileResult {
    let mut scanner = scan::Scanner::new();
    match scanner.scan(&program) {
        scan::ScanResult::Tokens(tokens) => {
            match parse::parse(&tokens) {
                parse::ParseResult::AST(mut ast) => {
                    let mut typechecker = typechecker::TypeChecker::new();
                    match typechecker.typecheck(&mut ast) {
                        typechecker::TypeResult::Success => {
                            let mut code_generator = codegen::CodeGenerator::new();
                            let asm = code_generator.gen_code(ast);
                            CompileResult::Program(asm)
                        }
                        typechecker::TypeResult::Error => {
                            return CompileResult::TypeError(typechecker.errors);
                        }
                    }
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