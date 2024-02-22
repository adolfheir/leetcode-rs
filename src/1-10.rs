use core::num;
use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut m = m as usize;
        let mut n = n as usize;
        let mut right = nums1.len();
        while n > 0 {
            right -= 1;
            if m == 0 || nums1[m - 1] < nums2[n - 1] {
                nums1[right] = nums2[n - 1];
                if n > 0 {
                    n -= 1
                }
            } else {
                nums1.swap(m - 1, right);
                if m > 0 {
                    m -= 1
                }
            }
        }
    }

    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        nums.retain(|&x| x != val);
        nums.len() as i32
    }

    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        nums.dedup();
        nums.len() as i32
    }

    pub fn remove_duplicates2(nums: &mut Vec<i32>) -> i32 {
        let n = nums.len();
        if n <= 2 {
            return n as i32;
        }

        let mut slow = 2;
        let mut fast = 2;

        while fast < n {
            if nums[slow - 2] != nums[fast] {
                nums[slow] = nums[fast];
                slow += 1;
            }
            fast += 1;
        }

        slow as i32
    }

    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut count = 0;
        let mut temp = nums[0];

        for num in nums {
            if count == 0 {
                temp = num;
            }
            count += if num == temp { 1 } else { -1 };
        }

        temp
    }

    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let _k = k as usize % nums.len();
        nums.rotate_right(_k)
    }

    pub fn max_profit(prices: Vec<i32>) -> i32 {
        let mut min = prices[0];
        let mut ret = 0;

        for price in prices {
            min = min.min(price);
            ret = ret.max(price - min);
        }
        ret
    }

    /* 动态规划 */
    pub fn max_profit2(prices: Vec<i32>) -> i32 {
        let n = prices.len();
        let mut vec = vec![(0, 0); n];
        // 0 代表有股票 1代表没股票
        // f[n][0]= max（f[n-1][0] , f[n-1][1]- prices[n] )
        // f[n][1]= max（f[n-1][1] , f[n-1][0]+ prices[n] )

        vec[0].0 = -prices[0];
        vec[0].1 = 0;

        for i in 1..n {
            vec[i].0 = vec[i - 1].0.max(vec[i - 1].1 - prices[i]);
            vec[i].1 = vec[i - 1].1.max(vec[i - 1].0 + prices[i]);
        }
        println!("{:?}", vec);
        vec[n - 1].1
    }

    /* 贪心实现 */
    pub fn max_profit2_2(prices: Vec<i32>) -> i32 {
        let mut ret = 0;

        let n = prices.len();
        for i in 1..n {
            let diff = prices[i] - prices[i - 1];
            if (diff > 0) {
                ret += diff;
            }
        }
        ret
    }

    pub fn can_jump(nums: Vec<i32>) -> bool {
        let len = nums.len() as i32;
        if len == 1 {
            return true;
        }

        let mut max = nums[0];
        let mut i = 0;

        while i < max {
            max = max.max(i + 1 + nums[i as usize]);
            println!("{:?},{}", max, len);
            if max >= len {
                return true;
            }
            //提前结束
            if max < i + 1 {
                return false;
            }
            i += 1
        }

        false
    }

    pub fn jump(nums: Vec<i32>) -> i32 {
        let len = nums.len();
        let mut count = 0;

        let mut end = 0;
        let mut temp_end = 0;

        for i in 0..len - 1 {
            temp_end = temp_end.max(i as i32 + nums[i]);

            if i as i32 == end {
                end = temp_end;
                count += 1
            }
        }

        count
    }
}

fn main() {
    /* 1. merge */
    // let num1 = &mut vec![1, 2, 3, 0, 0, 0];
    // Solution::merge(num1, 3, &mut vec![2, 5, 6], 3);
    // println!("{:?}", num1);

    /* 2. remove_element */
    // let num2 = &mut vec![3, 2, 2, 3];
    // let ret2 = Solution::remove_element(num2, 3);
    // println!("{:?},{}", num2, ret);

    /* 3. remove_duplicates */
    // let num3 = &mut vec![3, 2, 2, 3];
    // let ret3 = Solution::remove_duplicates(num3);
    // println!("{:?},{}", num3, ret3);

    /* 4. remove_duplicates 2 */
    // let num4 = &mut vec![0, 0, 1, 1, 1, 1, 2, 3, 3];
    // let ret4 = Solution::remove_duplicates2(num4);
    // println!("{:?},{}", num4, ret4);

    /* 5. majority_element */
    // let num5 = vec![2, 2, 1, 1, 1, 2, 2];
    // let ret5 = Solution::majority_element(num5);
    // println!("{}", ret5);

    /* 6. rotate */
    // let mut num6 = vec![1, 2, 3, 4, 5, 6, 7];
    // Solution::rotate(&mut num6, 3);
    // println!("{:?}", num6);

    /* 7. max_profit */
    // let num7 = vec![7, 1, 5, 3, 6, 4];
    // let ret7 = Solution::max_profit(num7);
    // println!("{:?}", ret7);

    /* 8. max_profit2 */
    // let num8 = vec![7, 1, 5, 3, 6, 4];
    // let ret8 = Solution::max_profit2(num8);
    // println!("{:?}", ret8);

    /* 8.1 max_profit2 */
    // let num8_1 = vec![7, 1, 5, 3, 6, 4];
    // let ret8_1 = Solution::max_profit2_2(num8_1);
    // println!("{:?}", ret8_1);

    /* 9. can_jump */
    // let num9 = vec![2, 0, 0];
    // let ret9 = Solution::can_jump(num9);
    // println!("{:?}", ret9);

    /* 10. jump */
    let num10 = vec![2, 3, 1, 1, 4];
    let ret10 = Solution::jump(num10);
    println!("{:?}", ret10);
}
