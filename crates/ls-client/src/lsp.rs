use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};

use crate::json_rpc;

pub mod capabilities;
pub mod messages;


pub struct LanguageServer {
    program: Child,
    stdin: ChildStdin,
    stdout: ChildStdout,
}

impl LanguageServer {
    pub fn new() -> LanguageServer {
        let mut program = Command::new("./rust-ana/rust-analyzer")
            // .args(["/C", "echo hello"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("msg");
        let (stdin, stdout) = (
            program.stdin.take().unwrap(),
            program.stdout.take().unwrap()
        );
        LanguageServer {
            program,
            stdin, stdout
        }
    }
    pub fn initialize(init_params: messages::InitializeParams) {
    
    }
}