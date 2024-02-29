/* 栈 */

struct MinStack {
    stk: Vec<i32>,
    min: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        Self {
            stk: vec![],
            min: vec![],
        }
    }

    fn push(&mut self, val: i32) {
        self.stk.push(val);
        if self.min.is_empty() || val <= *self.min.last().unwrap() {
            self.min.push(val);
        }
    }

    fn pop(&mut self) {
        if self.stk.pop().unwrap() == *self.min.last().unwrap() {
            self.min.pop();
        }
    }

    fn top(&self) -> i32 {
        return *self.stk.last().unwrap();
    }

    fn get_min(&self) -> i32 {
        return *self.min.last().unwrap();
    }
}

struct Solution;
impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = vec![];

        for c in s.chars() {
            println!("stack : {:?}", stack);
            match c {
                '(' | '[' | '{' => {
                    stack.push(c);
                    println!("push : {:?}", c);
                }
                ')' => {
                    if stack.pop() != Some('(') {
                        return false;
                    }
                }
                ']' => {
                    if stack.pop() != Some('[') {
                        return false;
                    }
                }
                '}' => {
                    if stack.pop() != Some('{') {
                        return false;
                    }
                }
                _ => {}
            }
        }
        stack.is_empty()
    }

    pub fn simplify_path(path: String) -> String {
        let mut stack: Vec<&str> = vec![];

        path.split("/").for_each(|v| match v {
            "." | "" => {}
            ".." => {
                stack.pop();
            }
            _ => stack.push(v),
        });

        let ret = stack.join("/");
        "/".to_string() + &ret
    }

    pub fn eval_rpn(tokens: Vec<String>) -> i32 {
        let mut stack: Vec<String> = vec![];

        for s in tokens {
            let v = s.as_str();
            match v {
                "+" | "-" | "*" | "/" => {
                    let b = stack.pop().unwrap().parse::<i32>().unwrap();
                    let a = stack.pop().unwrap().parse::<i32>().unwrap();
                    let c = match v {
                        "+" => a + b,
                        "-" => a - b,
                        "*" => a * b,
                        "/" => a / b,
                        _ => 0,
                    };
                    stack.push(c.to_string())
                }
                _ => {
                    stack.push(s);
                }
            }
        }

        stack[0].parse::<i32>().unwrap()
    }

    pub fn calculate(s: String) -> i32 {
        let bs = s.as_bytes();

        let mut res: i32 = 0;
        let mut flags = vec![];
        let mut current_num: i32 = 0;
        let mut last_operator = 0;

        for i in 0..bs.len() {
            let mut flag: u8 = if flags.len() > 0 {
                flags[flags.len() - 1]
            } else {
                0
            };

            match bs[i] {
                b'(' => {
                    flag = flag ^ last_operator;
                    flags.push(flag);

                    last_operator = 0;
                }
                b')' => {
                    flags.pop();
                }
                b'+' => last_operator = 0,
                b'-' => last_operator = 1,
                b' ' => {}
                n => {
                    current_num = current_num * 10 + (n - b'0') as i32;

                    if i < bs.len() - 1 && bs[i + 1] >= b'0' && bs[i + 1] <= b'9' {
                        continue;
                    }

                    if last_operator ^ flag == 0 {
                        res += current_num as i32;
                    } else {
                        res -= current_num as i32;
                    }
                    current_num = 0;
                }
            }
        }

        res
    }
}

fn main() {
    /* 52 summary_ranges */
    let s = "[".to_string();
    let ret = Solution::is_valid(s);
    println!("{:?}", ret);

    /* 53 simplify_path */
    let s = "/home/".to_string();
    let ret = Solution::simplify_path(s);
    println!("{:?}", ret);

    /* 54 MinStack */
    let mut min_stack = MinStack::new();
    min_stack.push(-2);
    min_stack.push(0);
    min_stack.push(-1);
    min_stack.push(-1);
    println!("{:?}", min_stack.get_min());

    /* 55  eval_rpn  */
    let tokens = vec![
        "2".to_string(),
        "1".to_string(),
        "+".to_string(),
        "3".to_string(),
        "*".to_string(),
    ];
    let ret = Solution::eval_rpn(tokens);
    println!("{:?}", ret);

    /* 56 calculate */
    let s = "1 + 1".to_string();
    let ret = Solution::calculate(s);
    println!("{:?}", ret);
}
