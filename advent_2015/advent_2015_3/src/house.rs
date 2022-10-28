#[derive(Debug, Copy, Clone)]
pub struct House {
    pub x: i32,
    pub y: i32,
    pub counter: i32,
    pub valid: bool
}

impl House {
    pub fn new(x: i32, y: i32, counter: i32) -> House {
        return House { x, y, counter, valid: true };
    }
    pub fn increment(&mut self) -> &mut House {
        self.counter += 1;
        return self;
    }
}