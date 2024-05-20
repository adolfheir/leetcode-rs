use std::collections::{HashMap, HashSet, VecDeque};
struct Solution;
impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        fn dfs(grid: &mut Vec<Vec<char>>, r: usize, c: usize) {
            grid[r][c] = '0';

            if r > 0 && r - 1 < grid.len() && grid[r - 1][c] == '1' {
                dfs(grid, r - 1, c);
            }
            if r + 1 < grid.len() && grid[r + 1][c] == '1' {
                dfs(grid, r + 1, c);
            }
            if c > 0 && c - 1 < grid[0].len() && grid[r][c - 1] == '1' {
                dfs(grid, r, c - 1);
            }
            if c + 1 < grid[0].len() && grid[r][c + 1] == '1' {
                dfs(grid, r, c + 1);
            }
        }
        let mut ans = 0;
        let mut grid = grid;
        for r in 0..grid.len() {
            for c in 0..grid[0].len() {
                if grid[r][c] == '1' {
                    ans += 1;
                    dfs(&mut grid, r, c);
                }
            }
        }
        ans
    }

    pub fn solve(board: &mut Vec<Vec<char>>) {
        // 思路：广度或深度优先遍历，从边缘为o的地方遍历，最后统一将内部的o变成X，外部遍历的改回来即可。
        let temp: char = 'T';
        let m = board.len();
        let n = board[0].len();
        let mut stack = vec![];
        for i in 0..m {
            if board[i][0] == 'O' {
                stack.push((i, 0));
                board[i][0] = temp;
            }
            if board[i][n - 1] == 'O' {
                stack.push((i, n - 1));
                board[i][n - 1] = temp;
            }
        }
        for i in 1..n - 1 {
            if board[0][i] == 'O' {
                stack.push((0, i));
                board[0][i] = temp;
            }
            if board[m - 1][i] == 'O' {
                stack.push((m - 1, i));
                board[m - 1][i] = temp;
            }
        }
        while let Some((x, y)) = stack.pop() {
            if x > 0 && board[x - 1][y] == 'O' {
                stack.push((x - 1, y));
                board[x - 1][y] = temp;
            }
            if x < m - 1 && board[x + 1][y] == 'O' {
                stack.push((x + 1, y));
                board[x + 1][y] = temp;
            }
            if y > 0 && board[x][y - 1] == 'O' {
                stack.push((x, y - 1));
                board[x][y - 1] = temp;
            }
            if y < n - 1 && board[x][y + 1] == 'O' {
                stack.push((x, y + 1));
                board[x][y + 1] = temp;
            }
        }
        //处理
        for i in 0..m {
            for j in 0..n {
                board[i][j] = match board[i][j] {
                    'O' => 'X',
                    'T' => 'O',
                    _ => 'X',
                };
            }
        }
    }

    pub fn calc_equation(
        equations: Vec<Vec<String>>,
        values: Vec<f64>,
        queries: Vec<Vec<String>>,
    ) -> Vec<f64> {
        fn dfs(
            graph: &HashMap<String, Vec<(String, f64)>>,
            visit: &mut HashMap<String, bool>,
            cur: &String,
            tar: &String,
            ans: f64,
        ) -> f64 {
            if let Some(vec) = graph.get(cur) {
                for (n, v) in vec {
                    let vis = visit.entry(n.clone()).or_insert(false);
                    if *vis {
                        continue;
                    }
                    *vis = true;

                    if n == tar {
                        return *v * ans;
                    }
                    let d = dfs(graph, visit, n, tar, ans * *v);
                    if d != -1. {
                        return d;
                    }
                }
            }
            -1.
        }

        let mut graph = HashMap::new();
        let mut ans = vec![];

        for (i, v) in equations.into_iter().enumerate() {
            let vec = graph.entry(v[0].clone()).or_insert(vec![]);
            vec.push((v[1].clone(), values[i]));
            let vec = graph.entry(v[1].clone()).or_insert(vec![]);
            vec.push((v[0].clone(), 1. / values[i]));
        }

        for v in queries {
            let c = &v[0];
            let d = &v[1];
            if !graph.contains_key(c) || !graph.contains_key(d) {
                ans.push(-1.);
                continue;
            }
            let mut visit = HashMap::new();
            ans.push(dfs(&graph, &mut visit, c, d, 1.))
        }
        ans
    }

    pub fn can_finish(mut num_courses: i32, mut prerequisites: Vec<Vec<i32>>) -> bool {
        let mut i = vec![0; num_courses as usize];
        let mut j = vec![Vec::with_capacity(10); num_courses as usize];
        prerequisites.drain(..).for_each(|x| {
            i[x[1] as usize] += 1;
            j[x[0] as usize].push(x[1]);
        });
        let mut vec: Vec<usize> = (0..num_courses as usize).filter(|&x| i[x] == 0).collect();
        num_courses -= vec.len() as i32;
        while let Some(k) = vec.pop() {
            j[k as usize].drain(..).for_each(|j| {
                i[j as usize] -= 1;
                if i[j as usize] == 0 {
                    vec.push(j as usize);
                    num_courses -= 1
                }
            })
        }
        num_courses == 0
    }

    pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
        let mut income = vec![0; num_courses as usize];
        let mut map = HashMap::new();
        prerequisites.iter().for_each(|x| {
            let (a, b) = (x[0] as usize, x[1] as usize);
            income[a] += 1;
            map.entry(b).or_insert(vec![]).push(a);
        });
        let mut courses = VecDeque::new();
        for (i, &n) in income.iter().enumerate() {
            if n == 0 {
                courses.push_back(i);
            }
        }

        let mut ans = vec![];
        while !courses.is_empty() {
            let course = courses.pop_front().unwrap();
            ans.push(course as i32);
            if let Some(next) = map.get(&course) {
                for &n in next {
                    income[n] -= 1;
                    if income[n] == 0 {
                        courses.push_back(n);
                    }
                }
            }
        }
        if ans.len() == num_courses as usize {
            ans
        } else {
            vec![]
        }
    }

    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        let mut board = board
            .into_iter()
            .rev()
            .enumerate()
            .map(|(i, mut v)| {
                if i % 2 == 0 {
                    v
                } else {
                    v.reverse();
                    v
                }
            })
            .flatten()
            .collect::<Vec<i32>>();
        let n = board.len();
        let mut vis = vec![false; n];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back((0, 0));

        while let Some((i, v)) = queue.pop_front() {
            if i == n - 1 {
                return v;
            }
            for j in (i + 1)..=(i + 6).min(n - 1) {
                let u = if board[j] == -1 {
                    j
                } else {
                    board[j] as usize - 1
                };

                if !vis[u] {
                    queue.push_back((u, v + 1));
                    vis[u] = true;
                }
            }
        }

        -1
    }

    pub fn min_mutation(start: String, end: String, bank: Vec<String>) -> i32 {
        fn compare(s1: &String, s2: &String) -> bool {
            let cnt = s1
                .chars()
                .zip(s2.chars())
                .fold(0, |cnt, (ch1, ch2)| if ch1 == ch2 { cnt } else { cnt + 1 });
            if cnt > 1 {
                false
            } else {
                true
            }
        }
        let mut q = VecDeque::new();
        let mut vis = HashSet::new();
        q.push_back((&start, 0));
        vis.insert(&start);
        while let Some((s, step)) = q.pop_front() {
            if s == &end {
                return step;
            }
            for ss in bank.iter() {
                if compare(ss, s) && !vis.contains(ss) {
                    q.push_back((ss, step + 1));
                    vis.insert(ss);
                }
            }
        }
        -1
    }

    pub fn ladder_length(begin_word: String, end_word: String, word_list: Vec<String>) -> i32 {
        let mut word_set = word_list.into_iter().collect::<HashSet<String>>();
        if !word_set.contains(&end_word) {
            return 0;
        }

        let mut word_queue = VecDeque::new();
        word_queue.push_back((begin_word, 1));

        while let Some((mut src, idx)) = word_queue.pop_front() {
            if end_word.eq(&src) {
                return idx;
            }

            let u8arr: &mut [u8] = unsafe { &mut *(src.as_bytes_mut() as *mut _) };

            let original = src.as_bytes();
            for i in 0..u8arr.len() {
                let origin_c = u8arr[i];
                for c in 'a' as u8..='z' as u8 {
                    if origin_c != c {
                        u8arr[i] = c;
                        if let Some(new_src) = word_set.take(&src) {
                            word_queue.push_back((new_src, idx + 1));
                        }
                    }
                }

                u8arr[i] = origin_c;
            }
        }

        0
    }
}

