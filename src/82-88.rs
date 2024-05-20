//二叉层次遍历（广度优先） & 二叉搜索树

use std::cell::RefCell;
use std::collections::HashMap;
use std::collections::VecDeque;
use std::fmt;
use std::rc::Rc;
use std::result;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
    pub val: i32,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        TreeNode {
            val,
            left: None,
            right: None,
        }
    }
}

struct Solution;
impl Solution {
    pub fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
        match root {
            None => Vec::new(),
            Some(root_rc) => {
                let mut queue = VecDeque::new();
                queue.push_back(root_rc);
                let mut ans = vec![];
                while !queue.is_empty() {
                    let mut most_right = 0;
                    let layer_len = queue.len();
                    for _ in 0..layer_len {
                        let node_rc = queue.pop_front().unwrap();
                        let node = node_rc.borrow();
                        most_right = node.val;
                        if let Some(left) = node.left.as_ref() {
                            queue.push_back(Rc::clone(left));
                        }
                        if let Some(right) = node.right.as_ref() {
                            queue.push_back(Rc::clone(right));
                        }
                    }

                    ans.push(most_right);
                }
                ans
            }
        }
    }

    pub fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
        let mut ans = vec![];
        let mut q = VecDeque::new();
        if let Some(node) = root {
            q.push_back(node);
        } else {
            return ans;
        }
        while !q.is_empty() {
            let mut sum = 0f64;
            let len = q.len();
            for _ in 0..len {
                let node = q.pop_front().unwrap();
                if let Some(left) = node.borrow_mut().left.take() {
                    q.push_back(left);
                }
                if let Some(right) = node.borrow_mut().right.take() {
                    q.push_back(right);
                }
                sum += node.borrow().val as f64;
            }
            ans.push(sum / len as f64);
        }
        ans
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = Vec::new();
        let mut cur = Vec::new();
        if let Some(x) = root {
            cur.push(x);
        }
        while !cur.is_empty() {
            let mut nxt = Vec::new();
            let mut vals = Vec::with_capacity(cur.len()); // 预分配空间
            for node in cur {
                let mut x = node.borrow_mut();
                vals.push(x.val);
                if let Some(left) = x.left.take() {
                    nxt.push(left);
                }
                if let Some(right) = x.right.take() {
                    nxt.push(right);
                }
            }
            cur = nxt;
            ans.push(vals);
        }
        ans
    }

    pub fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        let mut ans = vec![];
        let mut q = vec![];
        let mut turn = false;
        if let Some(r) = root {
            q.push(r);
            while !q.is_empty() {
                let mut t = vec![];
                let mut res = vec![];
                for i in 0..q.len() {
                    res.push(q[i].borrow().val);
                    if let Some(left) = q[i].borrow_mut().left.take() {
                        t.push(left);
                    }
                    if let Some(right) = q[i].borrow_mut().right.take() {
                        t.push(right);
                    }
                }
                if turn {
                    res.reverse();
                }
                turn = !turn;
                q = t;
                ans.push(res);
            }
        }
        ans
    }

    pub fn get_minimum_difference(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn in_order(node: Option<Rc<RefCell<TreeNode>>>, nums: &mut Vec<i32>) {
            if let Some(node) = node {
                in_order(node.borrow_mut().left.take(), nums);
                nums.push(node.borrow().val);
                in_order(node.borrow_mut().right.take(), nums);
            }
        }
        let mut nums = Vec::new();
        in_order(root, &mut nums);
        (1..nums.len())
            .map(|x| nums[x] - nums[x - 1])
            .min()
            .unwrap_or(0)
    }

    pub fn kth_smallest(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
        fn sub(
            root: Option<Rc<RefCell<TreeNode>>>,
            count: usize,
            target: usize,
        ) -> Result<usize, i32> {
            if let Some(root) = root {
                let count = sub(root.borrow().left.clone(), count, target)?;
                if count + 1 == target {
                    return Err(root.borrow().val);
                }
                sub(root.borrow().right.clone(), count + 1, target)
            } else {
                Ok(count)
            }
        }

        sub(root, 0, k as usize).unwrap_err()
    }

    pub fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        fn recv(root: Option<Rc<RefCell<TreeNode>>>, pre: &mut [i64; 1]) -> bool {
            if let Some(node) = root {
                if !recv(node.borrow_mut().left.take(), pre) {
                    return false;
                }
                if node.borrow().val as i64 <= pre[0] {
                    return false;
                } else {
                    pre[0] = node.borrow().val as i64;
                }
                recv(node.borrow_mut().right.take(), pre)
            } else {
                true
            }
        }
        let mut pre = [i64::MIN];
        recv(root, &mut pre)
    }
}

fn main() {
    /* 82 right_side_view */
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));
    let ans = Solution::right_side_view(Some(tree));
    println!("ans: {:?}", ans);

    /* 83. average_of_levels */
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));
    let ans = Solution::average_of_levels(Some(tree));
    println!("ans: {:?}", ans);

    /* 84. level_order */
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));
    let ans = Solution::level_order(Some(tree));
    println!("ans: {:?}", ans);

    /* 85.zigzag_level_order */
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));
    let ans = Solution::zigzag_level_order(Some(tree));
    println!("ans: {:?}", ans);

    /* 86. get_minimum_difference*/
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));
    let ans = Solution::get_minimum_difference(Some(tree));
    println!("get_minimum_difference ans: {:?}", ans);

    /* 87.kth_smallest  */
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));

    let ans = Solution::kth_smallest(Some(tree), 1);
    println!("kth_smallest ans: {:?}", ans);

    /* 88. */
    let tree = Rc::new(RefCell::new(TreeNode {
        val: 3,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 9,
            left: None,
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 20,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 15,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    }));

    let ans = Solution::is_valid_bst(Some(tree));
    println!("is_valid_bst ans: {:?}", ans);
    
}
