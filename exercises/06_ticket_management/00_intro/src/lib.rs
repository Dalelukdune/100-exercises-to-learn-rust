fn intro() -> &'static str {
    "I'm ready to build a ticket management system!"
}

#[cfg(test)]
mod tests {
    use crate::intro;

    #[test]
    fn test_intro() {
        assert_eq!(intro(), "I'm ready to build a ticket management system!");
    }
}
