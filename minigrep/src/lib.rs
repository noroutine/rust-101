//! A simple text searching library.
//!
//! This module provides functions to search for a query string within
//! contents, either case-sensitive or case-insensitive, and a utility
//! function to get the type name of a value.

/// Returns the name of the type of the given value.
///
/// # Type Parameters
///
/// * `T` - The type of the value.
///
/// # Arguments
///
/// * `_` - A reference to a value of type `T`.
///
/// # Examples
///
/// ```
/// let x = 5;
/// assert_eq!(minigrep::get_type_name(&x), "i32");
/// ```
pub fn get_type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

/// Searches for all lines in `contents` that contain the `query` string.
///
/// This search is case-sensitive.
///
/// # Arguments
///
/// * `query` - The string to search for.
/// * `contents` - The text contents to search within.
///
/// # Returns
///
/// A vector of string slices, each representing a line containing the query.
///
/// # Examples
///
/// ```
/// let contents = "Rust\nsafe\nfast\nproductive";
/// let result = minigrep::search("safe", contents);
/// assert_eq!(result, vec!["safe"]);
/// ```
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

/// Searches for all lines in `contents` that contain the `query` string,
/// ignoring case.
///
/// # Arguments
///
/// * `query` - The string to search for.
/// * `contents` - The text contents to search within.
///
/// # Returns
///
/// A vector of string slices, each representing a line containing the query,
/// case-insensitively.
///
/// # Examples
///
/// ```
/// let contents = "Rust\nSafe\nfast\nproductive";
/// let result = minigrep::search_case_insensitive("safe", contents);
/// assert_eq!(result, vec!["Safe"]);
/// ```
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query_lowercase))
        .collect()
}

#[cfg(test)]
#[path ="tests.rs"]
mod tests;
