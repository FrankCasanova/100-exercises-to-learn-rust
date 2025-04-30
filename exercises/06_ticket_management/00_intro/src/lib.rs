fn _intro() -> &'static str {
    // TODO: fix me 👇
    "I'm ready to build a ticket management system!"
}

#[cfg(test)]
mod tests {
    use crate::_intro;

    #[test]
    fn test_intro() {
        assert_eq!(_intro(), "I'm ready to build a ticket management system!");
    }
}
