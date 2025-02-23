fn main() {
    println!(
        "{} {} {}",
        std::env::consts::OS,
        std::env::consts::ARCH,
        is_musl::is_musl()
    );
}
