mod command;
mod display;
mod lookup;

use std::process::ExitCode;

fn main() -> ExitCode {
    match command::run() {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => {
            log::error!("{e}");
            ExitCode::FAILURE
        }
    }
}
