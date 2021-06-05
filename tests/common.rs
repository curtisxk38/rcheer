use std::fmt::format;
use std::process::Command;
use std::{fs::File, fs::remove_file, io::Write};



use rcheer_lib::CompileResult;

pub struct Test {
    pub input: &'static str,
    pub output: &'static str,
}

pub enum TestResult {
    Execution(i32),
    TypeError,
    ScanError,
    ParseError,
}

pub fn run_test(input: &'static str) -> TestResult {

    let output = format!("test.s");
    match rcheer_lib::compile(input) {
        CompileResult::Program(asm) => {
            let mut file = File::create(output.as_str())
                .expect(format!("Unable to create output file: {}", output.as_str()).as_str());
            file.write_all(asm.as_bytes())
                .expect("Failed to write to output file: {:?}")
        }   
        CompileResult::ParseError(p) => {
            println!("Error in parsing: {}", p.message);
            return TestResult::ParseError;
        }
        CompileResult::ScanError(s) => {
            println!("Error in scanning: {}", s);
            return TestResult::ScanError;
        }
        CompileResult::TypeError(errors) => {
            println!("Error in type checking");
            return TestResult::TypeError;
        }
    };
    let gcc_status = Command::new("gcc")
        .arg(output)
        .status();
    match gcc_status {
        Ok(s) => {
            println!("GCC status: {}", s);
        },
        Err(e) => {
            panic!("GCC failed to compile: {}", e)
        }
    }
    let status = Command::new("./a.out").status();
    match status {
        Ok(s) => {
            println!("status {}", s.code().unwrap());
            TestResult::Execution(s.code().unwrap())}
        Err(e) => {
            panic!("running executable error: {}", e)
        }
    }
}