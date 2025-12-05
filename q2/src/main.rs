use std::fs;

fn main() {
    let content = fs::read_to_string("src/input.txt").unwrap();

    let t = content.trim();

    let ranges = t.split(",");

    let mut invalid_total = 0;
    let mut invalid_total_two = 0;

    for range in ranges {
        let ids = range.split_once("-").unwrap();

        let lower_bound = u64::from_str_radix(ids.0, 10).unwrap();
        let upper_bound = u64::from_str_radix(ids.1, 10).unwrap();

        for id in lower_bound..=upper_bound {
            let mut id_string = id.to_string();

            for i in 1..=(id_string.len() / 2) {
                if id_string.len() % i != 0 {
                    continue;
                }

                let pattern = &id_string.as_str()[..i];

                for j in (i..id_string.len()).step_by(i) {
                    let check = &id_string.as_str()[j..j + i];
                    if check == pattern {
                        invalid_total += id;
                        break;
                    }
                }
            }

            if id_string.len() % 2 != 0 {
                continue;
            }

            let second_part = id_string.split_off(id_string.len() / 2);
            if second_part == id_string {
                invalid_total += id
            }
        }
    }

    println!("{invalid_total}")
}
