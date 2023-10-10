fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use std::process::Command;
    #[test]
    fn needs_sudo() {
        assert!(Command::new("apt-get")
            .args(["install", "hello"])
            .status()
            .expect("failed to run apt-get")
            .success())
    }
}
