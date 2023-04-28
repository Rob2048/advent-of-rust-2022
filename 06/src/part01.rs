fn main() {
    let signal_data = std::fs::read_to_string("input.txt").unwrap();
    let signal_bytes = signal_data.as_bytes();
    println!("{signal_data}");

    assert!(signal_data.len() >= 4);

    for i in 3..signal_data.len() {
        let b = signal_bytes[i];
        let c = char::from(b);

        let b0 = signal_bytes[i - 0];
        let b1 = signal_bytes[i - 1];
        let b2 = signal_bytes[i - 2];
        let b3 = signal_bytes[i - 3];

        // compare last 4 bytes.
        let found = b0 == b1 || b0 == b2 || b0 == b3 || b1 == b2 || b1 == b3 || b2 == b3;
        println!("{c} {b} {} {}", found, i);

        if !found {
            println!("Protocol start at {}", i + 1);
            break;
        }
    }
}
