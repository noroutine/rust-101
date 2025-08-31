pub fn get_type_name<T>(_: &T) -> &'static str {
    std::any::type_name::<T>()
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query_lowercase = query.to_lowercase();
    let mut results: Vec<&str> = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query_lowercase) {
            results.push(line);
        }
    }

    results
}

#[cfg(test)]
mod tests;
