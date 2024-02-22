use core::num;
use std::collections::HashSet;

struct Solution;
impl Solution {
    pub fn reverse_words(s: String) -> String {
        let mut s = s;
        s.split_whitespace().rev().collect::<Vec<&str>>().join(" ")
    }
    pub fn convert(s: String, num_rows: i32) -> String {
        let num_rows = num_rows as usize;
        let mut arr = vec![String::new(); num_rows];
        let iter = (0..num_rows).chain((1..num_rows - 1).rev()).cycle();
        iter.zip(s.chars()).for_each(|(i, c)| arr[i].push(c));
        arr.into_iter().collect()
    }

    pub fn str_str(haystack: String, needle: String) -> i32 {
        haystack.find(&needle).unwrap_or(usize::MAX) as i32
    }

    pub fn full_justify(words: Vec<String>, max_width: i32) -> Vec<String> {
        let l = words.len();
        let max_width = max_width as usize;
        let mut queue = Vec::new();
        let mut i = 0;
        let mut count = 0;
        let mut tmp = Vec::new();

        while i < l {
            count = 0;
            tmp = Vec::new();

            while i < l && count < max_width {
                // 按max_with将单词分组
                if count + words[i].len() > max_width {
                    break;
                }

                count += (words[i].len() + 1);
                tmp.push(words[i].clone());
                i += 1;
            }

            if i == l {
                // 最后一行，在单词后增加空格
                let last_line = tmp[..].join(&" ");
                let len = last_line.len();
                queue.push(last_line + &" ".repeat(max_width - len));
                break;
            }

            if tmp.len() == 1 {
                // 分组长度为1，在单词后增加空格
                queue.push(tmp[..].join(&"") + &" ".repeat(max_width - count + 1));
            } else {
                let t = tmp.len() - 1;
                let paddings = max_width - (count - t - 1);
                let space = paddings / t; // 每个单词后应该增加的平均空格数
                let mut extra = (paddings % t) as i32; // 如果每个单词后增加的空格数不能被平均，将额外空格分摊到前几个单词后面
                let res = tmp
                    .into_iter()
                    .enumerate()
                    .map(|(i, x)| {
                        let times = space + if extra <= 0 { 0 } else { 1 };
                        extra -= 1;

                        if i == t {
                            x
                        } else {
                            x + &" ".repeat(times)
                        }
                    })
                    .collect::<String>();
                queue.push(res);
            }
        }

        queue
    }
}

fn main() {
    /* 21. reverse_words */
    let s = "the sky is blue".to_string();
    let ret = Solution::reverse_words(s);
    println!("{:?}", ret);

    /* 22. convert */
    let s = "PAYPALISHIRING".to_string();
    let num_rows = 3;
    let ret = Solution::convert(s, num_rows);
    println!("{:?}", ret);

    /* 23. str_str */
    let haystack = "hello".to_string();
    let needle = "ll".to_string();
    let ret = Solution::str_str(haystack, needle);
    println!("{:?}", ret);

    /* 24. full_justify */
    let words = vec![
        "This".to_string(),
        "is".to_string(),
        "an".to_string(),
        "example".to_string(),
        "of".to_string(),
        "text".to_string(),
        "justification.".to_string(),
    ];
    let max_width = 16;
    let ret = Solution::full_justify(words, max_width);
    println!("{:?}", ret);

}
