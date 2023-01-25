use std::fs;

fn main() {
    let path = "input.txt";
    let contents = fs::read_to_string(path).expect("Something went wrong reading the file");

    // println!("contents: {}", contents);

    let mut vec: Vec<i32> = Vec::new();

    for chunk in contents.split("\n\n") {
        // println!("chunk: {}", chunk);
        let mut sum = 0;
        for line in chunk.split("\n") {
            sum += line.parse::<i32>().unwrap_or(0);
        }
        vec.push(sum);
    }
    println!("vec: {:?}", vec);
    //sort vec
    vec.sort();
    vec.reverse();
    println!("vec: {:?}", vec);
    let mut total = 0;
    for i in 0..=2 {
        total += vec[i];
    }
    println!("total: {}", total);
}
