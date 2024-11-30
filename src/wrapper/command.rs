use std::{
    io::{BufRead, BufReader, Write},
    os::unix::fs::MetadataExt,
    process::{ChildStdout, Command, ExitStatus, Stdio},
};

use crate::{error::CommandError, service::translation_service::{t, Labels}};

pub fn run_command(program: &str, args: Vec<String>) -> Result<ExitStatus, CommandError> {
    let mut command = Command::new(program);

    for arg in args {
        command.arg(arg);
    }

    let mut child = match command.stdout(Stdio::piped()).spawn() {
        Ok(child) => child,
        Err(_) => {
            return Err(CommandError::CommandFailed(
                Labels::Error_CommandFailed,
                None,
            ))
        }
    };

    let stdout = match child.stdout.take() {
        Some(stdout) => stdout,
        None => return Err(CommandError::UserAbort(Labels::Error_UserAbort, None)),
    };

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

    let child = match child.wait() {
        Ok(child) => child,
        Err(_) => {
            return Err(CommandError::CommandFailed(
                Labels::Error_CommandFailed,
                None,
            ))
        }
    };

    return Ok(child);
}

pub fn ask_continue() -> Result<(), CommandError> {
    print!("{}", t(Labels::Info_ConfirmContinue, None));
    std::io::stdout().flush().unwrap();
    let mut val: String = String::new();
    std::io::stdin().read_line(&mut val).unwrap();
    let val = val.trim();
    if val.eq("yes") | val.eq("y") {
        return Ok(());
    }
    Err(CommandError::UserAbort(Labels::Error_UserAbort, None))
}

pub fn get_uid() -> u32 {
    return std::fs::metadata("/proc/self")
        .map(|m| m.uid())
        .unwrap_or(1000);
}
