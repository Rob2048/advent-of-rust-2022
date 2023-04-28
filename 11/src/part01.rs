#[derive(Debug, Clone)]
enum InspectOperation {
    AddOperand,
    MultiplyOperand,
    AddOld,
    MultiplyOld,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: Vec<i32>,
    inspect_operation: InspectOperation,
    inspect_operand: i32,
    test_operand: i32,
    test_decision_a: i32,
    test_decision_b: i32,
    total_inspections: i32,
}

fn main() {
    let mut monkeys: Vec<Monkey> = Vec::new();

    let line_data = std::fs::read_to_string("input.txt").unwrap();
    let mut line_iter = line_data.lines();

    loop {
        if line_iter.next().is_none() {
            break;
        }

        let mut new_monkey = Monkey {
            items: Vec::new(),
            inspect_operation: InspectOperation::AddOperand,
            inspect_operand: 0,
            test_operand: 0,
            test_decision_a: 0,
            test_decision_b: 0,
            total_inspections: 0,
        };

        new_monkey.items = line_iter
            .next()
            .unwrap()
            .split("Starting items:")
            .nth(1)
            .unwrap()
            .split(',')
            .map(|x| x.trim().parse::<i32>().unwrap())
            .collect();

        let operation: Vec<&str> = line_iter
            .next()
            .unwrap()
            .split("Operation: new = old ")
            .nth(1)
            .unwrap()
            .split(' ')
            .collect();

        new_monkey.inspect_operation = if operation[1].trim() == "old" {
            if operation[0] == "*" {
                InspectOperation::MultiplyOld
            } else {
                InspectOperation::AddOld
            }
        } else {
            new_monkey.inspect_operand = operation[1].trim().parse().unwrap();
            if operation[0] == "*" {
                InspectOperation::MultiplyOperand
            } else {
                InspectOperation::AddOperand
            }
        };

        new_monkey.test_operand = line_iter
            .next()
            .unwrap()
            .split("Test: divisible by")
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        new_monkey.test_decision_a = line_iter
            .next()
            .unwrap()
            .split("If true: throw to monkey")
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        new_monkey.test_decision_b = line_iter
            .next()
            .unwrap()
            .split("If false: throw to monkey")
            .nth(1)
            .unwrap()
            .trim()
            .parse()
            .unwrap();

        println!("New monkey data {:#?}", new_monkey);

        monkeys.push(new_monkey);

        if line_iter.next().is_none() {
            break;
        }
    }

    for round in 0..20 {
        println!("Round {}", round + 1);

        for monkey_index in 0..monkeys.len() {
            let monkey = monkeys[monkey_index].clone();

            for item in &monkey.items {
                // println!("  Inspect {item}");

                let mut new_item_worry = match monkey.inspect_operation {
                    InspectOperation::AddOperand => item + monkey.inspect_operand,
                    InspectOperation::MultiplyOperand => item * monkey.inspect_operand,
                    InspectOperation::AddOld => item + item,
                    InspectOperation::MultiplyOld => item * item,
                };

                // println!("    Worry now {new_item_worry}");

                new_item_worry /= 3;

                // println!("    Bored {new_item_worry}");

                let throw_target = if new_item_worry % monkey.test_operand == 0 {
                    // println!("    Is divisible by {}", monkey.test_operand);
                    monkey.test_decision_a
                } else {
                    // println!("    Not divisible by {}", monkey.test_operand);
                    monkey.test_decision_b
                };

                // println!("    Thrown to {}", throw_target);
                monkeys[throw_target as usize].items.push(new_item_worry);
            }

            let monkey = &mut monkeys[monkey_index];
            monkey.total_inspections += monkey.items.len() as i32;
            monkey.items.clear();
        }

        println!();
        for (index, monkey) in monkeys.iter().enumerate() {
            println!("Monkey {}: {:?}", index, monkey.items);
        }
        println!();
    }

    for (index, monkey) in monkeys.iter().enumerate() {
        println!("Monkey {} inspected items {} times.", index, monkey.total_inspections);
    }
    println!();

    monkeys.sort_unstable_by(|a, b| a.total_inspections.cmp(&b.total_inspections));
    monkeys.reverse();

    let monkey_business = monkeys[0].total_inspections * monkeys[1].total_inspections;

    println!("Monkey business: {}", monkey_business);
    
}   
