use shared_lib::{get_args, get_contents_of_file};

mod house;
use house::House;

fn main() {
    println!("Advent of Code 2015\nDay 3: Perfectly Spherical Houses in a Vacuum");

    let input = get_contents_of_file(get_args(2)[1].clone());

    let mut houses: Vec<House> = vec!();

    let mut index = 0;
    let mut x = 0;
    let mut y = 0;
    visit_house(&mut houses, &x, &y);
    while index < input.len() {
        let c = &input[index..index + 1];
        x += match c {
            "<" => -1,
            ">" => 1,
            _ => 0
        };
        y += match c {
            "^" => 1,
            "v" => -1,
            _ => 0
        };
        visit_house(&mut houses, &x, &y);
        index += 1;
    }
    println!("Total Number of Unique Houses: {}", houses.len());
    let mut i = 0;
    let mut max = 1;
    while i < houses.len() {
        let house: &mut House = &mut houses[i];
            if house.counter > max {
                max = house.counter;
            }
        i += 1;
    }
}

fn visit_house(houses: &mut Vec<House>, x: &i32, y: &i32) {
    // Check to see if a house exists at the specified coordinate
    let mut house_exists = false;
    let mut i = 0;
    while i < houses.len() {
        let house: &mut House = &mut houses[i];
        if house.x == *x && house.y == *y {
            house_exists = true;
            house.increment();
        }
        i += 1;
    }
    if house_exists {
        return;
    } else {
        let house = House::new(*x, *y, 1);
        houses.push(house.clone());
    }
}