use std::collections::HashMap;

use colored::*;

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let line_iter = file_data.lines();

    let mut head = Vec2 { x: 0, y: 0 };
    let mut tail = Vec2 { x: 0, y: 0 };

    let mut position_list: HashMap<Vec2, bool> = HashMap::new();
    position_list.insert(Vec2 { x: 0, y: 0 }, true);

    for line in line_iter {
        println!("Line {}", line);

        let mut arg_iter = line.split(' ');

        let direction = arg_iter.next().unwrap().chars().nth(0).unwrap();
        let count: i32 = arg_iter.next().unwrap().parse().unwrap();

        println!("Direction: {} Count: {}", direction, count);

        for step in 0..count {
            match direction {
                'U' => head.y += 1,
                'R' => head.x += 1,
                'D' => head.y -= 1,
                'L' => head.x -= 1,
                _ => (),
            }

            let delta_x = head.x - tail.x;
            let delta_y = head.y - tail.y;

            if delta_x.abs() > 1 || delta_y.abs() > 1 {
                tail.x += delta_x.signum();
                tail.y += delta_y.signum();

                position_list.insert(tail.clone(), true);
            }

            println!(
                "Step {} Head: {},{} Tail: {},{}",
                step, head.x, head.y, tail.x, tail.y
            );
        }
    }

    let total_positions = position_list.iter().count();
    println!("Total positions: {}", total_positions);
}
