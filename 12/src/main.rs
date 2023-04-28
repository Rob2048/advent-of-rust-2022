fn main() {
    let mut heightmap: Vec<char> = Vec::new();
    let mut heightmap_width = 0;
    let mut heightmap_height = 0;

    let line_data = std::fs::read_to_string("sample.txt").unwrap();
    let line_iter = line_data.lines();

    for line in line_iter {
        println!("Line {}", line);

        let char_row: &mut Vec<char> = &mut line.chars().collect();
        heightmap.append(char_row);

        heightmap_width = line.as_bytes().len();
        heightmap_height += 1;
    }

    println!("Width: {} Height: {}", heightmap_width, heightmap_height);

    println!("{:?}", heightmap);
}
