use std::io;

trait PolynomialExt {
    fn is_polynomial(&self) -> bool;
    fn _split_string(s: &str) -> Vec<char>;
    fn _punctuation_match(word: &str) -> bool;
}

impl PolynomialExt for String {
    fn is_polynomial(&self) -> bool {
        let result_vec: Vec<_> = Self::_split_string(&self);
        let vec_len = result_vec.len();
        let mut right_count = 0;
        let mut left_count = 0;

        for char in result_vec.iter() {
            let mut last_symbol = &result_vec[(vec_len - 1) - left_count];

            if Self::_punctuation_match(&char.to_string()) {
                if Self::_punctuation_match(&last_symbol.to_string()) {
                    left_count += 1;
                    continue;
                }
            } else {
                if Self::_punctuation_match(&last_symbol.to_string()) {
                    for _ in 1..(vec_len - 1) - left_count {
                        left_count += 1;
                        last_symbol = &result_vec[(vec_len - 1) - left_count];
                        if !Self::_punctuation_match(&last_symbol.to_string()) {
                            if char != last_symbol {
                                // println!("{} {}", char, last_symbol);
                                return false;
                            } else {
                                break;
                            }
                        }
                    }
                    left_count += 1;
                } else {
                    if char != last_symbol {
                        // println!("{} {}", char, last_symbol);
                        return false;
                    } else { left_count += 1; }
                }
            }
        }
        return true;
    }

    fn _split_string(s: &str) -> Vec<char> {
        let words = s.chars().collect::<Vec<_>>();
        words
    }

    fn _punctuation_match(word: &str) -> bool {
        match word {
            "~"|"`"|"!"|"@"|"#"|"$"|"%"|"^"|"&"|"*"|"("|")"|"-"|"_"|
            "="|"+"|"\\"|"|"|"/"|":"|";"|"<"|">"|" "|","|"."|"'" => true,
            _ => false,
        }
    }
}

fn main() {
    // let mut input = String::new();
    // io::stdin().read_line(&mut input).expect("Err");
    // let mut input = String::from("Madam, I'm Adam").to_lowercase();
    // let mut input = String::from("Do geese see GOD").to_lowercase();
    let mut input = String::from("А роза упала на лапу Азора").to_lowercase();
    println!("{}", input.is_polynomial());
}
