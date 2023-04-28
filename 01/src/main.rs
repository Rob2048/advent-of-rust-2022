
fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let lines_itr = file_data.lines();

    let mut elves: Vec<i32> = Vec::new();

    for line in lines_itr {
        if line == "" {
            elves.push(0);
        } else {
            if elves.len() == 0 {
                elves.push(0)
            }

            let line_value: i32 = line.parse().expect("Line should have a valid integer value");
            
            let current_elf = elves.last_mut().unwrap();
            *current_elf += line_value;
        }
    }

    println!("Num elves: {}", elves.len());

    elves.sort();
    elves.reverse();

    for (idx, calories) in elves.iter().enumerate() {
        println!("Elf {}: {}", idx, calories);
    }

    let mut total_calories = 0;

    assert!(elves.len() >= 3);
    for elf in 0..3 {
        total_calories += elves[elf];
    }

    println!("Total calories for top elves: {}", total_calories);

}
