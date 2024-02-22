use core::num;
use rand::Rng;
use std::cmp::Reverse;
use std::collections::{HashMap, HashSet};

struct RandomizedSet {
    data: Vec<i32>,
    map: HashMap<i32, usize>,
}

impl RandomizedSet {
    fn new() -> Self {
        Self {
            data: vec![],
            map: HashMap::new(),
        }
    }

    fn insert(&mut self, val: i32) -> bool {
        if self.map.contains_key(&val) {
            return false;
        }
        self.map.insert(val, self.data.len());
        self.data.push(val);
        return true;
    }

    fn remove(&mut self, val: i32) -> bool {
        if !self.map.contains_key(&val) {
            return false;
        }

        let index = self.map.remove(&val).unwrap();
        let last = self.data.pop().unwrap();
        if index != self.data.len() {
            self.data[index] = last;
            self.map.insert(last, index);
        }

        return true;
    }

    fn get_random(&self) -> i32 {
        let len = self.map.len();

        if len == 0 {
            return -1;
        }
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..len);
        return self.data[idx];
    }
}

struct Solution;
impl Solution {
    pub fn h_index(citations: Vec<i32>) -> i32 {
        let mut citations = citations.clone();
        citations.sort();
        for (i, value) in citations.iter().enumerate() {
            if *value as usize >= citations.len() - i {
                return (citations.len() - i) as i32;
            }
        }
        return 0;
    }

    pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
        let mut list = vec![1; nums.len()];
        for i in 1..nums.len() {
            list[i] = nums[i - 1] * list[i - 1];
        }
        println!("list: {:#?}", list);
        let mut temp = 1;
        for i in (0..nums.len()).rev() {
            list[i] = temp * list[i];
            temp *= nums[i]
        }

        list
    }

    pub fn can_complete_circuit(gas: Vec<i32>, cost: Vec<i32>) -> i32 {
        let mut sum = 0;
        let mut start_index = 0;
        let mut temp = 0;

        for i in 0..gas.len() {
            let diff = gas[i] - cost[i];
            sum += diff;
            temp += diff;

            if temp < 0 {
                start_index = i + 1;
                temp = 0;
            }
        }

        if sum < 0 {
            return -1;
        }

        start_index as i32
    }

    pub fn candy(ratings: Vec<i32>) -> i32 {
        let mut nums = vec![1; ratings.len()];

        for i in 0..nums.len() {
            if i > 0 && ratings[i] > ratings[i - 1] {
                nums[i] = nums[i - 1] + 1;
            }
        }

        let mut sum = nums[nums.len() - 1];
        for i in (0..nums.len() - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                nums[i] = nums[i].max(nums[i + 1] + 1);
            }
            sum += nums[i];
        }

        sum
    }

    pub fn trap(height: Vec<i32>) -> i32 {
        let mut left = 0;
        let mut right = height.len() - 1;
        let mut sum = 0;

        let mut left_max = 0;
        let mut right_max = 0;

        //核心 是算left 还是算right
        while left < right {
            left_max = height[left].max(left_max);
            right_max = height[right].max(right_max);

            if height[left] < height[right] {
                sum += left_max - height[left];
                left += 1;
            } else {
                sum += right_max - height[right];
                right -= 1;
            }
        }
        sum
    }

    pub fn roman_to_int(s: String) -> i32 {
        s.chars()
            .rev()
            .fold((0, 0), |res, cur| {
                let n = match cur {
                    'I' => 1,
                    'V' => 5,
                    'X' => 10,
                    'L' => 50,
                    'C' => 100,
                    'D' => 500,
                    'M' => 1000,
                    _ => -9999,
                };
                (if n < res.1 { res.0 - n } else { res.0 + n }, n)
            })
            .0
    }

    pub fn int_to_roman(num: i32) -> String {
        let mut num = num;
        let mut roman = String::new();
        while num > 0 {
            match num {
                1000.. => {
                    num -= 1000;
                    roman.push_str("M");
                }
                900.. => {
                    num -= 900;
                    roman.push_str("CM");
                }
                500.. => {
                    num -= 500;
                    roman.push_str("D");
                }
                400.. => {
                    num -= 400;
                    roman.push_str("CD");
                }
                100.. => {
                    num -= 100;
                    roman.push_str("C");
                }
                90.. => {
                    num -= 90;
                    roman.push_str("XC");
                }
                50.. => {
                    num -= 50;
                    roman.push_str("L");
                }
                40.. => {
                    num -= 40;
                    roman.push_str("XL");
                }
                10.. => {
                    num -= 10;
                    roman.push_str("X");
                }
                9.. => {
                    num -= 9;
                    roman.push_str("IX");
                }
                5.. => {
                    num -= 5;
                    roman.push_str("V");
                }
                4.. => {
                    num -= 4;
                    roman.push_str("IV");
                }
                1.. => {
                    num -= 1;
                    roman.push_str("I");
                }
                _ => {}
            }
        }
        roman
    }

    pub fn length_of_last_word(s: String) -> i32 {
        s.split_whitespace().last().unwrap().len() as i32
    }
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        let mut ans = 0;
        for i in 0..strs[0].len() {
            for s in strs.iter() {
                let s = s.as_bytes();
                if s.len() == i || strs[0].as_bytes()[i] != s[i] {
                    return strs[0][0..ans].to_string();
                }
            }
            ans += 1;
        }
        strs[0][0..ans].to_string()
    }
}

