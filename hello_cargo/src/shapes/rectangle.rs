#[derive(Debug)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    pub fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }

    pub fn area(self: &Self) -> u32 {
        self.height * self.width
    }

    pub fn can_hold(self: &Self, other: &Self) -> bool {
        self.height >= other.height && self.width >= other.width
    }
}
