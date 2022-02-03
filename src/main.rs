use std::process;

fn main() {
    let config = advocat::parse_args();
    process::exit(advocat::run(config));
}
