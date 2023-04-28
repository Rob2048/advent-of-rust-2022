fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let line_itr = file_data.lines();

    let mut mode = 0;

    let mut stacks: Vec<Vec<char>> = Vec::new();

    for _ in 0..10 {
        stacks.push(Vec::new());
    }

    for line in line_itr {
        println!("Line: {}", line);

        if line == "" {
            println!("{:?}", stacks);
            println!("Instructions mode");
            mode = 1;
            continue;
        }

        if mode == 0 {
            for (idx, c) in line.chars().enumerate() {
                if c.is_alphabetic() {
                    let col = idx / 4 + 1;
                    println!("{}: {} {}", idx, c, col);

                    stacks.get_mut(col).unwrap().insert(0, c);
                }
            }
        } else {
            let mut ins_itr = line.split(' ');

            ins_itr.next();
            let count: i32 = ins_itr.next().unwrap().parse().unwrap();
            ins_itr.next();
            let from: i32 = ins_itr.next().unwrap().parse().unwrap();
            ins_itr.next();
            let to: i32 = ins_itr.next().unwrap().parse().unwrap();

            println!("{} {} {}", count, from, to);

            let to_idx = stacks[to as usize].len();

            for _ in 0..count {
                let stack_top = stacks.get_mut(from as usize).unwrap().pop().unwrap();
                stacks
                    .get_mut(to as usize)
                    .unwrap()
                    .insert(to_idx, stack_top);
            }
        }
    }

    println!("{:?}", stacks);

    print!("Result: ");

    for col in stacks {
        if let Some(num) = col.last() {
            print!("{}", num);
        }
    }

    println!();
}
