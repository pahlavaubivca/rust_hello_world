use std::collections::HashMap;
use log::info;

pub struct Sum2Nums {}

impl Sum2Nums {
    fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        println!("nums len {}. target {}", nums.len(), target);
        let mut result: Vec<i32> = Vec::new();
        let mut nums_hash: HashMap<&i32, usize> = HashMap::new();
        let mut iteration = 0;
        for i in nums.iter().enumerate()
        {
            iteration += 1;
            // let desired_number = target - i.1;
            if let Some(res) = nums_hash.get(&(target - i.1)) {
                result.append(vec![i.0 as i32, *res as i32].as_mut());
                break;
            }

            if !nums_hash.contains_key(i.1) {
                nums_hash.insert(i.1, i.0);
            }
        };
        println!("found indexes {} {}. iteration for find {}", result[0],result[1], iteration);
        result
    }

    pub fn run() {
        let nums = vec![3, 4, -6, 45, 0, 23, 2, 5, 6, 1, 6, 7, 98, 7, 65, 4, 4, 3, 5, 6, 7, 8];
        let target = 100;
        let result = Self::two_sum(nums, target);
        println!("result {:?} ",result);
    }
}