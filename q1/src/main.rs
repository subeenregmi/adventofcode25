use std::fs;

fn main() {
    println!("Hello, world!");

    let file_contents = fs::read_to_string("input.txt").unwrap();

    let lines = file_contents.split("\n");

    let mut dial_value = 50;
    let mut count = 0;
    let mut line_count = 0;

    for line in lines {
        line_count += 1;

        if line == "" {
            break;
        }

        let (d, a) = line.split_at(1);

        let direction = match d {
            "R" => 1,
            "L" => -1,
            _ => panic!("other than R and L"),
        };

        let amount = match i32::from_str_radix(a, 10) {
            Ok(a) => a,
            Err(e) => panic!("error turning string to i32, {e}"),
        };

        dial_value += direction * amount;

        dial_value %= 100;

        if dial_value == 0 {
            count += 1;
            dial_value = 0;
            continue;
        }

        if dial_value < 0 {
            dial_value += 100;
        }
    }

    println!("{count} {line_count}")
}
