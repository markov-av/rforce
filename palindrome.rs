use std::io;


fn split_string(s: &str) -> Vec<char> {
    let chars: Vec<_> = s.trim_end().chars().collect();
    chars
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Err");
    let r = split_string(&input);
    // println!("{}", r[r.len() - 1])
    for (i, c) in r.iter().enumerate() {
        println!("{} {}", i, c);
    }
}
