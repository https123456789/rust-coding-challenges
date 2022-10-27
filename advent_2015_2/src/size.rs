#[derive(Debug)]
pub struct Size {
    w: i32,
    l: i32,
    h: i32
}

impl Size {
    pub fn from_str(s: &str) -> Size {
        // Split the str by 'x'
        let split = s.split('x');
        let items: Vec<&str> = split.collect();
        return Size {
            w: items[0].parse::<i32>().unwrap(),
            l: items[1].parse::<i32>().unwrap(),
            h: items[2].parse::<i32>().unwrap()
        };
    }
    pub fn get_surface_area(&self) -> i32 {
        return (2 * self.w * self.l) +
            (2 * self.w * self.h) +
            (2 * self.h * self.l);
    }
    pub fn get_extra(&self) -> i32 {
        // Determine the smallest 2 sides
        let mut dims: Vec<i32> = vec!(self.w, self.h, self.l);
        dims.sort();
        return dims[0] * dims[1];
    }
}