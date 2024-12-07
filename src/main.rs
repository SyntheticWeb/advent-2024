use std::{
    collections::{HashMap, HashSet, VecDeque},
    fs, i32,
    path::PathBuf,
    usize, vec,
};

fn main() {
    println!("Welcome to Advent of Code 2024!");
    let (dec_1_1, dec_1_2) = advent_dec_1();
    let (dec_2_1, dec_2_2) = advent_dec_2();
    let (dec_3_1, dec_3_2) = advent_dec_3();
    let (dec_4_1, dec_4_2) = advent_dec_4();
    println!("December 1st - First Puzzle: {}", dec_1_1);
    println!("December 1st - Second Puzzle: {}", dec_1_2);
    println!("December 2nd - First Puzzle: {}", dec_2_1);
    println!("December 2nd - Second Puzzle: {}", dec_2_2);
    println!("December 3rd - First Puzzle: {}", dec_3_1);
    println!("December 3rd - Second Puzzle: {}", dec_3_2);
    println!("December 4th - First Puzzle: {}", dec_4_1);
    println!("December 4th - Second Puzzle: {}", dec_4_2);
}

fn advent_dec_1() -> (usize, usize) {
    let mut ans1: usize = 0;
    let mut ans2: usize = 0;

    let mut path = PathBuf::new();

    path.push("data/q1-data.txt".to_string());

    let mut input = fs::read_to_string(path).unwrap();

    input = str::replace(input.as_str(), "\n", " ");

    let mut split_input: Vec<&str> = input.split(' ').collect();

    let mut left_col: Vec<usize> = vec![];
    let mut right_col: Vec<usize> = vec![];

    let mut num_list: Vec<usize> = vec![];

    for i in 0..split_input.len() {
        let mut number: usize = 0;

        match split_input[i].parse::<usize>() {
            Ok(num) => {
                number = num;
            }
            Err(_) => continue,
        }

        num_list.push(number);
    }

    for i in 0..num_list.len() {
        if i % 2 == 0 {
            left_col.push(num_list[i]);
        } else {
            right_col.push(num_list[i]);
        }
    }

    left_col.sort();
    right_col.sort();

    let diff: Vec<usize> = left_col
        .clone()
        .into_iter()
        .zip(right_col.clone().into_iter())
        .map(|(x, y)| x.abs_diff(y))
        .collect();

    ans1 = diff.iter().sum();

    let left_sum = get_num_each_elem(left_col);
    let right_sum = get_num_each_elem(right_col);

    let mut sim_vector: Vec<usize> = vec![];

    for (key, val) in left_sum.into_iter() {
        if right_sum.contains_key(&key) {
            sim_vector.push(key * val * right_sum[&key])
        }
    }

    ans2 = sim_vector.iter().sum();

    return (ans1, ans2);
}

fn advent_dec_2() -> (usize, usize) {
    let mut ans1 = 0;
    let mut ans2 = 0;

    let mut path = PathBuf::new();

    path.push("data/d2-data.txt".to_string());

    let mut input = fs::read_to_string(path).unwrap();

    for line in input.lines() {
        let string_numbers = line.split_whitespace();

        let mut numbers: Vec<usize> = vec![];

        for (_i, val) in string_numbers.enumerate() {
            numbers.push(val.parse::<usize>().unwrap());
        }

        if check_safe(numbers.clone()) {
            ans1 = ans1 + 1;
            ans2 = ans2 + 1;
        } else {
            for (i, _val) in numbers.clone().iter().enumerate() {
                let mut new_nums = numbers.clone();
                new_nums.remove(i);
                if check_safe(new_nums) {
                    ans2 = ans2 + 1;
                    break;
                }
            }
        }
    }

    return (ans1, ans2);
}

