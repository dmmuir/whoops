use std::io::prelude::*;
use std::io::{self, Write};
use std::process::{Command, Stdio};

fn main() -> io::Result<()> {
    for line in io::stdin().lock().lines() {
        let mut line = line.unwrap_or(String::new());

        match command(line.as_bytes()) {
            Some(output) => io::stdout().write_all(&output).unwrap(),
            None => {
                line.push('\n'); // adding `\n` char 0x0A.
                io::stdout().write_all(line.as_bytes()).unwrap();
            }
        };
    }

    Ok(())
}

fn command(input: &[u8]) -> Option<Vec<u8>> {
    let (cmd, args) = get_cmd();
    let mut process = Command::new(cmd)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .ok()?;

    if let Some(child_stdin) = process.stdin.as_mut() {
        child_stdin.write_all(input).ok()?;
    }

    process
        .wait_with_output()
        .ok()
        .filter(|out| out.status.success())
        .map(|out| out.stdout)
}

fn get_cmd() -> (String, Vec<String>) {
    let cmd = std::env::args().nth(1).unwrap_or(String::new());
    let args = std::env::args().skip(2).collect();
    (cmd, args)
}
