use colored::*;

#[derive(Debug)]
struct Tree {
    height: i32,
    visible: bool,
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
            });
        }
    }

    println!("Grid size: {},{}", grid_width, grid_height);

    // +Y.
    for iter_x in 0..grid_width {
        let mut max_height: i32 = -1;

        for iter_y in 0..grid_height {
            let tree = &mut grid_data[iter_y * grid_width + iter_x];

            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }

    // -Y.
    for iter_x in 0..grid_width {
        let mut max_height: i32 = -1;

        for iter_y in (0..grid_height).rev() {
            let tree = &mut grid_data[iter_y * grid_width + iter_x];

            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }

    // +X.
    for iter_y in 0..grid_height {
        let mut max_height: i32 = -1;

        for iter_x in 0..grid_width {
            let tree = &mut grid_data[iter_y * grid_width + iter_x];

            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }

    // -X.
    for iter_y in 0..grid_height {
        let mut max_height: i32 = -1;

        for iter_x in (0..grid_width).rev() {
            let tree = &mut grid_data[iter_y * grid_width + iter_x];

            if tree.height > max_height {
                tree.visible = true;
                max_height = tree.height;
            }
        }
    }

    // Draw debug trees.
    for iter_y in 0..grid_height {
        for iter_x in 0..grid_width {
            let tree = &grid_data[iter_y * grid_width + iter_x];

            if tree.visible {
                print!("{}", tree.height.to_string().green());
            } else {
                print!("{}", tree.height.to_string().red());
            }
        }

        println!();
    }

    let total_visble_trees: i32 = grid_data
        .iter()
        .fold(0, |acc, tree| acc + if tree.visible { 1 } else { 0 });

    println!("Total visible trees: {}", total_visble_trees);
}
