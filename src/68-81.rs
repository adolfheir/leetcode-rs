/* 二叉树 */

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;
use std::result;

// Definition for a binary tree node.
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

impl fmt::Display for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TreeNode {{ val: {}, left: {:?}, right: {:?} }}",
            self.val, self.left, self.right
        )
    }
}

/* split */
struct BSTIterator {
    stack: Vec<Option<Rc<RefCell<TreeNode>>>>,
}
impl BSTIterator {
    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        BSTIterator { stack: vec![root] }
    }
    fn next(&mut self) -> i32 {
        let mut node = self.stack.pop().unwrap();
        while node.is_some() || !self.stack.is_empty() {
            while let Some(mut cur) = node {
                let left = cur.borrow_mut().left.take();
                self.stack.push(Some(cur));
                node = left;
            }
            if let Some(top) = self.stack.pop() {
                if let Some(mut cur) = top {
                    node = cur.borrow_mut().right.take();
                    if node.is_some() {
                        self.stack.push(node);
                    }
                    return cur.borrow().val;
                }
            }
        }
        return -1;
    }
    fn has_next(&self) -> bool {
        !self.stack.is_empty()
    }
}

/* split */

struct Solution;
impl Solution {
    pub fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        match root {
            Some(root) => {
                let left = root.borrow_mut().left.take();
                let right = root.borrow_mut().right.take();
                1 + Self::max_depth(left).max(Self::max_depth(right))
            }
            None => 0,
        }
    }
    pub fn is_same_tree(
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
        // symmetric: bool,
    ) -> bool {
        match (p, q) {
            (None, None) => true,
            (Some(p), Some(q)) => {
                p.borrow().val == q.borrow().val
                    && Self::is_same_tree(p.borrow().left.clone(), q.borrow().left.clone())
                    && Self::is_same_tree(p.borrow().right.clone(), q.borrow().right.clone())
            }
            _ => false,
        }
    }
    pub fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
        if let Some(r) = root.as_ref() {
            let (left, right) = (r.borrow().left.clone(), r.borrow().right.clone());
            r.borrow_mut().left = Solution::invert_tree(right);
            r.borrow_mut().right = Solution::invert_tree(left);
        }
        root
    }

    pub fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
        pub fn is_same_tree(
            p: Option<Rc<RefCell<TreeNode>>>,
            q: Option<Rc<RefCell<TreeNode>>>,
            // symmetric: bool,
        ) -> bool {
            match (p, q) {
                (None, None) => true,
                (Some(p), Some(q)) => {
                    p.borrow().val == q.borrow().val
                        && is_same_tree(p.borrow().left.clone(), q.borrow().right.clone())
                        && is_same_tree(p.borrow().right.clone(), q.borrow().left.clone())
                }
                _ => false,
            }
        }
        is_same_tree(
            root.clone().unwrap().borrow().left.clone(),
            root.unwrap().borrow().right.clone(),
        )
    }

    pub fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if preorder.len() == 0 {
            return None;
        }
        let mut root = TreeNode::new(preorder[0]);
        let mut idx = 0;
        for i in 0..inorder.len() {
            if inorder[i] == preorder[0] {
                idx = i;
                break;
            }
        }
        let ml = inorder[0..idx].len() + 1;
        let pl = preorder.len();
        let il = inorder.len();
        root.left = Solution::build_tree(preorder[1..ml].to_vec(), inorder[0..idx].to_vec());
        root.right =
            Solution::build_tree(preorder[ml..pl].to_vec(), inorder[(idx + 1)..il].to_vec());
        return Some(Rc::new(RefCell::new(root)));
    }

    pub fn build_tree2(inorder: Vec<i32>, postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
        if postorder.is_empty() || postorder.len() != inorder.len() {
            return None;
        }
        let mut val = postorder[postorder.len() - 1];
        let mut node = TreeNode::new(val);
        let mut p: usize = 0;
        for (_, &v) in inorder.iter().enumerate() {
            if v == val {
                break;
            }
            p += 1;
        }
        node.left = Solution::build_tree(
            inorder.as_slice()[0..p].to_vec(),
            postorder.as_slice()[0..p].to_vec(),
        );
        node.right = Solution::build_tree(
            inorder.as_slice()[p + 1..].to_vec(),
            postorder.as_slice()[p..postorder.len() - 1].to_vec(),
        );
        return Some(Rc::new(RefCell::new(node)));
    }

    pub fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
        //层序遍历
        let mut ans = Vec::new();
        let mut stack = Vec::new();
        if root.is_none() {
            return ans;
        }
        stack.push(root.unwrap());
        while stack.is_empty() != true {
            let num = stack.len();
            let mut level = Vec::new();
            for _i in 0..num {
                let tmp = stack.remove(0);
                level.push(tmp.borrow_mut().val);
                if tmp.borrow_mut().left.is_some() {
                    stack.push(tmp.borrow_mut().left.take().unwrap());
                }
                if tmp.borrow_mut().right.is_some() {
                    stack.push(tmp.borrow_mut().right.take().unwrap());
                }
            }
            ans.push(level);
        }
        ans
    }

    pub fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) -> &mut Option<Rc<RefCell<TreeNode>>> {
        if let Some(root) = root {
            // 1.
            let mut left = root.borrow_mut().left.take();
            Self::flatten(&mut left);

            let mut right = root.borrow_mut().right.take();
            Self::flatten(&mut right);

            // 2.
            root.borrow_mut().right = left;
            let mut cur = root.clone();

            while cur.borrow().right.is_some() {
                let next = cur.borrow().right.clone().unwrap();
                cur = next;
            }

            // 3.
            cur.borrow_mut().right = right;
        }

        return root;
    }

    pub fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
        if let Some(node) = root {
            let mut node = node.borrow_mut();
            let target_sum = target_sum - node.val;
            if node.left.is_none() && node.right.is_none() {
                // root 是叶子
                return target_sum == 0;
            }
            return Self::has_path_sum(node.left.take(), target_sum)
                || Self::has_path_sum(node.right.take(), target_sum);
        }
        false
    }

    pub fn sum_numbers(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn dfs(node: Option<&Rc<RefCell<TreeNode>>>, x: i32, ans: &mut i32) {
            if let Some(node) = node {
                let node = node.borrow();
                let x = x * 10 + node.val;
                if node.left.is_none() && node.right.is_none() {
                    // node 是叶子节点
                    *ans += x;
                    return;
                }
                dfs(node.left.as_ref(), x, ans);
                dfs(node.right.as_ref(), x, ans);
            }
        }
        let mut ans = 0;
        dfs(root.as_ref(), 0, &mut ans);
        ans
    }

    pub fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        fn helper(root: Option<Rc<RefCell<TreeNode>>>, res: &mut i32) -> i32 {
            if let Some(mut node) = root {
                let zero = 0;
                let left = zero.max(helper(node.borrow_mut().left.take(), res));
                let right = zero.max(helper(node.borrow_mut().right.take(), res));
                let ans = *res;
                let ans = ans.max(left + right + node.borrow().val);
                if ans > *res {
                    *res = ans;
                }
                return left.max(right) + node.borrow().val;
            }
            0
        }
        let mut ans = i32::MIN;
        helper(root, &mut ans);
        ans
    }

    pub fn count_height(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(node) = root {
            1 + Self::count_height(node.borrow().left.clone())
        } else {
            0
        }
    }

    pub fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        if let Some(root) = root {
            let left_height = Self::count_height(root.borrow().left.clone());
            let right_height = Self::count_height(root.borrow().right.clone());
            if left_height == right_height {
                (1 << left_height) + Self::count_nodes(root.borrow().right.clone())
            } else {
                (1 << right_height) + Self::count_nodes(root.borrow().left.clone())
            }
        } else {
            0
        }
    }

    pub fn lowest_common_ancestor(
        root: Option<Rc<RefCell<TreeNode>>>,
        p: Option<Rc<RefCell<TreeNode>>>,
        q: Option<Rc<RefCell<TreeNode>>>,
    ) -> Option<Rc<RefCell<TreeNode>>> {
        if root.is_none() || root == p || root == q {
            return root;
        }
        let x = root.as_ref().unwrap();
        let left = Self::lowest_common_ancestor(x.borrow_mut().left.take(), p.clone(), q.clone());
        let right = Self::lowest_common_ancestor(x.borrow_mut().right.take(), p, q);
        if left.is_some() && right.is_some() {
            return root;
        }
        if left.is_some() {
            left
        } else {
            right
        }
    }
}

