#[derive(Debug)]
struct Work {
    start: i32,
    end: i32,
}

fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let line_itr = file_data.lines();

    let mut total_overlaps = 0;

    for line in line_itr {
        println!("Line: {}", line);

        let mut num_iter = line.split([',', '-']);

        let elf0 = Work {
            start: num_iter.next().unwrap().parse().unwrap(),
            end: num_iter.next().unwrap().parse().unwrap(),
        };

        let elf1 = Work {
            start: num_iter.next().unwrap().parse().unwrap(),
            end: num_iter.next().unwrap().parse().unwrap(),
        };

        println!("Elf0: {:?} Elf1 {:?}", elf0, elf1);

        if elf0.start <= elf1.start && elf0.end >= elf1.end
            || elf1.start <= elf0.start && elf1.end >= elf0.end
        {
            total_overlaps += 1;
        }
    }

    println!("Total overlaps: {}", total_overlaps);
}
