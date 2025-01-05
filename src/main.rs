use blaze_downloader::{clear_terminal, handle_program_error};
use blaze_downloader::{get_config, run_program};

#[tokio::main]
async fn main() {
    clear_terminal();

    let args = match get_config() {
        Ok(args) => args,
        Err(e) => handle_program_error(e),
    };

    match run_program(args).await {
        Ok(msg) => println!("{msg}"),
        Err(e) => handle_program_error(e),
    }
}
