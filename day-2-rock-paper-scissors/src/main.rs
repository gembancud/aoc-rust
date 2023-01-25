use std::fs;

enum state {
    lose,
    win,
    draw,
}

fn main() {
    // read from input
    let path = String::from("input.txt");
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");
    let mut score = 0;
    for line in contents.lines() {
        let a = line.split(" ").collect::<Vec<&str>>();
        let enemyhand = match a[0] {
            "A" => 0,
            "B" => 1,
            "C" => 2,
            _ => panic!("Invalid input"),
        };
        let neededstate = match a[1] {
            "X" => state::lose,
            "Y" => {
                score += 3;
                state::draw
            }
            "Z" => {
                score += 6;
                state::win
            }
            _ => panic!("Invalid input"),
        };
        match (enemyhand, neededstate) {
            (0, state::lose) => score += 3,
            (0, state::draw) => score += 1,
            (0, state::win) => score += 2,
            (1, state::lose) => score += 1,
            (1, state::draw) => score += 2,
            (1, state::win) => score += 3,
            (2, state::lose) => score += 2,
            (2, state::draw) => score += 3,
            (2, state::win) => score += 1,
            _ => (),
        }
        // match (aa, bb) {
        //     (0, 0) => score += 3,
        //     (0, 1) => score += 6,
        //     (0, 2) => score += 0,
        //     (1, 0) => score += 0,
        //     (1, 1) => score += 3,
        //     (1, 2) => score += 6,
        //     (2, 0) => score += 6,
        //     (2, 1) => score += 0,
        //     (2, 2) => score += 3,
        //     _ => panic!("Invalid input"),
        // }
    }
    println!("{}", score);
}
