use std::collections::VecDeque;

/* 回溯 */
struct Solution {}

impl Solution {
    pub fn letter_combinations(digits: String) -> Vec<String> {
        const RANGE: [(usize, usize); 8] = [
            (0, 3),
            (3, 6),
            (6, 9),
            (9, 12),
            (12, 15),
            (15, 19),
            (19, 22),
            (22, 26),
        ];

        let acc = match digits.is_empty() {
            false => vec![String::new()],
            true => vec![],
        };

        digits.as_bytes().iter().fold(acc, |acc, c| {
            let (min, max) = RANGE[usize::from(c - 50)];
            acc.iter()
                .flat_map(|x| {
                    std::iter::repeat(x)
                        .zip(min..max)
                        .map(|(x, n)| format!("{}{}", x, (97u8 + n as u8) as char))
                })
                .collect::<Vec<String>>()
        })
    }

    pub fn combine(n: i32, k: i32) -> Vec<Vec<i32>> {
        fn dfs(cur: i32, n: i32, k: i32, tmp: &mut Vec<i32>, ans: &mut Vec<Vec<i32>>) {
            if tmp.len() + ((n - cur + 1) as usize) < k as usize {
                return;
            }
            // 记录合法的答案
            if tmp.len() == k as usize {
                ans.push(tmp.to_vec());
                return;
            }
            // 考虑选择当前位置
            tmp.push(cur);
            dfs(cur + 1, n, k, tmp, ans);
            tmp.pop();
            // 考虑不选择当前位置
            dfs(cur + 1, n, k, tmp, ans);
        }
        let mut tmp: Vec<i32> = Vec::new();
        let mut ans: Vec<Vec<i32>> = Vec::new();
        dfs(1, n, k, &mut tmp, &mut ans);
        ans
    }

    pub fn permute(nums: Vec<i32>) -> Vec<Vec<i32>> {
        if nums.len() < 1 {
            return Vec::with_capacity(0);
        }
        fn permute(nums: Vec<i32>, i: usize, rs: &mut Vec<Vec<i32>>) {
            if i == nums.len() {
                rs.push(nums);
                return;
            }
            for j in i..nums.len() {
                let mut nums = nums.clone();
                nums.swap(i, j);
                permute(nums, i + 1, rs);
            }
        }
        let mut rs = vec![];
        permute(nums, 0, &mut rs);
        rs
    }

    pub fn combination_sum(mut candidates: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
        fn back_trace(
            candidates: &Vec<i32>,
            curr: &mut Vec<i32>,
            ans: &mut Vec<Vec<i32>>,
            i: usize,
            target: i32,
        ) {
            if target == 0 {
                ans.push(curr.clone());
                return;
            }
            for j in i..candidates.len() {
                if target < candidates[j] {
                    break;
                }
                if j > 0 && candidates[j] == candidates[j - 1] {
                    continue;
                }
                curr.push(candidates[j]);
                back_trace(candidates, curr, ans, j, target - candidates[j]);
                curr.remove(curr.len() - 1);
            }
        }
        let mut ans = vec![];
        candidates.sort_unstable();
        back_trace(&candidates, &mut vec![], &mut ans, 0, target);
        ans
    }

    pub fn total_n_queens(n: i32) -> i32 {
        let n = n as usize;
        let mut res = 0;
        let mut stack = (0..n).rev().map(|j| (0, j, false)).collect::<VecDeque<_>>();
        #[allow(non_snake_case)]
        let (mut col, mut diagN, mut diagZ) = (
            vec![false; n],
            vec![false; 2 * n - 1],
            vec![false; 2 * n - 1],
        );
        while let Some((x, y, flag)) = stack.pop_back() {
            if flag {
                col[y] = false;
                diagN[n - 1 + x - y] = false;
                diagZ[x + y] = false;
            } else {
                col[y] = true;
                diagN[n - 1 + x - y] = true;
                diagZ[x + y] = true;
                stack.push_back((x, y, true));
                if x == n - 1 {
                    res += 1;
                    continue;
                }
                for j in (0..n).rev() {
                    if !col[j] && !diagN[n + x - j] && !diagZ[x + 1 + j] {
                        stack.push_back((x + 1, j, false));
                    }
                }
            }
        }
        res
    }

    pub fn generate_parenthesis(n: i32) -> Vec<String> {
        let mut m = vec![vec![String::new()]];
        for i in 1..=n as usize {
            let mut v = vec![];
            for j in 0..i {
                for p in m[j].iter() {
                    for q in m[i - 1 - j].iter() {
                        v.push(format!("{}({})", p, q));
                    }
                }
            }
            m.push(v);
        }
        m.into_iter().last().unwrap()
    }

    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        fn dfs(
            r: usize,
            c: usize,
            i: usize,
            word: &Vec<char>,
            board: &Vec<Vec<char>>,
            visited: &mut Vec<Vec<bool>>,
        ) -> bool {
            if r >= visited.len() || c >= visited[0].len() || visited[r][c] {
                return false;
            }
            if word[i] != board[r][c] {
                return false;
            }
            if i == word.len() - 1 {
                return true;
            }
            visited[r][c] = true;
            if dfs(r - 1, c, i + 1, word, board, visited)
                || dfs(r + 1, c, i + 1, word, board, visited)
                || dfs(r, c - 1, i + 1, word, board, visited)
                || dfs(r, c + 1, i + 1, word, board, visited)
            {
                return true;
            }
            visited[r][c] = false;
            false
        }
        let word = word.chars().collect::<Vec<_>>();
        for r in 0..board.len() {
            for c in 0..board[0].len() {
                let mut visited = vec![vec![false; board[0].len()]; board.len()];
                if dfs(r, c, 0, &word, &board, &mut visited) {
                    return true;
                }
            }
        }
        false
    }
}

fn main() {
    /* 100. letter_combinations */
    assert_eq!(
        Solution::letter_combinations("23".to_string()),
        vec!["ad", "ae", "af", "bd", "be", "bf", "cd", "ce", "cf"]
    );
    /* 101. combine */
    assert_eq!(
        Solution::combine(4, 2),
        vec![
            vec![2, 4],
            vec![3, 4],
            vec![2, 3],
            vec![1, 2],
            vec![1, 3],
            vec![1, 4]
        ]
    );
    /* 102.permute */
    assert_eq!(
        Solution::permute(vec![1, 2, 3]),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    );
    /* 103. combination_sum */
    assert_eq!(
        Solution::combination_sum(vec![2, 3, 6, 7], 7),
        vec![vec![2, 2, 3], vec![7]]
    );

    /* 104. total_n_queens */
    assert_eq!(Solution::total_n_queens(4), 2);

    /* 105. generate_parenthesis */
    assert_eq!(
        Solution::generate_parenthesis(3),
        vec!["((()))", "(()())", "(())()", "()(())", "()()()"]
    );

    /* 106。 exist */
    assert_eq!(
        Solution::exist(
            vec![
                vec!['A', 'B', 'C', 'E'],
                vec!['S', 'F', 'C', 'S'],
                vec!['A', 'D', 'E', 'E']
            ],
            "ABCCED".to_string()
        ),
        true
    );
}
