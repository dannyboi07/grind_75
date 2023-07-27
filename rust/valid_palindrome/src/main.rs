fn main() {
    println!("Hello, world!");
    println!(
        "{}",
        is_palindrome("A man, a plan, a canal: Panama".to_string())
    );
    println!("{}", is_palindrome("race a car".to_string()));
    println!("{}", is_palindrome("a.".to_string()));
    println!("{}", is_palindrome("0P".to_string()));
}

fn is_palindrome(s: String) -> bool {
    let s_bytes: &[u8] = s.as_bytes();
    let (mut left, mut right): (usize, usize) = (0, s.len() - 1);

    while left < right {
        let left_char: u8 = s_bytes[left];
        let right_char: u8 = s_bytes[right];

        if !left_char.is_ascii_alphanumeric() {
            left += 1;
            continue;
        } else if !right_char.is_ascii_alphanumeric() {
            right -= 1;
            continue;
        } else if left_char.to_ascii_lowercase() != right_char.to_ascii_lowercase() {
            return false;
        }

        left += 1;
        right -= 1;
    }

    true
}
