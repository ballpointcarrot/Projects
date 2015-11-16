use std::collections::HashMap;

fn get_vowels(input_string: &str) -> HashMap<char, usize> {
    let mut _map: HashMap<char, usize> = HashMap::new();
    _map.insert('a', 0);
    _map.insert('e', 0);
    _map.insert('i', 0);
    _map.insert('o', 0);
    _map.insert('u', 0);    
    for letter in input_string.to_lowercase().chars() {
        match letter {
            vowel @ 'a' |
            vowel @ 'o' |
            vowel @ 'e' |
            vowel @ 'u' |
            vowel @ 'i' => *_map.get_mut(&vowel).unwrap() += 1,
            _ => { /* noop */ }
        }
    }
    _map
}

#[test]
fn test_get_vowels() {
    let test_string = "Test all vowels in and under.";
    let mut result_map: HashMap<char, usize> = HashMap::new();
    result_map.insert('a', 2);
    result_map.insert('e', 3);
    result_map.insert('i', 1);
    result_map.insert('o', 1);
    result_map.insert('u', 1);
    
    assert!(get_vowels(test_string) == result_map);

    let test_string_2 = "Only o's.";
    let mut only_os_map: HashMap<char, usize> = HashMap::new();
    only_os_map.insert('o', 2);
    only_os_map.insert('a', 0);
    only_os_map.insert('e', 0);
    only_os_map.insert('u', 0);
    only_os_map.insert('i', 0);
    println!("{:?}", get_vowels(test_string_2));
    assert!(get_vowels(test_string_2) == only_os_map);

    let test_string_3 = "Rhythm 'n' blues";
    let mut third_string_map: HashMap<char, usize> = HashMap::new();
    third_string_map.insert('o', 0);
    third_string_map.insert('a', 0);
    third_string_map.insert('e', 1);
    third_string_map.insert('u', 1);
    third_string_map.insert('i', 0);
    assert!(get_vowels(test_string_3) == third_string_map);
    
}
