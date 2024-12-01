struct Entry {
    left: i32,
    right: i32,
}

// Part 1
fn part_one() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut entry = Entry { left: 0, right: 0 };
        let mut left = true;
        for word in line.split_whitespace() {
            if left {
                entry.left = word.parse().unwrap();
                left = false;
            } else {
                entry.right = word.parse().unwrap();
            }
        }
        left_list.push(entry.left);
        right_list.push(entry.right);
    }

    left_list.sort();
    right_list.sort();

    let mut total = 0;
    for i in 0..left_list.len() {
        let diff = (left_list[i] - right_list[i]).abs();
        total += diff;
    }

    println!("Part Two: {}", total);
}

// Part 2
fn part_two() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();

    for line in input.lines() {
        let mut entry = Entry { left: 0, right: 0 };
        let mut left = true;
        for word in line.split_whitespace() {
            if left {
                entry.left = word.parse().unwrap();
                left = false;
            } else {
                entry.right = word.parse().unwrap();
            }
        }
        left_list.push(entry.left);
        right_list.push(entry.right);
    }

    let mut total = 0;
    for left_value in left_list.iter() {
        let mut found = 0;
        for right_value in right_list.iter() {
            if left_value == right_value {
                found += 1;
            }
        }

        total += left_value * found;
    }

    println!("Part Two: {}", total);
}

fn main() {
    part_one();
    part_two();
}