fn main() {
    /* 11. h_index */
    // let num1 = vec![1, 3, 1];
    // let ret1 = Solution::h_index(num1);
    // println!("{:?}", ret1);

    /* 12. RandomizedSet */
    // let mut obj: RandomizedSet = RandomizedSet::new();
    // let ret_1: bool = obj.insert(3);
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_1: bool = obj.insert(3);
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_3: i32 = obj.get_random();
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_3: i32 = obj.get_random();
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_1: bool = obj.insert(1);
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_2: bool = obj.remove(3);
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_3: i32 = obj.get_random();
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_3: i32 = obj.get_random();
    // println!("len: {:?},{:?}", obj.map, obj.data);

    // let ret_1: bool = obj.insert(0);
    // println!("len: {:?},{:?}", obj.map, obj.data);
    // let ret_2: bool = obj.remove(0);
    // println!("len: {:?},{:?}", obj.map, obj.data);

    // println!("{:?}, {:?}, {:?}", ret_1, ret_2, ret_3);

    /* 13. product_except_self */
    // let num1 = vec![1,2,3,4];
    // let ret1 = Solution::product_except_self(num1);
    // println!("{:?}", ret1);

    /* 14. can_complete_circuit */
    // let gas = vec![1, 2, 3, 4, 5];
    // let cost = vec![3, 4, 5, 1, 2];
    // let ret1 = Solution::can_complete_circuit(gas, cost);
    // println!("{:?}", ret1);

    /* 15. candy */
    // let ratings = vec![1, 0, 2];
    // let ret1 = Solution::candy(ratings);
    // println!("{:?}", ret1);

    /* 16. trap*/
    // let height = vec![0, 1, 0, 2, 1, 0, 1, 3, 2, 1, 2, 1];
    // let ret1 = Solution::trap(height);
    // println!("{:?}", ret1);

    /* 17. roman_to_int */
    let s = "MCMXCIV".to_string();
    let ret1 = Solution::roman_to_int(s);
    println!("{:?}", ret1);

    /* 18. int_to_roman */
    let num = 3999;
    let ret1 = Solution::int_to_roman(num);
    println!("{:?}", ret1);

    /* 19. length_of_last_word */
    let s = "Hello World 23".to_string();
    let ret1 = Solution::length_of_last_word(s);
    println!("{:?}", ret1);

    /* 20. longest_common_prefix */
    let strs = vec![
        "flower".to_string(),
        "flow".to_string(),
        "flight".to_string(),
    ];

    // let strs = vec!["a".to_string()];
    let ret1 = Solution::longest_common_prefix(strs);
    println!("{:?}", ret1);
}
