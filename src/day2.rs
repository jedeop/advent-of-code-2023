use crate::utils;

#[derive(Default)]
struct Set {
    red: u32,
    green: u32,
    blue: u32,
}

pub fn part1() -> u32 {
    let input = utils::read_input(2);
    input
        .lines()
        .filter_map(|l| {
            let mut l = l.split(": ");
            let id = l
                .next()
                .unwrap()
                .chars()
                .skip(5)
                .collect::<String>()
                .parse::<u32>()
                .unwrap();

            let max_set = l
                .next()
                .unwrap()
                .split("; ")
                .map(|s| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    for cube in s.split(", ") {
                        let mut cube = cube.split_whitespace();
                        let (n, color) = (cube.next().unwrap(), cube.next().unwrap());
                        let n = n.parse::<u32>().unwrap();
                        match color {
                            "red" => red += n,
                            "green" => green += n,
                            "blue" => blue += n,
                            _ => panic!("unknown color"),
                        }
                    }

                    Set { red, green, blue }
                })
                .fold(Set::default(), |acc, s| {
                    let red = acc.red.max(s.red);
                    let green = acc.green.max(s.green);
                    let blue = acc.blue.max(s.blue);
                    Set { red, green, blue }
                });

            if max_set.red <= 12 && max_set.green <= 13 && max_set.blue <= 14 {
                Some(id)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2() -> u32 {
    let input = utils::read_input(2);
    input
        .lines()
        .map(|l| {
            let mut l = l.split(": ");
            let max_set = l
                .nth(1)
                .unwrap()
                .split("; ")
                .map(|s| {
                    let mut red = 0;
                    let mut green = 0;
                    let mut blue = 0;

                    for cube in s.split(", ") {
                        let mut cube = cube.split_whitespace();
                        let (n, color) = (cube.next().unwrap(), cube.next().unwrap());
                        let n = n.parse::<u32>().unwrap();
                        match color {
                            "red" => red += n,
                            "green" => green += n,
                            "blue" => blue += n,
                            _ => panic!("unknown color"),
                        }
                    }

                    Set { red, green, blue }
                })
                .fold(Set::default(), |acc, s| {
                    let red = acc.red.max(s.red);
                    let green = acc.green.max(s.green);
                    let blue = acc.blue.max(s.blue);
                    Set { red, green, blue }
                });

            max_set.red * max_set.green * max_set.blue
        })
        .sum()
}
