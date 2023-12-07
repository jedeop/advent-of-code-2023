use crate::utils;

#[derive(PartialEq, Eq)]
enum Item {
    Blank,
    Number(u32),
    Symbol,
}

const LINE_LEN: usize = 140;

fn check_adjacent(list: &[Item], index: usize) -> bool {
    let left_exist = index % LINE_LEN > 0;
    let right_exist = index % LINE_LEN + 1 < LINE_LEN;
    let up_exist = index / LINE_LEN > 0;
    let down_exist = index / LINE_LEN + 1 < LINE_LEN;

    if left_exist && list[index - 1] == Item::Symbol {
        return true;
    }
    if right_exist && list[index + 1] == Item::Symbol {
        return true;
    }
    if up_exist && list[index - LINE_LEN] == Item::Symbol {
        return true;
    }
    if down_exist && list[index + LINE_LEN] == Item::Symbol {
        return true;
    }
    if left_exist && up_exist && list[index - LINE_LEN - 1] == Item::Symbol {
        return true;
    }
    if left_exist && down_exist && list[index + LINE_LEN - 1] == Item::Symbol {
        return true;
    }
    if right_exist && up_exist && list[index - LINE_LEN + 1] == Item::Symbol {
        return true;
    }
    if right_exist && down_exist && list[index + LINE_LEN + 1] == Item::Symbol {
        return true;
    }

    false
}

pub fn part1() -> u32 {
    let input = utils::read_input(3);
    let input = input
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '.' => Item::Blank,
            num if num.is_numeric() => Item::Number(num.to_digit(10).unwrap()),
            _ => Item::Symbol,
        })
        .collect::<Vec<Item>>();

    let mut result = 0;

    let mut temp_num = 0;
    let mut is_adjacent = false;

    for (i, item) in input.iter().enumerate() {
        if let Item::Number(num) = item {
            temp_num = temp_num * 10 + num;
            if check_adjacent(&input, i) {
                is_adjacent = true;
            }
        } else {
            if is_adjacent {
                result += temp_num;
            }
            temp_num = 0;
            is_adjacent = false;
        }
    }

    result
}

pub fn part2() -> u32 {
    todo!()
}
