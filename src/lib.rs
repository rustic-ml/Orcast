/// Returns a greeting for the input name
///
/// # Examples
///
/// ```
/// let result = orcast::greet("World");
/// assert_eq!(result, "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }
} 