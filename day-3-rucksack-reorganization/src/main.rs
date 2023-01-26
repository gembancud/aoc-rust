mod mapper;

use mapper::mapper;
use std::collections::HashSet;

use std::fs;
fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    _second(&contents);
}

fn _first(contents: &str) {
    let mut sum = 0;

    for line in contents.lines() {
        let mut firsthalf = HashSet::new();
        for c in line[0..line.len() / 2].chars() {
            firsthalf.insert(mapper(&c));
        }
        let mut secondhalf = HashSet::new();
        for c in line[line.len() / 2..].chars() {
            secondhalf.insert(mapper(&c));
        }
        for i in firsthalf.intersection(&secondhalf) {
            println!("adding {}", i);
            sum += i;
        }
    }
    println!("{}", sum);
}

fn _second(contents: &str) {
    let mut sum = 0;

    let mut counter = 0;

    let mut first_group = HashSet::new();
    let mut second_group = HashSet::new();
    let mut third_group = HashSet::new();
    for line in contents.lines() {
        match counter {
            0 => {
                for c in line.chars() {
                    first_group.insert(mapper(&c));
                }
                counter += 1;
            }
            1 => {
                for c in line.chars() {
                    second_group.insert(mapper(&c));
                }
                counter += 1;
            }
            2 => {
                for c in line.chars() {
                    third_group.insert(mapper(&c));
                }
                counter = 0;

                // get the intersection of the three groups
                let mut intersection = first_group
                    .intersection(&second_group)
                    .cloned()
                    .collect::<HashSet<i32>>();
                intersection = intersection
                    .intersection(&third_group)
                    .cloned()
                    .collect::<HashSet<_>>();

                for i in intersection {
                    println!("adding {}", i);
                    sum += i;
                }

                first_group.clear();
                second_group.clear();
                third_group.clear();
            }
            _ => {}
        }
    }
    println!("{}", sum);
}
