fn main() {
	let file_data = std::fs::read_to_string("input.txt").unwrap();
	let line_itr = file_data.lines();

	let mut total_priority = 0;

	for line in line_itr {
		println!("Line: {}", line);
		assert!(line.len() % 2 == 0);

		let item_count = line.len() / 2;

		let comp0_str = line.get(..item_count).unwrap();
		let comp1_str = line.get(item_count..).unwrap();

		println!("{} :: {}", comp0_str, comp1_str);

		let mut shared_item_type: Option<char> = None;

		for comp0_char in comp0_str.chars() {
			for comp1_char in comp1_str.chars() {
				if comp0_char == comp1_char {
					println!("Found match: {}", comp0_char);
					shared_item_type = Some(comp0_char);
				}
			}
		}

		if let Some(shared_item) = shared_item_type {
			let value = if shared_item.is_lowercase() {
				shared_item as i32 - 'a' as i32 + 1
			} else {
				shared_item as i32 - 'A' as i32 + 27
			};

			println!("Shared item: {} ({})", shared_item, value);

			total_priority += value;

		} else {
			println!("No shared item");
		}
	}

	println!("Total priority: {}", total_priority);
}