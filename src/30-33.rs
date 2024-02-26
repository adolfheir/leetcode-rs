/* 滑动窗口 */
use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut l: i32 = 0;
        let mut r: i32 = 0;
        let mut sum = 0;
        let mut ret: i32 = i32::MAX;

        while l <= r {
            if sum >= target {
                ret = ret.min(r - l);

                if l == nums.len() as i32 {
                    break;
                }

                sum -= nums[l as usize];
                l += 1;
            } else {
                if r == nums.len() as i32 {
                    break;
                }

                sum += nums[r as usize];
                r += 1;
            }
        }

        if ret == i32::MAX {
            return 0;
        } else {
            return ret;
        }
    }

    pub fn length_of_longest_substring(s: String) -> i32 {
        let s = s.chars().collect::<Vec<char>>();
        let mut set: HashSet<char> = HashSet::new();
        let mut l = 0;
        let mut r = 0;
        let mut ret = 0;

        while r < s.len() && l < s.len() {
            if set.contains(&s[r]) {
                set.remove(&s[l]);
                l += 1;
            } else {
                set.insert(s[r]);
                ret = ret.max(r - l + 1);
                r = r + 1;
            }
        }

        ret as i32
    }

    pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
        assert!(words.len() > 0);
        let word_len = words[0].len();
        if s.len() < words.len() * word_len {
            return vec![];
        }

        use std::collections::HashMap;
        let mut words_count = HashMap::<&str, u32>::new();
        for word in words.iter() {
            *words_count.entry(word).or_insert(0) += 1;
        }

        let mut answers = Vec::new();
        'l: for i in 0..=s.len() - word_len * words.len() {
            let mut m = HashMap::new();
            for j in (i..s.len()).step_by(word_len).take(words.len()) {
                let word = &s[j..j + word_len];
                if words_count.contains_key(word) {
                    *m.entry(word).or_insert(0) += 1;
                } else {
                    continue 'l;
                }
                if m[word] > words_count[word] {
                    continue 'l;
                }
            }
            if m.eq(&words_count) {
                answers.push(i as i32);
            }
        }
        answers
    }

    pub fn min_window(s: String, t: String) -> String {
        let mut ans = "";
        let mut ans_len = s.len();
        let mut count = t.len();
        let mut map = HashMap::new();
        for c in t.chars() {
            let cnt = map.entry(c).or_insert(0);
            *cnt += 1;
        }

        let ss = s.chars().collect::<Vec<char>>();

        let mut l = 0;
        for r in 0..ss.len() {
            if !map.contains_key(&ss[r]) {
                continue;
            }
            if let Some(n) = map.get_mut(&ss[r]) {
                *n -= 1;
                if *n >= 0 {
                    count -= 1;
                }
            }

            while count == 0 {
                if ans_len > r - l {
                    ans = &s[l..=r];
                    ans_len = r - l + 1;
                }
                if !map.contains_key(&ss[l]) {
                    l += 1;
                    continue;
                }
                if let Some(n) = map.get_mut(&ss[l]) {
                    *n += 1;
                    if *n > 0 {
                        count += 1;
                    }
                }
                l += 1;
            }
        }
        ans.to_string()
    }
}

fn main() {
    /* 30. min_sub_array_len */
    let nums = vec![1, 1, 1, 1, 1, 1, 1, 1];
    let target = 11;
    let ret = Solution::min_sub_array_len(target, nums);
    println!("{}", ret);

    /* 31. length_of_longest_substring */
    let s = "abcabcbb".to_string();
    let ret = Solution::length_of_longest_substring(s);
    println!("{}", ret);

    /* 32. find_substring */
    let s = "barfoothefoobarman".to_string();
    let words = vec!["foo".to_string(), "bar".to_string()];
    let ret = Solution::find_substring(s, words);
    println!("{:?}", ret);

    /* 33. min_window */
    let s = "wordgoodgoodgoodbestword".to_string();
    let t = "ABC".to_string();
    let ret = Solution::min_window(s, t);
    println!("{}", ret);
}
