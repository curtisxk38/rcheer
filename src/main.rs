use std::{env, fs::{self, File}, io::Write};

mod scan;
mod token;
mod parse;
mod ast;

enum CompileResult {
    Program(String),
    Error
}

fn compile(program: String) -> CompileResult {
    match scan::scan(&program) {
        scan::ScanResult::Tokens(tokens) => {
            match parse::parse(&tokens) {
                parse::ParseResult::AST(ast) => {

                }
                parse::ParseResult::Error => {
                    return CompileResult::Error;
                }
            }
        }
        scan::ScanResult::Error(error) => {
            return CompileResult::Error;
        }
    }
    CompileResult::Program(String::new())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("usage: rcheer [filename]");
    } else {
        let filename = &args[1];
        let contents = fs::read_to_string(filename)
            .expect("Something went wrong reading the file");
        let result = compile(contents);
        let output = "output.s";
        match result {
            CompileResult::Program(asm) => {
                let mut file = File::create(output)
                    .expect(format!("Unable to create output file: {}", output).as_str());
                file.write_all(asm.as_bytes())
                    .expect("Failed to write to output file: {:?}")
            }
            CompileResult::Error => {
                println!("error");
            }
        }
    }
}
