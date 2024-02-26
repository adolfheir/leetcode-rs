/* 双指针 */
use core::num;
use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut s = s;
        s.retain(|c| c.is_ascii_alphanumeric());
        s = s.to_lowercase();
        let _s = s.chars().rev().collect::<String>();
        s == _s
    }

    //s :abc t:abcdef
    pub fn is_subsequence(s: String, t: String) -> bool {
        let s = s.chars().collect::<Vec<char>>();
        let t = t.chars().collect::<Vec<char>>();

        let mut si = 0;
        let mut ti = 0;

        while ti < t.len() && si < s.len() {
            if s[si] == t[ti] {
                si += 1;
            }
            ti += 1;
        }

        si == s.len()
    }

    pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i = 0;
        let mut j = numbers.len() - 1;

        while i < j {
            if numbers[i] + numbers[j] == target {
                return vec![i as i32 + 1, j as i32 + 1];
            } else if numbers[i] + numbers[j] < target {
                i += 1;
            } else {
                j -= 1;
            }
        }

        vec![]
    }

    pub fn max_area(height: Vec<i32>) -> i32 {
        let mut i = 0;
        let mut j = height.len() - 1;
        let mut ret = 0;

        while i < j {
            let min = height[i].min(height[j]);
            let temp = (j - i) as i32 * min;
            ret = ret.max(temp);

            if height[i] < height[j] {
                i += 1;
            } else {
                j -= 1;
            }
        }

        ret
    }
    pub fn three_sum(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut ret = vec![];
        let mut nums = nums;
        nums.sort();

        for i in 0..nums.len() {
            if nums[i] > 0 {
                break;
            }
            if i > 0 && nums[i] == nums[i - 1] {
                continue;
            }
            let mut l = i + 1;
            let mut r = nums.len() - 1;
            while l < r {
                let sum = nums[i] + nums[l] + nums[r];
                if sum == 0 {
                    ret.push(vec![nums[i], nums[l], nums[r]]);
                    while r < nums.len() - 1 && nums[r] == nums[r - 1] {
                        r -= 1;
                    }
                    while l < nums.len() - 1 && nums[l] == nums[l + 1] {
                        l += 1;
                    }
                }
                if sum > 0 {
                    r -= 1;
                } else {
                    l += 1;
                }
            }
        }
        ret
    }
}

fn main() {
    /* 25. is_palindrome */
    let s = "A man, a plan, a canal: Panama".to_string();
    let ret = Solution::is_palindrome(s);
    println!("{}", ret);

    /* 26. is_palindrome2 */
    let s = "a1bc".to_string();
    let t = "abcdef".to_string();
    let ret = Solution::is_subsequence(s, t);
    println!("{}", ret);

    /* 27. is_palindrome2 */
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ret = Solution::two_sum(nums, target);
    println!("{:?}", ret);

    /* 28. is_palindrome2 */
    let height = vec![1, 8, 6, 2, 5, 4, 8, 3, 7];
    let ret = Solution::max_area(height);
    println!("{}", ret);

    // 29. three_sum
    let nums = vec![-1, 0, 1, 2, -1, -4];
    let ret = Solution::three_sum(nums);
    println!("{:?}", ret);
}
