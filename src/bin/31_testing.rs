
// rust cargo comes with a simple testing framework
// tests are marked with the #[test] attribute
// 
// unit tests are usually placed in a tests module to conditionally compile them only when building tests
// 

fn first_word(text: &str) -> &str {
    match text.find(" ") {
        Some(index) => &text[..index],
        None => text,
    }
}

#[cfg(test)] // this is only active when you run cargo test
mod tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        assert_eq!(first_word(""), "");
    }
    
    #[test]
    fn test_single_word() {
        assert_eq!(first_word("hello"), "hello");
    }
    
    #[test]
    fn test_multiple_words() {
        assert_eq!(first_word("hello world"), "hello");
    }
}