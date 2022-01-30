use std::process;

fn main() {
    let name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let authors = env!("CARGO_PKG_AUTHORS");

    println!("{} v{} by {}", name, version, authors);

    process::exit(advocat::run());
}
