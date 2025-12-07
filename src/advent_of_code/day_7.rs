#[derive(Debug, Clone, Copy)]
enum Node {
    Spawn,
    Vacuous,
    Split,
    Ray,
}

fn char_to_mi(c: &char) -> Node {
    return match c {
        'S' => Node::Spawn,
        '.' => Node::Vacuous,
        '^' => Node::Split,
        '|' => Node::Ray,
        _ => panic!("Unknown character in input"),
    };
}

fn mi_to_char(mi: &Node) -> char {
    return match mi {
        Node::Spawn => 'S',
        Node::Vacuous => '.',
        Node::Split => '^',
        Node::Ray => '|',
    };
}

fn debug_print_grid(grid: &Vec<Vec<Node>>) {
    for row in grid {
        let line: String = row.into_iter().map(mi_to_char).collect();
        println!("{}", line);
    }
}

fn parse(content: &str) -> Vec<Vec<Node>> {
    let len_row = content.lines().count();
    let len_col = content.lines().next().unwrap().chars().count();

    let mut grid: Vec<Vec<Node>> = vec![vec![Node::Vacuous; len_col]; len_row];
    for (y, row) in content.lines().enumerate() {
        for (x, char) in row.chars().enumerate() {
            grid[y][x] = char_to_mi(&char);
        }
    }

    return grid;
}

pub fn logic_1(content: &str) -> i32 {
    let mut grid: Vec<Vec<Node>> = parse(content);

    let mut num_splits = 0;
    let len_x: usize = grid[0].len();
    let len_y: usize = grid.len();

    for y in 0..len_y {
        for x in 0..len_x {
            let mi = grid[y][x];
            let has_bottom = y + 1 < len_y;
            let has_left = x > 0;
            let has_right = x + 1 < len_x;
            match mi {
                Node::Spawn => {
                    if has_bottom {
                        grid[y + 1][x] = Node::Ray;
                    }
                }
                Node::Ray => {
                    if !has_bottom {
                        continue;
                    }

                    let char_below = grid[y + 1][x];
                    match char_below {
                        Node::Vacuous => {
                            grid[y + 1][x] = Node::Ray;
                        }
                        Node::Split => {
                            let mut did_split = false;
                            if has_left {
                                grid[y + 1][x - 1] = Node::Ray;
                                did_split = true;
                            }
                            if has_right {
                                grid[y + 1][x + 1] = Node::Ray;
                                did_split = true;
                            }

                            if did_split {
                                num_splits += 1;
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    return num_splits;
}

pub fn logic_2(content: &str) -> i32 {
    let mut grid: Vec<Vec<Node>> = parse(content);

    let mut num_worlds = 1;
    let len_x: usize = grid[0].len();
    let len_y: usize = grid.len();

    for y in 0..len_y {
        for x in 0..len_x {
            let mi = grid[y][x];
            let has_bottom = y + 1 < len_y;
            let has_left = x > 0;
            let has_right = x + 1 < len_x;
            match mi {
                Node::Spawn => {
                    if has_bottom {
                        grid[y + 1][x] = Node::Ray;
                    }
                }
                Node::Ray => {
                    if !has_bottom {
                        continue;
                    }

                    let char_below = grid[y + 1][x];
                    match char_below {
                        Node::Vacuous => {
                            grid[y + 1][x] = Node::Ray;
                        }
                        Node::Split => {
                            if has_left {
                                grid[y + 1][x - 1] = Node::Ray;
                                num_worlds += 1;
                            }
                            if has_right {
                                grid[y + 1][x + 1] = Node::Ray;
                                num_worlds += 1;
                            }
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    return num_worlds;
}

pub fn main() {
    let content = crate::util::read_file_to_string("assets/day_7/sample.txt");
    let sol_1 = logic_1(&content);
    // TODO: Fix logic 2; its broken
    let sol_2 = logic_2(&content);
    print!("{}\n", sol_1);
    print!("{}\n", sol_2);
}
