use std::fs;

fn task1(commands: &[(&str, i32)]) -> (i32, i32) {
    let mut pos = (0, 0);
    for cmd in commands {
        match cmd.0 {
            "forward" => pos.0 += cmd.1,
            "down" => pos.1 += cmd.1,
            "up" => pos.1 -= cmd.1,
            _ => panic!("Error!"),
        }
    }

    pos
}

fn task2(commands: &[(&str, i32)]) -> (i32, i32) {
    let mut pos = (0, 0);
    let mut aim = 0;
    for cmd in commands {
        match cmd.0 {
            "forward" => { pos.0 += cmd.1; pos.1 += aim * cmd.1 },
            "down" => aim += cmd.1,
            "up" => aim -= cmd.1,
            _ => panic!("Error!"),
        }
    }

    pos
}

fn main() {
    let content = fs::read_to_string("input.txt").unwrap();

    let entries = content.lines();
    let commands: Vec<(&str, i32)> = entries
        .map(|e| {
            let mut cmd_arg = e.split(' ');
            let cmd = cmd_arg.next().unwrap();
            let arg = cmd_arg
                .next().unwrap()
                .parse().unwrap();
            (cmd, arg)
        })
        .collect();

    let result = task1(&commands);
    println!("Task 1: {:?}", result.0 * result.1);
    let result = task2(&commands);
    println!("Task 1: {:?}", result.0 * result.1);
}
