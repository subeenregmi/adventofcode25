use std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();

    let banks = content.lines();

    let mut joltage = 0;

    for bank in banks {
        let jolts = bank.chars().collect::<Vec<char>>();

        let mut left = jolts[0];
        let mut right = jolts[1];

        for jolt_i in 1..jolts.len() {
            let jolt = jolts[jolt_i];

            if jolt > left && (jolt_i != jolts.len() - 1) {
                left = jolt;
                right = jolts[jolt_i + 1];
                continue;
            }

            if jolt > right {
                right = jolt;
                continue;
            }
        }

        let max_jolt = (left.to_digit(10).unwrap() * 10) + right.to_digit(10).unwrap();
        joltage += max_jolt;
    }

    println!("{joltage}")
}