fn main() {
    /* 68. max_depth */
    // 构建一棵示例树
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_node = Rc::new(RefCell::new(TreeNode::new(9)));
    let right_node = Rc::new(RefCell::new(TreeNode::new(20)));
    let left_right_node = Rc::new(RefCell::new(TreeNode::new(15)));
    let left_right_right_node = Rc::new(RefCell::new(TreeNode::new(7)));
    root.borrow_mut().left = Some(left_node.clone());
    root.borrow_mut().right = Some(right_node.clone());
    left_node.borrow_mut().right = Some(left_right_node.clone());
    right_node.borrow_mut().right = Some(left_right_right_node.clone());
    // 求解树的最大深度并打印结果
    let depth = Solution::max_depth(Some(root));
    println!("max_depth:{}", depth);

    /* 69. is_same_tree */
    let root = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    let root2 = Some(Rc::new(RefCell::new(TreeNode::new(1))));
    // assert_eq!(Solution::is_same_tree(root, root2), true);
    println!("is_same_tree:{}", Solution::is_same_tree(root, root2));

    /* 70. invert_tree */
    let root = Rc::new(RefCell::new(TreeNode::new(3)));
    let left_node = Rc::new(RefCell::new(TreeNode::new(9)));
    let right_node = Rc::new(RefCell::new(TreeNode::new(20)));
    let left_right_node = Rc::new(RefCell::new(TreeNode::new(15)));
    let left_right_right_node = Rc::new(RefCell::new(TreeNode::new(7)));
    root.borrow_mut().left = Some(left_node.clone());
    root.borrow_mut().right = Some(right_node.clone());
    left_node.borrow_mut().right = Some(left_right_node.clone());
    right_node.borrow_mut().right = Some(left_right_right_node.clone());
    let ans = Solution::invert_tree(Some(root));

    if let Some(node_ref) = ans {
        let node_borrow = node_ref.borrow();
        let node = &*node_borrow;
        println!("invert_tree: {}", node);
    }

    /* 71. is_symmetric */
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    };
    let tree = Some(Rc::new(RefCell::new(root)));
    // assert_eq!(Solution::is_same_tree(root, root2), true);
    println!("is_symmetric: {}", Solution::is_symmetric(tree));

    /* 72. build_tree */
    let preorder = vec![3, 9, 20, 15, 7];
    let inorder = vec![9, 3, 15, 20, 7];

    let ans = Solution::build_tree(preorder, inorder);

    if let Some(node_ref) = ans {
        let node_borrow = node_ref.borrow();
        let node = &*node_borrow;
        println!("build_tree: {}", node);
    }

    /* 73. build_tree2 */
    let inorder = vec![9, 3, 15, 20, 7];
    let postorder = vec![9, 15, 7, 20, 3];
    let ans = Solution::build_tree2(inorder, postorder);
    if let Some(node_ref) = ans {
        let node_borrow = node_ref.borrow();
        let node = &*node_borrow;
        println!("build_tree2: {}", node);
    }

    /* 74.level_order */
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    };

    let ans = Solution::level_order(Some(Rc::new(RefCell::new(root))));
    println!("level_order: {:?}", ans);

    /* 75 flatten */
    let root = TreeNode {
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 3,
                left: None,
                right: None,
            }))),
        }))),
    };
    let root_rc = &mut Some(Rc::new(RefCell::new(root)));
    let ans = Solution::flatten(root_rc);
    if let Some(node_rc) = ans.take() {
        if let Ok(node_refcell) = Rc::try_unwrap(node_rc) {
            let node_inner = node_refcell.into_inner();
            println!("flatten: {:?}", node_inner);
        }
    }

    /* 76. has_path_sum */
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
        }))),
    };
    let targetSum = 22;
    let ans = Solution::has_path_sum(Some(Rc::new(RefCell::new(root))), targetSum);
    println!("has_path_sum: {:?}", ans);

    /* 77. sum_numbers  */
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
        }))),
    };
    let ans = Solution::sum_numbers(Some(Rc::new(RefCell::new(root))));
    println!("sum_numbers: {:?}", ans);

    /* 78. max_path_sum */
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
        }))),
    };

    let ans = Solution::max_path_sum(Some(Rc::new(RefCell::new(root))));
    println!("max_path_sum: {:?}", ans);

    /* 79. BSTIterator */
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
        }))),
    };
    let tree = Some(Rc::new(RefCell::new(root)));
    let mut bts = BSTIterator::new(tree);

    println!("bts1:{:?}", bts.next());
    println!("bts2:{:?}", bts.next());

    /* 80. */
    let root = TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
        }))),
    };
    let tree = Some(Rc::new(RefCell::new(root)));
    let root = Solution::count_nodes(tree);
    println!("count_nodes:{:?}", root);

    /* 81. 相同父子树 */
    let root = Rc::new(RefCell::new(TreeNode {
        val: 5,
        left: Some(Rc::new(RefCell::new(TreeNode {
            val: 4,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 11,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 7,
                    left: None,
                    right: None,
                }))),
                right: Some(Rc::new(RefCell::new(TreeNode {
                    val: 2,
                    left: None,
                    right: None,
                }))),
            }))),
            right: None,
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode {
            val: 8,
            left: Some(Rc::new(RefCell::new(TreeNode {
                val: 4,
                left: Some(Rc::new(RefCell::new(TreeNode {
                    val: 4,
                    left: Some(Rc::new(RefCell::new(TreeNode {
                        val: 1,
                        left: None,
                        right: None,
                    }))),
                    right: None,
                }))),
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode {
                val: 13,
                left: None,
                right: None,
            }))),
        }))),
    }));

    
}
