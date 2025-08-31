use adder::add;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_works_unit() {
        assert_eq!(add(2, 2), 4);
    }
}