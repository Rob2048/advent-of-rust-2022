use colored::*;

fn render_screen(cycle: i32, x: i32) {
    let col = cycle % 40;

    if col == 0 {
        print!("Cycle {:3} -> ", cycle + 1);
    }

    let x_pos = x;
    if col >= x_pos - 1 && col <= x_pos + 1 {
        print!("{}", "#".green());
    } else {
        print!(".");
    }

    if col == 39 {
        println!();
    }
}

fn main() {
    let mut cpu_x = 1;
    let mut cpu_cycle: i32 = 0;

    for line in std::fs::read_to_string("input.txt").unwrap().lines() {
        // println!("Line: {line}");

        // Fetch.
        let mut param_iter = line.split(' ');
        let opcode = param_iter.next().unwrap();

        if opcode == "noop" {
            // Render.
            render_screen(cpu_cycle, cpu_x);
            cpu_cycle += 1;
        } else if opcode == "addx" {
            // Decode.
            let param: i32 = param_iter.next().unwrap().parse().unwrap();

            // Render.
            render_screen(cpu_cycle, cpu_x);
            cpu_cycle += 1;

            // Render.
            render_screen(cpu_cycle, cpu_x);
            cpu_cycle += 1;

            // Apply.
            cpu_x += param;
        }
    }
}