fn main() {
    /* 89. num_islands */
    let ret = Solution::num_islands(vec![
        vec!['1', '1', '1', '1', '0'],
        vec!['1', '1', '0', '1', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '0', '0', '0'],
    ]);
    println!("num_islands: {:?}", ret);

    /* 90. solve */
    let mut data = vec![
        vec!['X', 'X', 'X', 'X'],
        vec!['X', 'O', 'O', 'X'],
        vec!['X', 'X', 'O', 'X'],
        vec!['X', 'O', 'X', 'X'],
    ];

    Solution::solve(&mut data);
    println!("solve: {:?}", data);

    /* 91. calc_equation */
    let equations = vec![
        vec!["a".to_string(), "b".to_string()],
        vec!["b".to_string(), "c".to_string()],
    ];
    let values = vec![2.0, 3.0];
    let queries = vec![
        vec!["a".to_string(), "c".to_string()],
        vec!["b".to_string(), "a".to_string()],
        vec!["a".to_string(), "e".to_string()],
        vec!["a".to_string(), "a".to_string()],
        vec!["x".to_string(), "x".to_string()],
    ];
    let ret = Solution::calc_equation(equations, values, queries);

    println!("calcEquation: {:?}", ret);

    /* 92. can_finish */
    let num_courses = 2;
    let prerequisites = vec![vec![1, 0]];
    let ret = Solution::can_finish(num_courses, prerequisites);
    println!("canFinish: {}", ret);

    /* 93. find_order */
    let num_courses = 4;
    let prerequisites = vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]];
    let ret = Solution::find_order(num_courses, prerequisites);
    println!("findOrder: {:?}", ret);

    /* 94. snakes_and_ladders */
    let board = vec![
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 35, -1, -1, 13, -1],
        vec![-1, -1, -1, -1, -1, -1],
        vec![-1, 15, -1, -1, -1, -1],
    ];
    println!(
        "snakes_and_ladders: {:?}",
        Solution::snakes_and_ladders(board)
    );

    /* 95. min_mutation */
    let bank = vec![
        "AACCGGTA".to_string(),
        "AACCGCTA".to_string(),
        "AAACGGTA".to_string(),
    ];
    println!(
        "min_mutation: {}",
        Solution::min_mutation("AACCGGTT".to_string(), "AACCGGTA".to_string(), bank)
    );

    /* 96. ladder_length */
    let begin_word = "hit".to_string();
    let end_word = "cog".to_string();
    let word_list = vec![
        "hot".to_string(),
        "dot".to_string(),
        "dog".to_string(),
        "lot".to_string(),
        "log".to_string(),
        "cog".to_string(),
    ];
    println!(
        "ladder_length: {}",
        Solution::ladder_length(begin_word, end_word, word_list)
    );
}
