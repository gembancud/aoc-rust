use std::fs;

fn main() {
    // read from input
    let path = String::from("input.txt");
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let mut score = 0;
    for line in contents.lines() {
        let a = line.split(" ").collect::<Vec<&str>>();
        let aa = match a[0] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!("Invalid input"),
        };
        let bb = match a[1] {
            "X" => {
                score += 1;
                0
            }
            "Y" => {
                score += 2;
                1
            }
            "Z" => {
                score += 3;
                2
            }
            _ => panic!("Invalid input"),
        };
        match (aa, bb) {
            (0, 0) => score += 3,
            (0, 1) => score += 6,
            (0, 2) => score += 0,
            (1, 0) => score += 0,
            (1, 1) => score += 3,
            (1, 2) => score += 6,
            (2, 0) => score += 6,
            (2, 1) => score += 0,
            (2, 2) => score += 3,
            _ => panic!("Invalid input"),
        }
    }
    println!("{}", score);
}
