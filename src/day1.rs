use crate::utils;

fn input() -> Vec<String> {
    let input = utils::read_input(1);
    input.lines().map(|l| l.to_string()).collect()
}

pub fn part1() -> u32 {
    let inputs = input();

    let result = inputs
        .iter()
        .map(|l| {
            let mut chars = l.chars().filter(|c| c.is_numeric());
            let first = chars.next().unwrap();
            let last = match chars.last() {
                Some(n) => n,
                None => first,
            };
            format!("{}{}", first, last).parse::<u32>().unwrap()
        })
        .sum();

    result
}

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_number(line: &str) -> u32 {
    for i in 0..line.len() {
        let subset = line.chars().skip(i).take(5).collect::<String>();
        for num in 1..=9 {
            if subset.starts_with(NUMBERS[num - 1]) {
                return num as u32;
            }
        }
        if let Some(num) = subset.chars().next().unwrap().to_digit(10) {
            return num;
        }
    }
    0
}
fn find_number_rev(line: &str) -> u32 {
    let len = line.len();
    for i in 1..=len {
        let subset = line.chars().skip(len - i).take(5).collect::<String>();
        for num in 1..=9 {
            if subset.starts_with(NUMBERS[num - 1]) {
                return num as u32;
            }
        }
        if let Some(num) = subset.chars().next().unwrap().to_digit(10) {
            return num;
        }
    }
    0
}

pub fn part2() -> u32 {
    let inputs = input();

    let mut result = 0;

    for line in inputs {
        let first = find_number(&line);
        let last = find_number_rev(&line);

        result += format!("{}{}", first, last).parse::<u32>().unwrap();
    }

    result
}
