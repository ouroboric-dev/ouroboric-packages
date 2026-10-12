//! The official crate of the Ouroboric project (https://ouroboric.dev).

const INFO: &str = "Ouroboric: open, self-improving software factories.\n\
Website: https://ouroboric.dev\n\
Source:  https://github.com/ouroboric-dev";

fn main() {
    println!("{INFO}");
}

#[cfg(test)]
mod tests {
    use super::INFO;

    #[test]
    fn info_names_the_project_and_website() {
        assert!(INFO.starts_with("Ouroboric: "));
        assert!(INFO.contains("https://ouroboric.dev"));
        assert_eq!(INFO.lines().count(), 3);
    }
}
