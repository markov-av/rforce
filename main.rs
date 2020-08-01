use std::io;
use std::num::ParseFloatError;


fn get_prob(mut n: i32, mut p: f64) -> f64 {
    let mut positive_prob = 1.0;
    loop {
        if n > 0 {
           if n % 2 == 1 {
               positive_prob = positive_prob * (1.0 - p)
                   + (1.0 - positive_prob) * p;
           }
            p = p * (1.0 - p) + (1.0 - p) * p;
            n /= 2;
        }
        else {
            break
        }
    }
    positive_prob
}

fn split_once(in_string: &str) -> (i32, f64) {
    let mut splitter = in_string.split(' ');
    let n = splitter.next().unwrap().parse::<i32>();
    let n = match n {
        Ok(n) => n,
        Err(error) => {
            panic!("There was a problem: {:?}", error)
        },
    };
    let p= splitter.next().unwrap().trim_end().parse::<f64>();
    let p = match p {
        Ok(p) => p,
        Err(error) => {
            panic!("There was a problem: {:?}", error)
        },
    };
    (n, p)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Err");
    let (n, p) = split_once(&input);
    let a = get_prob(n, p);
    println!("{}", a);
}
