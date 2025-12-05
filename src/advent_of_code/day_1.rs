pub fn logic_1(content: &str) -> i32 {
    let mut res = 0;
    let mut dial = 50;

    // go line by line
    for line in content.lines() {
        let mut it = line.chars();
        let dir = if it.next().unwrap() == 'L' { -1 } else { 1 };
        let mag: i32 = it.collect::<String>().parse().unwrap();
        dial += dir * mag;
        dial = dial.rem_euclid(100);

        if dial == 0 {
            res += 1;
        }
    }

    return res;
}

pub fn logic_2(content: &str) -> i32 {
    let mut res: i32 = 0;
    let mut dial: i32 = 50;

    // go line by line
    for line in content.lines() {
        let mut it = line.chars();
        let net_pos: i32;

        let char = it.next().unwrap();
        let mag: i32 = it.collect::<String>().parse().unwrap();

        match char {
            'L' => {
                net_pos = dial - mag;
                if dial != 0 && net_pos <= 0 {
                    res += 1;
                }
                res += net_pos.abs() / 100;
            }
            'R' => {
                net_pos = dial + mag;
                res += net_pos / 100;
            }
            _ => panic!("invalid direction"),
        }

        dial = net_pos.rem_euclid(100);
    }

    return res;
}

pub fn main() {
    let content = crate::util::read_file_to_string("assets/day_1/input.txt");
    let sol_1 = logic_1(&content);
    let sol_2 = logic_2(&content);
    print!("{}\n", sol_1);
    print!("{}\n", sol_2);
}
