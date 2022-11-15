use shared_lib::{get_args, get_contents_of_file};

mod house;
use house::House;

fn main() {
    println!("Advent of Code 2015\nDay 3 Part 2: Perfectly Spherical Houses in a Vacuum");

    let input = get_contents_of_file(get_args(2)[1].clone());

    let mut houses: Vec<House> = vec!();

    let mut index = 0;
    // Santa cords
    let mut sx = 0;
    let mut sy = 0;
    // RoboSanta cords
    let mut rsx = 0;
    let mut rsy = 0;
    let mut sorr = 0; // Santa (0) or RoboSanta (1)
    visit_house(&mut houses, &sx, &sy);
    visit_house(&mut houses, &rsx, &rsy);
    while index < input.len() {
        let c = &input[index..index + 1];
        if sorr == 1 {
            rsx += match c {
                "<" => -1,
                ">" => 1,
                _ => 0
            };
            rsy += match c {
                "^" => 1,
                "v" => -1,
                _ => 0
            };
            visit_house(&mut houses, &rsx, &rsy);
        } else {
            sx += match c {
                "<" => -1,
                ">" => 1,
                _ => 0
            };
            sy += match c {
                "^" => 1,
                "v" => -1,
                _ => 0
            };
            visit_house(&mut houses, &sx, &sy);
        }
        index += 1;
        if sorr == 0 {
            sorr = 1;
        } else {
            sorr = 0;
        }
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