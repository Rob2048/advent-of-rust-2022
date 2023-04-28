use colored::*;

#[derive(Debug)]
struct Tree {
    height: i32,
    visible: bool,
    score: usize,
}

fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let line_itr = file_data.lines();

    let mut grid_width = 0;
    let mut grid_height = 0;
    let mut grid_data: Vec<Tree> = Vec::new();

    for line in line_itr {
        println!("Line {}", line);
        let line_bytes = line.as_bytes();
        grid_height += 1;

        if grid_width == 0 {
            grid_width = line_bytes.len();
        }

        for b in line.chars() {
            grid_data.push(Tree {
                height: b.to_digit(10).unwrap() as i32,
                visible: false,
                score: 0,
            });
        }
    }

    println!("Grid size: {},{}", grid_width, grid_height);

    let mut highest_scenic_score = 0;

    for iter_x in 0..grid_height {
        for iter_y in 0..grid_width {
            let tree = &grid_data[iter_y * grid_width + iter_x];

            // Check each direction for viewing distance (stop at equal or taller).
            let mut view_score = 0;
            for new_x in (iter_x + 1)..grid_width {
                let target = &grid_data[iter_y * grid_width + new_x];
                view_score += 1;

                if target.height >= tree.height {
                    break;
                }
            }
            let mut scenic_score = view_score;

            let mut view_score = 0;
            for new_x in (0..iter_x).rev() {
                let target = &grid_data[iter_y * grid_width + new_x];
                view_score += 1;

                if target.height >= tree.height {
                    break;
                }
            }
            scenic_score *= view_score;

            let mut view_score = 0;
            for new_y in (iter_y + 1)..grid_height {
                let target = &grid_data[new_y * grid_width + iter_x];
                view_score += 1;

                if target.height >= tree.height {
                    break;
                }
            }
            scenic_score *= view_score;

            let mut view_score = 0;
            for new_y in (0..iter_y).rev() {
                let target = &grid_data[new_y * grid_width + iter_x];
                view_score += 1;

                if target.height >= tree.height {
                    break;
                }
            }
            scenic_score *= view_score;

            highest_scenic_score = std::cmp::max(highest_scenic_score, scenic_score);

            let tree = &mut grid_data[iter_y * grid_width + iter_x];
            tree.score = scenic_score;

            // Multiply all viewing distances together.
        }
    }

    // Draw debug trees.
    for iter_y in 0..grid_height {
        for iter_x in 0..grid_width {
            let tree = &grid_data[iter_y * grid_width + iter_x];

            print!("{}({})", tree.height.to_string().green(), tree.score);
        }

        println!();
    }

    println!("Highest scenic score: {}", highest_scenic_score);
}