fn advent_dec_3() -> (usize, usize) {
    let mut ans1 = 0;
    let mut ans2 = 0;

    let mut path = PathBuf::new();

    path.push("data/d3-data.txt");

    let mut input = fs::read_to_string(path).unwrap();
    let mut sum: usize = 0;
    let mut sum_enabled: usize = 0;
    let mut enabled = true;

    for (i, c) in input.char_indices() {
        if c == 'm' {
            if let Some(num) = extract_mul_result(input.get(i..i + 12).unwrap()) {
                sum = sum + num;
                if enabled {
                    sum_enabled = sum_enabled + num;
                }
            }
        } else if c == 'd' {
            if input.get(i..i + 5).unwrap() == "don't" {
                enabled = false;
            } else if input.get(i..i + 2).unwrap() == "do" {
                enabled = true;
            }
        }
    }

    ans1 = sum;
    ans2 = sum_enabled;

    return (ans1, ans2);
}

fn advent_dec_4() -> (usize, usize) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut path = PathBuf::new();

    path.push("data/d4-data.txt");

    let mut input = fs::read_to_string(path).unwrap();

    input = input.replace("X", "4");
    input = input.replace("M", "3");
    input = input.replace("A", "2");
    input = input.replace("S", "1");

    let mut grid: Vec<Vec<usize>> = Vec::new();
    let mut row_ind = 0;

    for line in input.lines() {
        let mut row: Vec<usize> = vec![];

        for c in line.chars() {
            row.push(c.to_digit(10).unwrap() as usize);
        }

        grid.push(row);
        row_ind += 1;
    }
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 4 {
                println!("y: {} x: {}", i + 1, j + 1);
                println!("Starting read!");
                let add = get_num_xmas(grid.clone(), j, i);
                ans1 += add;
                println!("Added: {}", add);
            }
        }
    }

    for i in 0..grid.len() - 3 {
        for j in 0..grid[0].len() - 3 {
            let mut nums: Vec<usize> = vec![];
            for y in 0..3 {
                for x in 0..3 {
                    nums.push(grid[i + y][j + x]);
                }
            }

            let mut sum = 0;

            for (i, num) in nums.into_iter().enumerate() {
                if i % 2 != 0 {
                    if i == 1 {
                        if num != 2 {}
                    } else {
                    }

                    sum += num;
                }
            }
        }
    }

    return (ans1, ans2);
}

struct Lab {
    floor: Vec<Vec<char>>,
    max_x: i32,
    max_y: i32,
    guard_pos: (i32, i32),
    guard_dir: Direction,
    guard_spaces: HashSet<(i32, i32)>,
}

enum Collision {
    Obstacle,
    Escape,
}

impl Lab {
    fn new(
        guard_pos: (i32, i32),
        guard_dir: Direction,
        xsize: i32,
        ysize: i32,
        floor: Vec<Vec<char>>,
    ) -> Lab {
        Lab {
            floor,
            max_x: xsize,
            max_y: ysize,
            guard_pos,
            guard_dir,
            guard_spaces: HashSet::new(),
        }
    }

    fn check_collision(&mut self) -> Collision {
        let mut dx = 0;
        let mut dy = 0;

        while self.is_in_bounds(self.guard_pos) {
            match self.guard_dir {
                Direction::Up => dy = -1,
                Direction::Down => dy = 1,
                Direction::Left => dx = -1,
                Direction::Right => dx = 1,
                _ => return Collision::Obstacle,
            }

            self.guard_spaces
                .insert((self.guard_pos.0, self.guard_pos.1));

            self.guard_pos.0 += dx;
            self.guard_pos.1 += dy;

            if self.floor[self.guard_pos.1 as usize][self.guard_pos.0 as usize] == '#' {
                self.guard_dir = self.guard_dir.rotate_cw();
                return Collision::Obstacle;
            }
        }

        return Collision::Escape;
    }

    fn is_in_bounds(&self, (x, y): (i32, i32)) -> bool {
        if x < 0 || x > self.max_x - 1 || y < 0 || y > self.max_y - 1 {
            return false;
        }

        return true;
    }
}

fn advent_dec_6() -> (usize, usize) {
    let mut ans1 = 0;
    let mut ans2 = 0;
    let mut path = PathBuf::new();

    path.push("data/d6-test.txt");

    let mut input = fs::read_to_string(path).unwrap();

    let mut grid: Vec<Vec<char>> = vec![];
    let mut guard_pos: (i32, i32) = (0, 0);
    let mut guard_dir: Direction = Direction::Up;

    for (line_num, line) in input.lines().enumerate() {
        let mut row: Vec<char> = vec![];

        for (i, c) in line.chars().enumerate() {
            if let pos = match c {
                'v' | '>' | '<' | '^' => {
                    let dir = match c {
                        'v' => Direction::Down,
                        '>' => Direction::Right,
                        '^' => Direction::Up,
                        '<' => Direction::Left,
                        _ => Direction::DownLeft,
                    };
                    (i, line_num)
                }
                _ => (0, 0),
            }

            row.push(c);
        }

        grid.push(row);
    }

    return (ans1, ans2);
}

