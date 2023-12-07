use std::collections::{HashMap, HashSet};

use crate::utils;

const LINE_LEN: usize = 140;

#[derive(PartialEq, Eq)]
enum Item {
    Blank,
    Number(u32),
    Symbol,
    Gear(u32), // u32 =>  id
}
impl Item {
    fn is_symbol(&self) -> bool {
        matches!(self, Item::Symbol | Item::Gear(_))
    }
}

pub fn part1() -> u32 {
    let input = input();

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
    let input = input();

    let mut result = 0;

    let mut temp_num = 0;
    let mut temp_gears = Vec::new();

    let mut gears: HashMap<u32, HashSet<u32>> = HashMap::new();

    for (i, item) in input.iter().enumerate() {
        if let Item::Number(num) = item {
            temp_num = temp_num * 10 + num;
            temp_gears.append(&mut check_gear(&input, i))
        } else {
            for id in &temp_gears {
                let gear = gears.entry(*id).or_default();
                gear.insert(temp_num);
            }
            temp_num = 0;
            temp_gears.clear();
        }
    }

    for (_, nums) in gears {
        if nums.len() == 2 {
            let mut ratio = 1;
            for num in nums {
                ratio *= num;
            }
            result += ratio;
        }
    }

    result
}

fn input() -> Vec<Item> {
    let mut gear_id_incr = 0;

    utils::read_input(3)
        .lines()
        .flat_map(|l| l.chars())
        .map(|c| match c {
            '.' => Item::Blank,
            '*' => {
                gear_id_incr += 1;
                Item::Gear(gear_id_incr)
            }
            num if num.is_numeric() => Item::Number(num.to_digit(10).unwrap()),
            _ => Item::Symbol,
        })
        .collect::<Vec<Item>>()
}

fn check_adjacent(list: &[Item], index: usize) -> bool {
    let left_exist = index % LINE_LEN > 0;
    let right_exist = index % LINE_LEN + 1 < LINE_LEN;
    let up_exist = index / LINE_LEN > 0;
    let down_exist = index / LINE_LEN + 1 < LINE_LEN;

    if left_exist && list[index - 1].is_symbol() {
        return true;
    }
    if right_exist && list[index + 1].is_symbol() {
        return true;
    }
    if up_exist && list[index - LINE_LEN].is_symbol() {
        return true;
    }
    if down_exist && list[index + LINE_LEN].is_symbol() {
        return true;
    }
    if left_exist && up_exist && list[index - LINE_LEN - 1].is_symbol() {
        return true;
    }
    if left_exist && down_exist && list[index + LINE_LEN - 1].is_symbol() {
        return true;
    }
    if right_exist && up_exist && list[index - LINE_LEN + 1].is_symbol() {
        return true;
    }
    if right_exist && down_exist && list[index + LINE_LEN + 1].is_symbol() {
        return true;
    }

    false
}

fn check_gear(list: &[Item], index: usize) -> Vec<u32> {
    let left_exist = index % LINE_LEN > 0;
    let right_exist = index % LINE_LEN + 1 < LINE_LEN;
    let up_exist = index / LINE_LEN > 0;
    let down_exist = index / LINE_LEN + 1 < LINE_LEN;

    let mut result = Vec::new();

    if left_exist {
        if let Item::Gear(id) = list[index - 1] {
            result.push(id);
        }
    }
    if right_exist {
        if let Item::Gear(id) = list[index + 1] {
            result.push(id);
        }
    }
    if up_exist {
        if let Item::Gear(id) = list[index - LINE_LEN] {
            result.push(id);
        }
    }
    if down_exist {
        if let Item::Gear(id) = list[index + LINE_LEN] {
            result.push(id);
        }
    }
    if left_exist && up_exist {
        if let Item::Gear(id) = list[index - LINE_LEN - 1] {
            result.push(id);
        }
    }
    if left_exist && down_exist {
        if let Item::Gear(id) = list[index + LINE_LEN - 1] {
            result.push(id);
        }
    }
    if right_exist && up_exist {
        if let Item::Gear(id) = list[index - LINE_LEN + 1] {
            result.push(id);
        }
    }
    if right_exist && down_exist {
        if let Item::Gear(id) = list[index + LINE_LEN + 1] {
            result.push(id);
        }
    }

    result
}
