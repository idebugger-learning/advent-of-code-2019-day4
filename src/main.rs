use itertools::Itertools;

fn main() {
    let left = 206938;
    let right = 679128;

    let mut counter = 0;
    for i in left..=right {
        if check_condition(i) {
            println!("Conditions matched for {}", i);
            counter += 1;
        }
    }

    println!("{} passwords matched conditions", counter);
}

fn check_condition(number: usize) -> bool {
    let str = number.to_string();

    check_adjacent_chars(&str) && check_non_decreasing(&str)
}

fn check_adjacent_chars(str: &String) -> bool {
    str.chars()
        .group_by(|c| *c)
        .into_iter()
        .any(|group| group.1.count() == 2)
}

fn check_non_decreasing(str: &String) -> bool {
    for (a, b) in str.chars().tuple_windows() {
        let a = a.to_digit(10).unwrap();
        let b = b.to_digit(10).unwrap();
        if b < a {
            return false;
        }
    }

    return true;
}
