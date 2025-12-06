pub fn parser(input: &str) -> Vec<(i64, i64)> {
    return input
        .trim_end()
        .split(',')
        .map(|range_str| {
            let mut values = range_str.split('-').map(|x| x.parse::<i64>().unwrap());
            return (values.next().unwrap(), values.next().unwrap());
        })
        .collect();
}

pub fn logic_1(input: &str) -> i64 {
    let pairs = parser(input);
    let mut res = 0;
    for pair in pairs {
        for id in pair.0..=pair.1 {
            let digits: u32 = ((id as f64).log10().floor() as u32) + 1;
            if digits % 2 != 0 {
                continue;
            }

            let divisor: i64 = (10 as i64).pow(digits / 2);
            let lhs: i64 = id / divisor;
            let rhs: i64 = id % divisor;

            if lhs == rhs {
                res += id;
            }
        }
    }
    return res;
}

fn get_divisors(num: u32) -> Vec<i32> {
    let mut divisors = Vec::new();
    let num_sqrt = (num as f64).sqrt() as u32;
    for d1 in 1..=num_sqrt {
        if num % d1 == 0 {
            divisors.push(d1 as i32);

            let d2 = num / d1;
            if d1 != d2 {
                divisors.push(d2 as i32);
            }
        }
    }
    return divisors;
}

pub fn logic_2(input: &str) -> i64 {
    let pairs = parser(input);
    let mut res = 0;
    for pair in pairs {
        for id in pair.0..=pair.1 {
            let id_str = id.to_string();
            let chunk_sizes = get_divisors(id_str.len() as u32);

            for size in chunk_sizes {
                let mut is_valid = false;
                let mut sample: Option<&[u8]> = None;
                for chunk in id_str.as_bytes().chunks(size as usize) {
                    if sample.is_none() {
                        sample = Some(chunk);
                        continue;
                    }

                    if sample.unwrap() != chunk {
                        is_valid = false;
                        break;
                    }

                    is_valid = true;
                }

                if is_valid {
                    res += id;
                    break;
                }
            }
        }
    }

    return res;
}

pub fn main() {
    let content = crate::util::read_file_to_string("assets/day_2/input.txt");
    let sol_1 = logic_1(&content);
    let sol_2 = logic_2(&content);
    print!("{}\n", sol_1);
    print!("{}\n", sol_2);
}
