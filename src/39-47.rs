/* 哈希表 */
use std::collections::{HashMap, HashSet};

struct Solution;
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        let mut magazine_map: HashMap<char, i32> = HashMap::new();
        for c in magazine.chars() {
            let cnt = magazine_map.entry(c).or_insert(0);
            *cnt += 1;
        }
        for zh in ransom_note.chars() {
            if magazine_map.contains_key(&zh) {
                let cnt = magazine_map.get_mut(&zh).unwrap();
                if *cnt > 0 {
                    *cnt -= 1;
                } else {
                    return false;
                }
            } else {
                return false;
            }
        }

        true
    }

    pub fn is_isomorphic(s: String, t: String) -> bool {
        let mut hash_s: HashMap<char, char> = HashMap::new();
        let mut hash_t: HashMap<char, char> = HashMap::new();

        for (sv, tv) in s.chars().zip(t.chars()) {
            let up_s = hash_s.insert(sv, tv);

            match up_s {
                Some(value) => {
                    if value != tv {
                        return false;
                    }
                }
                None => (),
            }

            let up_t = hash_t.insert(tv, sv);

            if let Some(value) = up_t {
                if value != sv {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn word_pattern(pattern: String, s: String) -> bool {
        let s = s.split_ascii_whitespace().collect::<Vec<&str>>();

        if s.len() != pattern.len() {
            return false;
        }

        let mut hash_s = HashMap::new();
        let mut hash_t = HashMap::new();

        for (sv, tv) in s.iter().zip(pattern.chars()) {
            let up_s = hash_s.insert(sv, tv);

            match up_s {
                Some(value) => {
                    if value != tv {
                        return false;
                    }
                }
                None => (),
            }

            let up_t = hash_t.insert(tv, sv);

            if let Some(value) = up_t {
                if value != sv {
                    return false;
                }
            }
        }

        return true;
    }

    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        }
        let mut hash = [0; 26];

        s.chars()
            .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] += 1);
        t.chars()
            .for_each(|ch| hash[(ch as u8 - 'a' as u8) as usize] -= 1);

        !hash.iter().any(|&x| x != 0)
    }

    pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
        let mut hash_map: HashMap<String, Vec<String>> = HashMap::new();

        for s in strs {
            let mut vec_s = s.chars().collect::<Vec<char>>();
            vec_s.sort();
            let key = vec_s.iter().collect::<String>();

            let vet = hash_map.entry(key).or_insert(Vec::new());
            vet.push(s);
        }

        hash_map.into_values().collect::<Vec<Vec<String>>>()
    }
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut hash_map: HashMap<i32, i32> = HashMap::new();
        for (k, &v) in nums.iter().enumerate() {
            if let Some(&value) = hash_map.get(&(target - v)) {
                return vec![value as i32, k as i32];
            }

            hash_map.insert(v, k as i32);
        }

        unreachable!()
    }

    pub fn is_happy(n: i32) -> bool {
        fn kuaile(n: i32, mut arr: Vec<i32>) -> bool {
            let mut num = n;
            let mut temp: i32 = 0;
            while num > 0 {
                temp += (num % 10) * (num % 10);
                num /= 10;
            }
            if temp == 1 {
                return true;
            } else if arr.contains(&temp) {
                return false;
            } else {
                arr.push(temp);
                kuaile(temp, arr)
            }
        }
        kuaile(n, Vec::new())
    }

    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        let mut cache = HashMap::new();

        for i in 0..nums.len() {
            if let Some(&value) = cache.get(&nums[i]) {
                if f64::abs((value as f64 - i as f64) as f64) <= k as f64 {
                    return true;
                }
            }
            cache.insert(nums[i], i);
        }
        false
    }

    pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
        let d: HashSet<_> = nums.iter().map(|&i| i).collect();
        nums.into_iter()
            .map(|mut i| {
                let mut t = 1;
                if !d.contains(&(i - 1)) {
                    while d.contains(&(i + 1)) {
                        t += 1;
                        i += 1;
                    }
                }
                t
            })
            .max()
            .unwrap_or(0)
    }
}

fn main() {
    /* 39. can_construct */
    let ransom_note = "aa".to_string();
    let magazine = "aab".to_string();
    let ret = Solution::can_construct(ransom_note, magazine);
    println!("{}", ret);

    /* 40. is_isomorphic */
    let s = "egg".to_string();
    let t = "add".to_string();
    let ret = Solution::is_isomorphic(s, t);
    println!("{}", ret);

    /* 41. word_pattern */
    let pattern = "aaa".to_string();
    let s = "aa aa aa aa".to_string();
    let ret = Solution::word_pattern(pattern, s);
    println!("{}", ret);

    /* 42. is_anagram */
    let s = "anagram".to_string();
    let t = "nagaram".to_string();
    let ret = Solution::is_anagram(s, t);
    println!("{}", ret);

    /* 43. group_anagrams */
    let strs = vec![
        "eat".to_string(),
        "tea".to_string(),
        "tan".to_string(),
        "ate".to_string(),
        "nat".to_string(),
        "bat".to_string(),
    ];
    let ret = Solution::group_anagrams(strs);
    println!("{:?}", ret);

    /* 44. two_sum */
    let nums = vec![2, 7, 11, 15];
    let target = 9;
    let ret = Solution::two_sum(nums, target);
    println!("{:?}", ret);

    /* 45. is_happy */
    let n = 19;
    let ret = Solution::is_happy(n);
    println!("{}", ret);

    /* 46. contains_nearby_duplicate */
    let nums = vec![1, 0, 1, 1];
    let k = 3;
    let ret = Solution::contains_nearby_duplicate(nums, k);
    println!("{}", ret);

    /* 47. longest_consecutive */
    let nums = vec![100, 4, 200, 1, 3, 2];
    let ret = Solution::longest_consecutive(nums);
    println!("{}", ret);
}
