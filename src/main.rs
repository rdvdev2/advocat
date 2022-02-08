use advocat::error;
use std::process;

fn main() -> ! {
    let exitcode = match advocat::run() {
        Ok(c) => c,
        Err(e) => {
            error!("{}", e);
            *e
        }
    };

    process::exit(exitcode);
}
