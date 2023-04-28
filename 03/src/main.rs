fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let line_itr = file_data.lines();
    let lines: Vec<&str> = line_itr.collect();

    assert!(lines.len() % 3 == 0);
    println!("Lines: {}", lines.len());

	let mut total_priority = 0;

    for i in 0..lines.len() / 3 {
        println!("Group: {}", i);

        let l0 = lines[i * 3 + 0];
        let l1 = lines[i * 3 + 1];
        let l2 = lines[i * 3 + 2];

        let mut badge: Option<char> = None;

        'char_loop_0: for c0 in l0.chars() {
            'char_loop_1: for c1 in l1.chars() {
                if c1 == c0 {
                    for c2 in l2.chars() {
                        if c2 == c0 {
                            badge = Some(c0);
                            break 'char_loop_0;
                        }
                    }

                    break 'char_loop_1;
                }
            }
        }

        if let Some(b) = badge {
            println!("Found badge: {}", b);

            let value = if b.is_lowercase() {
                b as i32 - 'a' as i32 + 1
            } else {
                b as i32 - 'A' as i32 + 27
            };

			total_priority += value;
        } else {
            println!("NO BADGE");
            assert!(false);
        }
    }

    println!("Total priority: {}", total_priority);
}
