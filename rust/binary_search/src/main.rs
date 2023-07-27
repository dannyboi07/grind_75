fn main() {
    println!("Hello, world!");
    println!("{}", search(vec![-1, 0, 3, 5, 9, 12], 9));
    println!("{}", search(vec![-1, 0, 3, 5, 9, 12], 2));
    println!("{}", search(vec![5], -5));
}

fn search(nums: Vec<i32>, target: i32) -> i32 {
    let (mut left, mut right): (i32, i32) = (0, (nums.len() - 1).try_into().unwrap());

    while left <= right {
        let mid: i32 = (right + left) / 2;

        let mid_num: i32 = nums[mid as usize];
        if mid_num == target {
            return mid as i32;
        } else if target < mid_num {
            right = mid - 1;
        } else if target > mid_num {
            left = mid + 1;
        }
    }

    -1
}
