use std::fs;
fn main() {
    run_p1(String::from("src/test.txt"));
    run_p1(String::from("src/real.txt"));
    run_p2(String::from("src/test.txt"));
    run_p2(String::from("src/real.txt"));
}

fn read_file(fname: &str) -> std::string::String {
    fs::read_to_string(fname).expect("Ooops!")
}

fn str_2_vec(s: &str) -> Vec<i32> {
    let numbers: Vec<i32> = s
        .split_whitespace()
        .map(|s| s.parse().expect("parse error"))
        .collect();
    numbers
}

fn run_p1(s: String) {
    let test_chr: String = read_file(&s);
    let numbers = str_2_vec(&test_chr);
    let (p1result, _, _) = part_1_calc(&numbers, Some(2020));
    println!("Part 1 result: {}", p1result);
}

fn run_p2(s: String) {
    let test_chr: String = read_file(&s);
    let numbers = str_2_vec(&test_chr);
    let (result, _, _, _) = part_2_calc(&numbers);
    println!("Part 2 result: {}", result);
}


fn part_1_calc(v: &[i32], val: Option<i32>) -> (i32, i32, i32) {
    let num = val.unwrap_or(2020);
    let mut test = 0;
    for n in v {
        test = num - n;
        if v.contains(&test) {
            return (n * test, *n, test);
        } else {
        }
    }
    (0, test, 0)
}


fn part_2_calc(v: &[i32]) -> (i32, i32, i32, i32) {
    let mut t1: i32 = 0;
    for (i,n) in v.iter().enumerate() {
        t1 = 2020 - n;
        let mut v_copy = v.to_vec();
        v_copy.remove(i);
        let (_, a, b) = part_1_calc(&v_copy, Some(t1));
        if a != 0 && b != 0 {
            return (n * a * b, t1, a, b)
        } else {
            t1=0;
        }
    }
 (0, t1, 0, 0)
}
