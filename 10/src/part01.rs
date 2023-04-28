use colored::*;

fn main() {
    let mut cpu_x = 1;
    let mut cpu_cycle: i32 = 0;

    let mut cpu_x_prev: i32 = 1;
    let mut cpu_x_apply_cycle: i32 = 0;

    let mut signal_strength: i32 = 0;
    let mut signal_target_cycle: i32 = 20;

    for line in std::fs::read_to_string("input.txt").unwrap().lines() {
        println!("Line: {line}");

        let mut param_iter = line.split(' ');
        let opcode = param_iter.next().unwrap();

        if opcode == "noop" {
            cpu_cycle += 1;
        } else if opcode == "addx" {
            let param: i32 = param_iter.next().unwrap().parse().unwrap();

            cpu_x_prev = cpu_x;
            cpu_x_apply_cycle = cpu_cycle + 3;

            cpu_cycle += 2;
            cpu_x += param;
        }

        println!("Cycle {cpu_cycle} X {cpu_x}");

        if cpu_cycle >= signal_target_cycle {
            let signal_add = if signal_target_cycle >= cpu_x_apply_cycle {
                cpu_x
            } else {
                cpu_x_prev
            };

            println!("Check cycle {}", signal_add.to_string().green());

            signal_strength += signal_target_cycle * signal_add;
            signal_target_cycle += 40;
        }
    }

    println!("Signal strenght: {}", signal_strength);
}
