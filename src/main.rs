use std::io::{self, Write};

const GREETING: &str = "Hello from Rust in Docker! 🦀";

fn greeting() -> &'static str {
    GREETING
}

fn main() {
    println!("{}", greeting());
    io::stdout().flush().expect("failed to flush stdout");
    std::thread::sleep(std::time::Duration::from_millis(200));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn greeting_matches_expected_text() {
        assert_eq!(greeting(), "Hello from Rust in Docker! 🦀");
    }

    #[test]
    fn greeting_mentions_rust_and_docker() {
        let message = greeting();
        assert!(message.contains("Rust"));
        assert!(message.contains("Docker"));
    }

    #[test]
    fn greeting_is_not_empty() {
        assert!(!greeting().trim().is_empty());
    }
}
