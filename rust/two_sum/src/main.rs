use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    println!("{:?}", two_sum(vec![2, 7, 11, 15], 9))
}

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut track_map: HashMap<i32, i32> = HashMap::new();

    for (i, num) in nums.iter().enumerate() {
        let num_target: i32 = target - num;

        if track_map.contains_key(&num_target) {
            return vec![*track_map.get(&num_target).unwrap(), i as i32];
        }

        track_map.insert(*num, i as i32);
    }

    vec![]
}
