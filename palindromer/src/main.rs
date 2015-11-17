use std::path::Path;
use std::io::BufRead;
use std::io::BufReader;
use std::fs::File;

fn is_palindrome(word: &str) -> bool {
    match word.len() {
        0 => true,
        1 => true,
        _ => {
            let word_chars: Vec<char> = word.chars().collect();
            if word_chars.first() == word_chars.last() {
                is_palindrome(&(word[1..word.len()-1]))
            } else {
                false
            }
        }
    }
}

fn main() {
    let path = Path::new("/usr/share/dict/american-english");
    let f = File::open(path).unwrap();
    let filedata = BufReader::new(&f);

    for line in filedata.lines() {
        match line {
            Ok(actual_data) => {
                println!("{}", actual_data);   
            },
            Err(_) => {
                
            }
        }
    }
}

#[test]
fn test_is_palindrome() {
    assert!(is_palindrome("a"));
    assert!(is_palindrome("bab"));
    assert!(is_palindrome("bbaabb"));
    assert!(is_palindrome("racecar"));
    assert!(!is_palindrome("ab"));
    assert!(!is_palindrome("baby"));
    assert!(!is_palindrome("aaaaaaaaaaaaaaaaaaaac"));
}
