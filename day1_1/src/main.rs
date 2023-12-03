use std::fs;

const PATH: &str = "./data.txt";

fn main() {
    let content = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let lines = content.lines().collect::<Vec<&str>>();
    let mut total = 0;
    lines.iter().for_each(|line| {
        let mut first = -1;
        let mut last = -1;
        let chars = line.trim().chars();

        chars.filter(|char| char.is_numeric()).for_each(|char| {
            let digit = (char.to_string()).parse::<i32>().unwrap();

            if first == -1 {
                first = digit;
            }
            last = digit;
        });

        total += first * 10 + last;
    });

    println!("{}", total); //55108
}
