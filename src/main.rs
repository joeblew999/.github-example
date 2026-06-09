//! Minimal Rust crate so the example's CI compiles + runs a real toolchain on
//! every OS (ubuntu/macos/windows) — not just nushell tasks. Rust comes from
//! rustup + rust-toolchain.toml (NEVER mise); `mise run ci` → `rust:test`.

fn greet() -> String {
    "hello, world".to_string()
}

fn main() {
    println!("{}", greet());
}

#[cfg(test)]
mod tests {
    use super::greet;

    #[test]
    fn greets() {
        assert_eq!(greet(), "hello, world");
    }
}
