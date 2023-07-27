use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    // println!(
    //     "{}",
    //     is_anagram("anagram".to_string(), "nagaram".to_string())
    // );
    // println!("{}", is_anagram("rat".to_string(), "car".to_string()));
    println!("{}", is_anagram("aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaab".to_string(), "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbba".to_string()));
}

fn is_anagram(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let s_bytes: &[u8] = s.as_bytes();
    let t_bytes: &[u8] = t.as_bytes();
    let mut count_map: HashMap<u8, i16> = HashMap::new();

    for i in 0..s.len() {
        count_map
            .entry(s_bytes[i])
            .and_modify(|count: &mut i16| *count += 1)
            .or_insert(1);

        count_map
            .entry(t_bytes[i])
            .and_modify(|count: &mut i16| *count -= 1)
            .or_insert(-1);
    }

    for c_count in count_map.values() {
        if *c_count != 0 {
            return false;
        }
    }

    true
}

// let mut s_count_map: HashMap<u8, u8> = HashMap::new();
// s.bytes().for_each(|c_byte: u8| {
//     s_count_map
//         .entry(c_byte)
//         .and_modify(|count| *count += 1)
//         .or_insert(1);
// });

// let mut result: bool = true;
// let mut t_count_map: HashMap<u8, u8> = HashMap::new();
// t.bytes().for_each(|c_byte| {
//     let t_char_count: &mut u8 = t_count_map
//         .entry(c_byte)
//         .and_modify(|count| *count += 1)
//         .or_insert(1);
//     if !s_count_map.contains_key(&c_byte) || s_count_map.get(&c_byte).unwrap() < t_char_count {
//         result = false;
//     }
// });
