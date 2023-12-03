use std::fs;

const PATH: &str = "./data.txt";

fn main() {
    let content = fs::read_to_string(PATH).expect("Something went wrong reading the file");
    let lines = content.lines().collect::<Vec<&str>>();
    let mut total = 0;
    lines.iter().for_each(|line| {
      println!("{}", line);
    });

    println!("{}", total); //55108
}
