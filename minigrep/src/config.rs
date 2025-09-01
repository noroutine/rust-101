use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
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

fn env_var_is_truthy(var_name: &str) -> bool {
    const TRUTHY: &[&str] = &["1", "true", "yes", "on", "enable", "enabled"];

    env::var(var_name)
        .map(|val| TRUTHY.contains(&val.to_lowercase().as_str()))
        .unwrap_or(false)
}
