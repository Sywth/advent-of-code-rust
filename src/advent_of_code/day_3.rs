pub fn parse(input: &str) -> (i32, Vec<i32>) {
    let stride: i32 = input.lines().next().unwrap().chars().count() as i32;
    let mut voltages: Vec<i32> = Vec::new();

    for line in input.lines() {
        for char in line.chars() {
            voltages.push(char.to_digit(10).unwrap() as i32);
        }
    }

    return (stride, voltages);
}

pub fn logic_1(input: &str) -> i32 {
    let (stride, voltages) = parse(input);
    let mut res_voltage: i32 = 0;

    struct MaxVal {
        idx: usize,
        val: i32,
    }

    for bank in voltages.chunks(stride as usize) {
        let mut max_1 = None::<MaxVal>;
        for (i, &v) in bank[..bank.len() - 1].iter().enumerate() {
            max_1 = match max_1 {
                None => Some(MaxVal { idx: i, val: v }),
                Some(mv) => match v > mv.val {
                    true => Some(MaxVal { idx: i, val: v }),
                    false => Some(mv),
                },
            }
        }

        let mut max_2 = None::<MaxVal>;
        let new_start_idx = max_1.as_ref().unwrap().idx + 1;
        for (i, &v) in bank[new_start_idx..].iter().enumerate() {
            max_2 = match max_2 {
                None => Some(MaxVal { idx: i, val: v }),
                Some(mv) => match v > mv.val {
                    true => Some(MaxVal {
                        idx: i + new_start_idx,
                        val: v,
                    }),
                    false => Some(mv),
                },
            }
        }

        let net_voltage = max_1.unwrap().val * 10 + max_2.unwrap().val;
        res_voltage += net_voltage;
    }

    return res_voltage;
}

pub fn logic_2(input: &str) -> i64 {
    const NUM_LENGTH: usize = 12;

    let (stride, voltages) = parse(input);
    let mut res_voltage: i64 = 0;

    struct MaxVal {
        idx: usize,
        val: i32,
    }

    for bank in voltages.chunks(stride as usize) {
        let mut maxes: Vec<MaxVal> = Vec::new();
        for _ in 0..NUM_LENGTH {
            let mut max: Option<MaxVal> = None::<MaxVal>;
            let idx_start = match maxes.last() {
                None => 0,
                Some(mv) => mv.idx + 1,
            };
            let idx_end = bank.len() - (NUM_LENGTH - maxes.len());

            for (i, &v) in bank[idx_start..=idx_end].iter().enumerate() {
                let net_idx = i + idx_start;
                max = match max {
                    None => Some(MaxVal {
                        idx: net_idx,
                        val: v,
                    }),
                    Some(mv) => match v > mv.val {
                        true => Some(MaxVal {
                            idx: net_idx,
                            val: v,
                        }),
                        false => Some(mv),
                    },
                }
            }
            maxes.push(max.unwrap());
        }

        let mut net_voltage: i64 = 0;
        for (i, mv) in maxes.iter().enumerate() {
            let power = (NUM_LENGTH - i - 1) as u32;
            let voltage = mv.val as i64 * 10_i64.pow(power);
            net_voltage += voltage;
        }
        res_voltage += net_voltage;
    }

    return res_voltage;
}

pub fn main() {
    let content = crate::util::read_file_to_string("assets/day_3/input.txt");
    let sol_1 = logic_1(&content);
    let sol_2 = logic_2(&content);
    print!("{}\n", sol_1);
    print!("{}\n", sol_2);
}
