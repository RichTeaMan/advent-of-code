use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut max_calories_1 = 0;
    let mut max_calories_2 = 0;
    let mut max_calories_3 = 0;
    // File hosts must exist in current path before this produces output
    if let Ok(lines) = read_lines("./input.txt") {
        let mut calories = 0;
        // Consumes the iterator, returns an (Optional) String
        for line_opt in lines {
            if let Ok(line) = line_opt {
                let line_calories_opt = line.parse::<i32>();
                if let Ok(line_calories) = line_calories_opt {
                    calories += line_calories;
                } else if line.is_empty() {
                    
                    
                    if calories > max_calories_1 {
                        max_calories_3 = max_calories_2;
                        max_calories_2 = max_calories_1;
                        max_calories_1 = calories;
                    }
                    else if calories > max_calories_2 {
                        max_calories_3 = max_calories_2;
                        max_calories_2 = calories;
                    }
                    else if calories > max_calories_3 {
                        max_calories_3 = calories;
                    }



                    calories = 0;
                }
            }
        }
        println!("Max calories 1: {max_calories_1}.");
        println!("Max calories 2: {max_calories_2}.");
        println!("Max calories 3: {max_calories_3}.");
        println!("Answer : {ans}", ans = max_calories_1 + max_calories_2 + max_calories_3);
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
