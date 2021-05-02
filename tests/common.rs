use std::process::Command;
use std::{fs::File, io::Write};


use rcheer_lib::CompileResult;

pub struct Test {
    pub input: &'static str,
    pub output: &'static str,
}

pub fn run_test(input: &'static str) -> i32 {

    let output = "test.s";
    match rcheer_lib::compile(input) {
        CompileResult::Program(asm) => {
            let mut file = File::create(output)
                .expect(format!("Unable to create output file: {}", output).as_str());
            file.write_all(asm.as_bytes())
                .expect("Failed to write to output file: {:?}")
        }   
        CompileResult::ParseError(p) => {
            println!("Error in parsing: {}", p.message)
        }
        CompileResult::ScanError(s) => {
            println!("Error in scanning: {}", s)
        }
    };
    Command::new("gcc").arg(output);
    let status = Command::new("./a.out").status();
    match status {
        Ok(s) => {s.code().unwrap()}
        Err(_) => {
            panic!("no status")
        }
    }
}