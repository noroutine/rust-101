#![allow(dead_code)]
#![allow(unused_imports)]

mod shapes;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
// #[cfg(all(test, feature = "db-tests"))]
mod tests;