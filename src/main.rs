fn main() {
    if let Err(e) = qp::run() {
        eprintln!("error: {:#}", e);
        std::process::exit(1);
    }
}
