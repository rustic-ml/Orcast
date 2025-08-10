/// Returns a greeting for the input name
///
/// # Examples
///
/// ```
/// let result = orcast::greet("World");
/// assert_eq!(result, "Hello, World!");
/// ```
pub fn greet(name: &str) -> String {
    format!("Hello, {name}!")
}

pub mod config;
pub mod alpaca;
pub mod error;
pub mod http;
pub mod trading;
pub mod market_data;
pub mod streaming;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_greet() {
        assert_eq!(greet("World"), "Hello, World!");
    }
} 