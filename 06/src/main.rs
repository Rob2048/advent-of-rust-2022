fn main() {
    let signal_data = std::fs::read_to_string("input.txt").unwrap();
    let signal_bytes = signal_data.as_bytes();
    println!("{signal_data}");

    let preamble_length = 14;

    assert!(signal_data.len() >= preamble_length);

    let mut checklist: [i32; 26] = [0; 26];

    for i in 0..signal_data.len() {
        let b = signal_bytes[i];
        
        let char_index = b as i32 - 'a' as i32;
        // println!("Add {} {}", char_index, char::from(b));
        checklist[char_index as usize] += 1;

        if i >= preamble_length {
            // Remove last element from checklist.
            let char_index = signal_bytes[i - preamble_length] as i32 - 'a' as i32;
            // println!("Remove {} {}", char_index, char::from(signal_bytes[i - preamble_length]));
            checklist[char_index as usize] -= 1;
        }

        // println!("{:?}", checklist);

        // Verify checklist.
        if i >= preamble_length - 1 {
            // println!("Check");
            let mut found = true;
            for e in checklist {
                if e > 1 {
                    found = false;
                    break;
                }
            }

            if found {
                println!("Found message preamble at {}", i + 1);
                break;
            }
        }
    }
}
