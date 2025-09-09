//! This module handles parsing configuration from command-line arguments and environment variables.

use std::env;

/// Configuration for the minigrep application.
#[derive(Debug)]
pub struct Config {
    /// The query string to search for.
    pub query: String,
    /// The path to the file to search within.
    pub file_path: String,
    /// Whether to ignore case when searching.
    pub ignore_case: bool,
}

impl Config {
    /// Creates a new `Config` by parsing command-line arguments and environment variables.
    ///
    /// # Returns
    ///
    /// * `Ok(Config)` if the required arguments are provided.
    /// * `Err(&'static str)` if the query string or file path arguments are missing.
    ///
    /// # Example
    ///
    /// ```
    /// let config = Config::new().expect("Failed to parse config");
    /// println!("Searching for '{}' in file '{}'", config.query, config.file_path);
    /// ```
    pub fn new() -> Result<Config, &'static str> {
        let mut args = env::args();

        dbg!(&args);

        let _program_name = args.next(); // ignore program name
        let query = args.next().ok_or("Didn't get a query string")?;
        let file_path = args.next().ok_or("Didn't get a file path")?;
        let ignore_case = env_var_is_truthy("IGNORE_CASE");

        Ok(Config {
            query,
            file_path,
            ignore_case,
        })
    }
}

/// Checks if the environment variable with the given name is set to a truthy value.
///
/// # Arguments
///
/// * `var_name` - The name of the environment variable to check.
///
/// # Returns
///
/// * `true` if the environment variable is set to one of the following values (case insensitive):
///   `"1"`, `"true"`, `"yes"`, `"on"`, `"enable"`, `"enabled"`.
/// * `false` otherwise or if the environment variable is not set.
fn env_var_is_truthy(var_name: &str) -> bool {
    const TRUTHY: &[&str] = &["1", "true", "yes", "on", "enable", "enabled"];

    env::var(var_name)
        .map(|val| TRUTHY.contains(&val.to_lowercase().as_str()))
        .unwrap_or(false)
}
