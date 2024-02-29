/* 区间 */

use std::cmp::Ordering;

struct Solution;
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut ret: Vec<String> = vec![];
        let mut i = 0;

        while i < nums.len() {
            let low = i;

            i = i + 1;

            while i < nums.len() && nums[i] == nums[i - 1] + 1 {
                i = i + 1;
            }
            let high = i - 1;

            if low < high {
                let mut s = String::new();
                s.push_str(nums[low].to_string().as_str());
                s.push_str("->");
                s.push_str(nums[high].to_string().as_str());
                ret.push(s);
            } else {
                ret.push(nums[low].to_string());
            }
        }

        ret
    }

    pub fn merge(intervals: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        let mut sort_vec = intervals;
        sort_vec.sort_by(|a, b| {
            let a = a[0];
            let b = b[0];

            if a < b {
                return Ordering::Less;
            } else if a > b {
                return Ordering::Greater;
            } else {
                return Ordering::Equal;
            }
        });

        let mut ret: Vec<Vec<i32>> = vec![];

        let (mut l, mut r) = (sort_vec[0][0], sort_vec[0][1]);

        for i in 1..sort_vec.len() {
            let vec = &sort_vec[i];
            let left = vec[0];
            let right = vec[1];

            if left <= r {
                r = r.max(right);
            } else {
                ret.push(vec![l, r]);

                (l, r) = (sort_vec[i][0], sort_vec[i][1])
            }
        }
        ret.push(vec![l, r]);

        return ret;
    }

    pub fn insert(mut intervals: Vec<Vec<i32>>, new_interval: Vec<i32>) -> Vec<Vec<i32>> {
        intervals.push(new_interval);
        return Solution::merge(intervals);
    }

    pub fn find_min_arrow_shots(mut points: Vec<Vec<i32>>) -> i32 {
        if points.len() == 0 {
            return 0;
        }

        points.sort_by_key(|x| x[1]);

        let mut count = 1;
        let mut pos = points[0][1];

        for i in 0..points.len() {
            let point = &points[i];
            if point[0] > pos {
                count += 1;
                pos = point[1];
            }
        }

        count
    }
}

fn main() {
    /* 48. summary_ranges */
    let nums = vec![0, 1, 2, 4, 5, 7];
    let ret = Solution::summary_ranges(nums);
    println!("{:?}", ret);

    /* 49. merge */
    let intervals = vec![vec![1, 4], vec![2, 3]];
    let ret = Solution::merge(intervals);
    println!("{:?}", ret);

    /* 50. insert */
    let intervals = vec![vec![1, 3], vec![6, 9]];
    let new_interval = vec![2, 5];
    let ret = Solution::insert(intervals, new_interval);
    println!("{:?}", ret);

    /* 51. find_min_arrow_shots */
    let points: Vec<Vec<i32>> = vec![vec![1, 2], vec![3, 4], vec![5, 6], vec![7, 8], vec![9, 10]];
    let ret = Solution::find_min_arrow_shots(points);
    println!("{}", ret);
}
