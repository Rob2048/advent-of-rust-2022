fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let lines_itr = file_data.lines();

    let mut max_calories = 0;
    let mut local_calories = 0;

    for line in lines_itr {
        println!("Line: {}", line);

        if line == "" {
            println!("New elf");
            local_calories = 0;
        } else {
            let line_value: i32 = line.parse().unwrap();
            local_calories += line_value;

            if local_calories > max_calories {
                max_calories = local_calories;
            }

            println!("Current calories: {}", local_calories);
        }
    }

    println!("Max calories: {}", max_calories);
}