struct XmaxPath {
    x: usize,
    y: usize,
    dir: Direction,
}

#[derive(Debug, Clone)]
enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Direction {
    fn rotate_cw(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => Direction::Up,
        }
    }

    fn rotate_cc(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            _ => Direction::Up,
        }
    }
}

enum XDir {
    Left,
    Right,
    None,
}

enum YDir {
    Up,
    Down,
    None,
}

fn get_num_xmas(grid: Vec<Vec<usize>>, x: usize, y: usize) -> usize {
    let y_bnd = grid.len();
    let x_bnd = grid[0].len();

    let mut queue: VecDeque<XmaxPath> = VecDeque::new();

    let mut num_xmas = 0;
    for (xp, yp, dir) in get_neighbors(x, y, x_bnd, y_bnd) {
        if grid[yp][xp] == 3 {
            println!("Found an M in {:?} direction!", dir.clone());
            let new_path = XmaxPath { x: xp, y: yp, dir };
            queue.push_front(new_path);
        }
    }

    while let Some(path) = queue.pop_front() {
        let dir = path.dir;
        let mut xmas: bool = true;
        println!("Evaluating direction {:?}", dir);

        let xdir: XDir;
        let ydir: YDir;

        match dir {
            Direction::UpLeft | Direction::UpRight | Direction::Up => {
                if path.y < 2 {
                    println!("Break y lower");
                    xmas = false;
                }

                ydir = YDir::Up;
            }
            Direction::Down | Direction::DownLeft | Direction::DownRight => {
                if y_bnd - path.y < 3 {
                    xmas = false;
                    println!("Break y upper");
                }
                ydir = YDir::Down;
            }
            _ => {
                ydir = YDir::None;
            }
        }

        match dir {
            Direction::Left | Direction::DownLeft | Direction::UpLeft => {
                if path.x < 2 {
                    println!("Break x lower");
                    xmas = false;
                }
                xdir = XDir::Left;
            }
            Direction::Right | Direction::DownRight | Direction::UpRight => {
                if x_bnd - path.x < 3 {
                    println!("Break x upper");
                    xmas = false;
                }
                xdir = XDir::Right;
            }
            _ => {
                xdir = XDir::None;
            }
        }

        if xmas != false {
            let mut xchecks: Vec<usize> = vec![];
            let mut ychecks: Vec<usize> = vec![];

            match xdir {
                XDir::Right => xchecks.extend(vec![path.x + 1, path.x + 2]),
                XDir::Left => xchecks.extend(vec![path.x - 1, path.x - 2]),
                XDir::None => xchecks.extend(vec![path.x, path.x]),
            }

            match ydir {
                YDir::Down => ychecks.extend(vec![path.y + 1, path.y + 2]),
                YDir::Up => ychecks.extend(vec![path.y - 1, path.y - 2]),
                YDir::None => ychecks.extend(vec![path.y, path.y]),
            }

            for (i, (xpath, ypath)) in xchecks.into_iter().zip(ychecks.into_iter()).enumerate() {
                println!("Check({},{}): {:?}", ypath, xpath, grid[ypath][xpath]);
                if i == 0 && grid[ypath][xpath] != 2 {
                    println!("Did not find A!");
                    xmas = false;
                    break;
                } else if i == 1 && grid[ypath][xpath] != 1 {
                    println!("Did not find S!");
                    xmas = false;
                    break;
                }

                xmas == true;
            }
        }

        if xmas == true {
            println!("Found xmas!");
            num_xmas += 1;
        }
    }

    return num_xmas;
}

