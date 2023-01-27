use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    _second(&contents);
}

fn _first(contents: &str) {
    let mut sum = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }
        let qq: Vec<_> = line.split(",").collect();

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        let closure = |q: usize, x: &mut i32, y: &mut i32| {
            let range: Vec<i32> = qq[q]
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            *x = range[0];
            *y = range[1];
        };
        closure(0, &mut a, &mut b);
        closure(1, &mut c, &mut d);

        if a <= c && b >= d {
            sum += 1;
        } else if a >= c && b <= d {
            sum += 1;
        }
    }

    println!("{}", sum);
}

fn _second(contents: &str) {
    let mut sum = 0;
    for line in contents.lines() {
        if line.len() == 0 {
            break;
        }
        let qq: Vec<_> = line.split(",").collect();

        let mut a = 0;
        let mut b = 0;
        let mut c = 0;
        let mut d = 0;

        let closure = |q: usize, x: &mut i32, y: &mut i32| {
            let range: Vec<i32> = qq[q]
                .split("-")
                .map(|s| s.parse::<i32>().unwrap())
                .collect();
            *x = range[0];
            *y = range[1];
        };
        closure(0, &mut a, &mut b);
        closure(1, &mut c, &mut d);

        if a <= d && b >= c {
            println!("{} {} {} {}", a, b, c, d);
            sum += 1;
        }
    }

    println!("{}", sum);
}
