use std::env;

#[derive(Debug)]
pub struct Config {
    pub query: String,
    pub file_path: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn new() -> Result<Config, &'static str> {
        let args: Vec<String> = env::args().collect();

        dbg!(&args);

        if args.len() < 3 {
            return Err("need at least two arguments");
        }

        let query = args[1].clone();
        let file_path = args[2].clone();
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