fn get_neighbors(x: usize, y: usize, x_bnd: usize, y_bnd: usize) -> Vec<(usize, usize, Direction)> {
    let mut neighbors: Vec<(usize, usize, Direction)> = vec![];

    let range: Vec<i32> = vec![-1, 0, 1];

    let x32: i32 = x.try_into().unwrap();
    let y32: i32 = y.try_into().unwrap();

    let x_bnd32: i32 = x_bnd.try_into().unwrap();
    let y_bnd32: i32 = y_bnd.try_into().unwrap();

    for i in range.clone() {
        for j in range.clone() {
            if (x32 + j >= 0 && x32 + j < x_bnd32)
                && (y32 + i >= 0 && y32 + i < y_bnd32)
                && (j.abs() + i.abs() != 0)
            {
                let new_x = (x32 + j).try_into().unwrap();
                let new_y = (y32 + i).try_into().unwrap();

                let unit_vec = (j, i);

                let dir: Direction;

                match unit_vec {
                    (-1, -1) => dir = Direction::UpLeft,
                    (-1, 0) => dir = Direction::Left,
                    (-1, 1) => dir = Direction::DownLeft,
                    (1, -1) => dir = Direction::UpRight,
                    (1, 0) => dir = Direction::Right,
                    (1, 1) => dir = Direction::DownRight,
                    (0, 1) => dir = Direction::Down,
                    (0, -1) => dir = Direction::Up,
                    _ => dir = Direction::Up,
                }
                neighbors.push((new_x, new_y, dir));
            }
        }
    }

    return neighbors;
}

fn extract_mul_result(input: &str) -> Option<usize> {
    let mut result = Some(0);

    let (mul, nums) = input.split_at(3);

    if mul != "mul" {
        return None;
    }

    if !nums.starts_with('(') {
        return None;
    }

    let mut first_num = 0;
    let mut second_num = 0;

    let opt = nums.split_once(',');
    let mut first;
    let mut second;

    match opt {
        Some((str1, str2)) => {
            first = str1;
            second = str2;
        }
        None => {
            return None;
        }
    }

    if !first.starts_with('(') {
        return None;
    }

    match check_first_num(first) {
        Some(num) => first_num = num,
        None => {
            return None;
        }
    }

    match check_second_num(second) {
        Some(num) => second_num = num,
        None => {
            return None;
        }
    }

    result = Some(first_num * second_num);

    return result;
}

fn check_first_num(input: &str) -> Option<usize> {
    if input.len() > 4 {
        return None;
    }

    if !check_if_digits(input.get(1..).unwrap()) {
        return None;
    }

    let num = input.get(1..).unwrap().parse::<usize>().unwrap();

    return Some(num);
}

fn check_second_num(input: &str) -> Option<usize> {
    let opt = input.split_once(')');
    let mut first;
    let mut second;

    match opt {
        Some((str1, str2)) => {
            first = str1;
            second = str2;
        }
        None => {
            return None;
        }
    }

    if !check_if_digits(first) {
        return None;
    }

    let num = first.parse::<usize>().unwrap();
    return Some(num);
}

fn check_if_digits(input: &str) -> bool {
    for c in input.chars() {
        if !c.is_digit(10) {
            return false;
        }
    }

    return true;
}

fn check_safe(nums: Vec<usize>) -> bool {
    let mut safe = false;
    if nums.is_sorted_by(|a, b| (a < b)) {
        let mut prev_num = nums.first().unwrap().clone();
        let mut rate_check = true;
        for num in nums.into_iter() {
            if (num - prev_num) >= 4 {
                rate_check = false;
            }

            prev_num = num;
        }

        if rate_check {
            safe = true;
        }
    } else if nums.is_sorted_by(|a, b| (a > b)) {
        let mut prev_num = nums.first().unwrap().clone();
        let mut rate_check = true;
        for num in nums.into_iter() {
            if (prev_num - num) >= 4 {
                rate_check = false;
            }

            prev_num = num;
        }

        if rate_check {
            safe = true;
        }
    }

    return safe;
}

fn get_num_each_elem(vector: Vec<usize>) -> HashMap<usize, usize> {
    let mut map: HashMap<usize, usize> = HashMap::new();

    for num in vector.into_iter() {
        if map.contains_key(&num) {
            map.insert(num, map[&num] + 1);
        } else {
            map.insert(num, 1);
        }
    }

    return map;
}
