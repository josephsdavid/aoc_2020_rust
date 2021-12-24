use counter::Counter;
use std::fs;
fn main() {
    run_p1();
    run_p2();
}

fn run_p1() {
    println!("Running p1 test!");
    let test = read_file("src/data.txt");
    let test_vec: Vec<&str> = test.split("\n").collect();
    let test_len: i32 = test_vec.len().try_into().unwrap();

    let mut count: i32 = 0;
    for t in test.split("\n") {
        let vec: Vec<&str>= t.split(":").collect();
        if vec.len() > 1 {
            let lhs: Vec <&str> = vec[0].split(" ").collect();
            let bounds: Vec <usize> = lhs[0].split("-")
            .map(|s| s.parse().expect("parse error"))
            .collect();
            let cc = vec[1].chars().collect::<Counter<_>>();
            let key  = lhs[1].chars().next().unwrap();
            if cc[&key] < bounds[0] || cc[&key] > bounds[1] {
                count += 1
            }

        }
    }
    println!("Number of valid passwords: {}", test_len - count - 1)
}

fn run_p2() {
    let test = read_file("src/data.txt");
    let test_vec: Vec<&str> = test.split("\n").collect();
    let test_len: i32 = test_vec.len().try_into().unwrap();
    let mut count: i32 = 0;
    for t in test.split("\n") {
        let mut rowcount: i32 = 0;
        let vec: Vec<&str>= t.split(":").collect();
        if vec.len() > 1 {
            let lhs: Vec <&str> = vec[0].split(" ").collect();
            let bounds: Vec <usize> = lhs[0].split("-")
            .map(|s| s.parse().expect("parse error"))
            .collect();
            let key  = lhs[1].chars().next().unwrap();
            let rhs_no_ws: String = vec[1].to_owned().split_whitespace().collect();
            let rhs: Vec<char> = rhs_no_ws.chars().collect();
            for b in bounds{
                if rhs.len() > b-1{  
                    if rhs[b-1] == key {
                        rowcount += 1;
                    }
                }    else {
                    println!("ok!");
                    rowcount += 1;
                }
            }
        }
        if rowcount != 1 {
            count += 1
        }
    }
    println!("Running p2 test!");
    println!("Number of valid passwords: {}", test_len - count)
}


fn read_file(fname: &str) -> std::string::String {
    fs::read_to_string(fname).expect("Ooops!")
}


