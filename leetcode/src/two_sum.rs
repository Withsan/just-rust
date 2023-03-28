use std::collections::HashMap;

pub struct Solution;

impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map: HashMap<i32, i32> = HashMap::with_capacity(nums.capacity());
        for (index, num) in nums.iter().enumerate() {
            let i = &(target - num);
            if let Some(v) = map.get(i) {
                return vec![index as i32, v.to_owned()];
            }
            map.insert(num.to_owned(), index as i32);
        }
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let solution = Solution::two_sum(vec![2, 7, 11, 15], 9);
        println!("{:?}", solution);
    }
}
