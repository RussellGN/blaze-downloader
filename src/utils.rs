use crate::data::ProgramError;
use colored::*;
use std::{env::consts, process::Command};

pub type PEResult<T = ()> = Result<T, ProgramError>;

pub fn run_child_cmd(cmd: &str) -> PEResult {
    if consts::OS == "linux" {
        let status = Command::new("sh").arg("-c").arg(cmd).status();
        if let Err(e) = status {
            return Err(ProgramError::new(format!("Error running `{cmd}`: {e}")));
        }
        Ok(())
    } else if consts::OS == "windows" {
        let output = Command::new("cmd").arg("/C").arg(cmd).status();
        if let Err(e) = output {
            return Err(ProgramError::new(format!("Error running `{cmd}`: {e}")));
        }
        Ok(())
    } else {
        Err(ProgramError::new(format!("OS not supported by CLI")))
    }
}

pub fn clear_terminal() {
    let _ = run_child_cmd("cls");
    let _ = run_child_cmd("clear");
}

pub fn red_log(s: &str) {
    println!("{}", s.red());
}

pub fn yellow_log(s: &str) {
    println!("{}", s.yellow());
}
