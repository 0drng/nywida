use std::{io::{BufRead, BufReader, Write}, os::unix::fs::MetadataExt, process::{ChildStdout, Command, ExitStatus, Stdio}};

use crate::error::CommandError;

pub fn run_command(program: &str, args: Vec<String>) -> Result<ExitStatus, std::io::Error> {
    let mut command = Command::new(program);

    for arg in args {
        command.arg(arg);
    };

    let mut child = command.stdout(Stdio::piped())
    .spawn()
    .expect("Failed to spawn command");

    let stdout = child
        .stdout
        .take()
        .expect("Failed to get stdout from command");

    let mut bufread: BufReader<ChildStdout> = BufReader::new(stdout);
    let mut buf: String = String::new();

    while let Ok(n) = bufread.read_line(&mut buf) {
        if n > 0 {
            println!("{}", buf.trim());
            buf.clear();
        } else {
            break;
        }
    }

    return child.wait();

}

pub fn ask_continue() -> Result<(), CommandError> {
    print!("Do you want to continue? y/N: ");
    std::io::stdout().flush().unwrap();
    let mut val: String = String::new();
    std::io::stdin().read_line(&mut val).unwrap();
    let val = val.trim();
    if val.eq("yes") | val.eq("y") {
        return Ok(());
    }
    Err(CommandError::UserAbort(
        "error.userAbort".to_owned(),
        Vec::new(),
    ))
}

pub fn get_uid() -> u32 {
    return std::fs::metadata("/proc/self").map(|m| m.uid()).unwrap_or(1000)
}