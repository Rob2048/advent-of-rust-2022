use std::collections::HashMap;

use colored::*;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
struct Vec2 {
    x: i32,
    y: i32,
}

fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let line_iter = file_data.lines();

    let mut knots = [Vec2 { x: 0, y: 0 }; 10];

    let mut position_list: HashMap<Vec2, bool> = HashMap::new();
    position_list.insert(Vec2 { x: 0, y: 0 }, true);

    for (line_idx, line) in line_iter.enumerate() {
        println!("Line {}", line);

        let mut arg_iter = line.split(' ');

        let direction = arg_iter.next().unwrap().chars().nth(0).unwrap();
        let count: i32 = arg_iter.next().unwrap().parse().unwrap();

        println!("Direction: {} Count: {}", direction, count);

        for step in 0..count {
            match direction {
                'U' => knots[0].y += 1,
                'R' => knots[0].x += 1,
                'D' => knots[0].y -= 1,
                'L' => knots[0].x -= 1,
                _ => (),
            }

            let knot_count = knots.len();

            for index in 1..knot_count {
                let knot_head = knots[index - 1];
                let knot_tail = &mut knots[index];

                let delta_x = knot_head.x - knot_tail.x;
                let delta_y = knot_head.y - knot_tail.y;

                if delta_x.abs() > 1 || delta_y.abs() > 1 {
                    knot_tail.x += delta_x.signum();
                    knot_tail.y += delta_y.signum();

                    if index == knot_count - 1 {
                        position_list.insert(knot_tail.clone(), true);
                    }
                }
            }

            // println!(
            //     "Step {} Head: {},{} Tail: {},{}",
            //     step, head.x, head.y, tail.x, tail.y
            // );
        }

        // for iter_y in 0..26 {
        //     for iter_x in 0..26 {
        //         let x = iter_x - 11;
        //         let y = (26 - iter_y) - 6;

        //         let mut print_char = '.';

        //         for (idx, knot) in knots.iter().enumerate() {
        //             if knot.x == x && knot.y == y {
        //                 if print_char == '.' {
        //                     print_char = char::from_digit(idx as u32, 10).unwrap();
        //                 }
        //             }
        //         }

        //         print!("{print_char}");
        //     }
        //     println!();
        // }

        // if line_idx == 2 {
        //     break;
        // }
    }
    
    let total_positions = position_list.iter().count();
    println!("Total positions: {}", total_positions);
}
