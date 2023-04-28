#[derive(Debug, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

#[derive(Debug, PartialEq)]
enum WinTarget {
    Lose,
    Draw,
    Win,
}

fn convert_player0(input: char) -> Option<Shape> {
    if input == 'A' {
        Some(Shape::Rock)
    } else if input == 'B' {
        Some(Shape::Paper)
    } else if input == 'C' {
        Some(Shape::Scissors)
    } else {
        None
    }
}

fn convert_player1(input: char) -> Option<WinTarget> {
    if input == 'X' {
        Some(WinTarget::Lose)
    } else if input == 'Y' {
        Some(WinTarget::Draw)
    } else if input == 'Z' {
        Some(WinTarget::Win)
    } else {
        None
    }
}

fn compare(p0: Shape, p1: Shape) -> i32 {
    let mut score = match p1 {
        Shape::Rock => 1,
        Shape::Paper => 2,
        Shape::Scissors => 3,
    };

    // Outcome
    score += if p0 == p1 {
        3
    } else if (p0 == Shape::Rock && p1 == Shape::Paper)
        || (p0 == Shape::Paper && p1 == Shape::Scissors)
        || (p0 == Shape::Scissors && p1 == Shape::Rock)
    {
        6
    } else {
        0
    };

    return score;
}

fn main() {
    let file_data = std::fs::read_to_string("input.txt").unwrap();
    let lines_itr = file_data.lines();

    let mut total_score = 0;

    for line in lines_itr {
        println!("{}", line);

        let mut token_itr = line.split(' ');

        let p0_str = token_itr.next().unwrap();
        let p1_str = token_itr.next().unwrap();

        println!("P0: {}, P1: {} ", p0_str, p1_str);

        let p0 = convert_player0(p0_str.chars().nth(0).unwrap()).unwrap();
        let win_target = convert_player1(p1_str.chars().nth(0).unwrap()).unwrap();

        println!("p0: {:?}, win target: {:?}", p0, win_target);

        let p1 = match p0 {
            Shape::Rock => match win_target {
                WinTarget::Lose => Shape::Scissors,
                WinTarget::Draw => Shape::Rock,
                WinTarget::Win => Shape::Paper,
            },
            Shape::Paper => match win_target {
                WinTarget::Lose => Shape::Rock,
                WinTarget::Draw => Shape::Paper,
                WinTarget::Win => Shape::Scissors,
            },
            Shape::Scissors => match win_target {
                WinTarget::Lose => Shape::Paper,
                WinTarget::Draw => Shape::Scissors,
                WinTarget::Win => Shape::Rock,
            },
        };

        let round_score = compare(p0, p1);
        println!("Round score: {}", round_score);

        total_score += round_score;
    }

    println!("Total score: {}", total_score);
}
