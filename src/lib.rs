mod constants;
mod data;
mod utils;

use data::{Config, Flag, ProgramError};
use std::{env, process};
use utils::PEResult;
pub use utils::{clear_terminal, red_log, yellow_log};

pub fn get_config() -> PEResult<Config> {
    let mut raw_args = env::args();
    raw_args.next(); // pop off executable path
    Config::from(raw_args)
}

pub fn run_program(config: Config) -> PEResult<&'static str> {
    if Flag::handle_help_flag(&config).is_ok() {
        return Ok("END OF HELP SECTION");
    };

    Ok("DONE")
}

const ERROR_EXIT_CODE: i32 = 0; // Not an error exit code, I know. Using it so that terminal doesnt print extra text on-exit

pub fn handle_program_error(e: ProgramError) -> ! {
    red_log(format!("Error: {} \nExiting...", e.msg()).as_str());
    process::exit(ERROR_EXIT_CODE)
}
