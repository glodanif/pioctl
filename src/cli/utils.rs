use std::io::Error;
use std::os::unix::process::ExitStatusExt;
use std::process::{Command, ExitStatus, Output};

pub fn run(command: &str, args: &[&str], dry_run: bool) -> Result<Output, Error> {
    if dry_run {
        println!("[DRY RUN] {} {}", command, args.join(" "));
        Ok(Output {
            status: ExitStatus::from_raw(0),
            stdout: Vec::new(),
            stderr: Vec::new(),
        })
    } else {
        Command::new(command).args(args).output()
    }
}
