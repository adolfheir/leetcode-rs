use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::result;

/* 链表 */
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }

    // 在链表末尾添加一个新节点
    fn append(&mut self, value: i32) {
        let mut current = self;
        while let Some(ref mut next_node) = current.next {
            current = next_node;
        }
        current.next = Some(Box::new(ListNode::new(value)))
    }
}

struct Node {
    key: i32,
    value: i32,
    prev: Option<Rc<RefCell<Node>>>,
    next: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(key: i32, value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Node {
            key,
            value,
            prev: None,
            next: None,
        }))
    }
}

struct LRUCache {
    capacity: usize,
    dummy: Rc<RefCell<Node>>,
    key_to_node: HashMap<i32, Rc<RefCell<Node>>>,
}

impl LRUCache {
    pub fn new(capacity: i32) -> Self {
        let dummy = Node::new(0, 0);
        dummy.borrow_mut().prev = Some(dummy.clone());
        dummy.borrow_mut().next = Some(dummy.clone());
        LRUCache {
            capacity: capacity as usize,
            dummy,
            key_to_node: HashMap::new(),
        }
    }

    pub fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.key_to_node.get(&key) {
            // 有这本书
            let node = node.clone();
            let value = node.borrow().value;
            self.remove(node.clone()); // 把这本书抽出来
            self.push_front(node); // 放在最上面
            return value;
        }
        -1 // 没有这本书
    }

    pub fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.key_to_node.get(&key) {
            // 有这本书
            let node = node.clone();
            node.borrow_mut().value = value; // 更新 value
            self.remove(node.clone()); // 把这本书抽出来
            self.push_front(node); // 放在最上面
            return;
        }
        let node = Node::new(key, value); // 新书
        self.key_to_node.insert(key, node.clone());
        self.push_front(node); // 放在最上面
        if self.key_to_node.len() > self.capacity {
            // 书太多了
            let back_node = self.dummy.borrow().prev.clone().unwrap();
            self.key_to_node.remove(&back_node.borrow().key);
            self.remove(back_node); // 去掉最后一本书
        }
    }

    // 删除一个节点（抽出一本书）
    fn remove(&mut self, x: Rc<RefCell<Node>>) {
        let prev = x.borrow().prev.clone().unwrap();
        let next = x.borrow().next.clone().unwrap();
        prev.borrow_mut().next = Some(next.clone());
        next.borrow_mut().prev = Some(prev);
    }

    // 在链表头添加一个节点（把一本书放在最上面）
    fn push_front(&mut self, x: Rc<RefCell<Node>>) {
        let next = self.dummy.borrow().next.clone();
        x.borrow_mut().prev = Some(self.dummy.clone());
        x.borrow_mut().next = next.clone();
        self.dummy.borrow_mut().next = Some(x.clone());
        next.unwrap().borrow_mut().prev = Some(x);
    }
}

struct Solution;
impl Solution {
    pub fn has_cycle(head: Option<Box<ListNode>>) -> bool {
        true
    }

    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;
        let mut dummy = ListNode::new(0);

        let mut cur = &mut dummy;
        let mut carry = 0; // 进位
        while l1.is_some() || l2.is_some() || carry > 0 {
            let mut sum = carry;

            if let Some(l1_node) = l1 {
                sum += l1_node.val;
                l1 = l1_node.next;
            }

            if let Some(l2_node) = l2 {
                sum += l2_node.val;
                l2 = l2_node.next;
            }

            carry = sum / 10;
            cur.next = Some(Box::new(ListNode::new(sum % 10)));
            cur = cur.next.as_mut().unwrap()
        }

        dummy.next
    }

    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (None, r) => r,
            (l, None) => l,
            (Some(mut l), Some(mut r)) => {
                if l.val <= r.val {
                    l.next = Solution::merge_two_lists(l.next, Some(r));
                    Some(l)
                } else {
                    r.next = Solution::merge_two_lists(Some(l), r.next);
                    Some(r)
                }
            }
        }
    }

    pub fn copyRandomList(head: Option<Box<ListNode>>) -> bool {
        true
    }

    pub fn reverse_between(
        head: Option<Box<ListNode>>,
        left: i32,
        right: i32,
    ) -> Option<Box<ListNode>> {
        let mut dummy = ListNode::new(0);
        dummy.next = head;
        let mut ptr = &mut dummy;

        for _ in 1..left {
            ptr = ptr.next.as_mut().unwrap();
        }

        if let Some(mut reversed_sublist) = ptr.next.take() {
            let mut tail = reversed_sublist.next.take();

            for _ in left..right {
                let mut node = tail.unwrap();
                tail = node.next.replace(reversed_sublist);
                reversed_sublist = node;
            }

            ptr.next = Some(reversed_sublist);

            for _ in left..=right {
                ptr = ptr.next.as_mut().unwrap();
            }

            ptr.next = tail;
        }

        dummy.next
    }

    // 反转一次，返回反转后的head和remain
    // 如果为最后一次不足以反转，remain为None
    fn reverse_one(
        head: Option<Box<ListNode>>,
        k: i32,
    ) -> (Option<Box<ListNode>>, Option<Box<ListNode>>) {
        let mut pre = head.as_ref();
        for _ in 0..k {
            if pre.is_none() {
                return (head, None);
            }
            pre = pre.unwrap().next.as_ref();
        }

        let mut remain = head;
        let mut dummy = ListNode::new(0);
        for _ in 0..k {
            if let Some(mut n) = remain {
                remain = n.next.take();
                n.next = dummy.next.take();
                dummy.next = Some(n);
            }
        }

        (dummy.next, remain)
    }

    pub fn reverse_k_group(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        let mut remain = head;
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;
        while remain.is_some() {
            let (new_head, new_remain) = Solution::reverse_one(remain, k);
            remain = new_remain;
            tail.next = new_head;
            while tail.next.as_ref().is_some() {
                tail = tail.next.as_mut().unwrap();
            }
        }

        dummy.next
    }

    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut dummy = Some(Box::new(ListNode { val: 0, next: head }));
        let mut slow_p = &mut dummy;
        let mut fast_p = &slow_p.clone();

        for _ in 0..=n {
            if let Some(fast_node) = fast_p {
                fast_p = &fast_node.next;
            } else {
                return None;
            }
        }

        while fast_p.is_some() {
            fast_p = &fast_p.as_ref().unwrap().next;
            slow_p = &mut slow_p.as_mut().unwrap().next;
        }

        let remove_p = &mut slow_p.as_mut().unwrap().next;
        slow_p.as_mut().unwrap().next = remove_p.as_mut().unwrap().next.take();

        dummy.unwrap().next
    }

    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        //如果有重复的，则都要删除，采用弹出，加入方法
        // 每一次必须知道本次的情况下，才能加入上一个。
        let mut res = Some(Box::new(ListNode::new(0)));
        let mut p = res.as_mut().unwrap();
        let mut pre = 101;
        while let Some(mut node) = head {
            head = node.next.take();
            //如果当前访问的值与下一个值相等或与上一个值相等，则当前值不加进去。
            if (head.is_some() && head.as_ref().unwrap().val == node.val) || node.val == pre {
                pre = node.val;
            } else {
                pre = node.val;
                p.next = Some(node);
                p = p.next.as_mut().unwrap();
            }
        }
        res.as_mut().unwrap().next.take()
    }

    pub fn rotate_right(head: Option<Box<ListNode>>, k: i32) -> Option<Box<ListNode>> {
        if head.is_none() || k == 0 {
            return head;
        }
        let mut head = head;
        let mut ptr = &head;
        let mut len = 0;
        while let Some(ref t) = ptr {
            ptr = &t.next;
            len += 1;
        }
        let k = k % len;
        if k == 0 {
            return head;
        }
        let mut ptr = &mut head;
        for _ in 1..len - k {
            ptr = &mut ptr.as_mut().unwrap().next;
        }
        let mut new_head = ptr.as_mut().unwrap().next.take();
        let mut tail = &mut new_head;
        while tail.is_some() && tail.as_ref().unwrap().next.is_some() {
            tail = &mut tail.as_mut().unwrap().next;
        }
        tail.as_mut().unwrap().next = head;
        new_head
    }

    pub fn partition(head: Option<Box<ListNode>>, x: i32) -> Option<Box<ListNode>> {
        if head.is_none() || head.as_ref().unwrap().next.is_none() {
            return head;
        }
        let mut head = head;
        let mut small = ListNode::new(0);
        let mut large = ListNode::new(0);
        let (mut s, mut l) = (&mut small, &mut large);
        while let Some(mut node) = head {
            head = node.next.take();
            match x {
                x if x <= node.val => {
                    l.next = Some(node);
                    l = l.next.as_mut().unwrap().as_mut();
                }
                _ => {
                    s.next = Some(node);
                    s = s.next.as_mut().unwrap().as_mut();
                }
            }
        }
        s.next = large.next;
        small.next
    }
}

fn main() {
    /* 57 has_cycle */
    let mut head = ListNode::new(3);
    head.append(2);
    head.append(0);
    head.append(-4);
    let input = Some(Box::new(head));
    println!("{:?}", Solution::has_cycle(input));

    /* 58 add_two_numbers */
    let mut l1 = ListNode::new(2);
    l1.append(4);
    let mut l2 = ListNode::new(5);
    l2.append(2);

    let l1 = Some(Box::new(l1));
    let l2 = Some(Box::new(l2));

    let result = Solution::add_two_numbers(l1, l2);
    println!("{:?}", result);

    /* 59 merge_two_lists */
    let mut l1 = ListNode::new(2);
    l1.append(4);
    let mut l2 = ListNode::new(5);
    l2.append(2);

    let l1 = Some(Box::new(l1));
    let l2 = Some(Box::new(l2));

    let result = Solution::merge_two_lists(l1, l2);
    println!("{:?}", result);

    /* 60 */

    /* 61 reverse_between */
    let mut l1 = ListNode::new(1);
    l1.append(2);
    l1.append(3);
    l1.append(4);
    l1.append(5);
    l1.append(6);
    let l1 = Some(Box::new(l1));

    let result = Solution::reverse_between(l1, 2, 4);
    println!("reverse_between:");
    println!(" {:?}", result);

    /* 62  reverse_k_group*/
    let mut l1 = ListNode::new(1);
    l1.append(2);

    let l1 = Some(Box::new(l1));
    let result = Solution::reverse_k_group(l1, 2);
    println!("reverse_k_group:");
    println!(" {:?}", result);

    /* 63  remove*/
    let mut l1 = ListNode::new(1);
    l1.append(2);

    let l1 = Some(Box::new(l1));
    let result = Solution::remove_nth_from_end(l1, 1);
    println!("remove_nth_from_end:");
    println!(" {:?}", result);

    /* 64  delete_duplicates */
    let mut l1 = ListNode::new(1);
    l1.append(1);

    let l1 = Some(Box::new(l1));
    let result = Solution::delete_duplicates(l1);
    println!("delete_duplicates:");
    println!(" {:?}", result);

    /* 65  rotate_right */
    let mut l1 = ListNode::new(1);
    l1.append(2);

    let l1 = Some(Box::new(l1));
    let result = Solution::rotate_right(l1, 2);
    println!("rotate_right:");
    println!(" {:?}", result);

    /* 66  partition*/
    let mut l1 = ListNode::new(1);
    l1.append(2);

    let l1 = Some(Box::new(l1));
    let result = Solution::partition(l1, 2);
    println!("partition:");
    println!(" {:?}", result);

    /* 67  lru */
    let mut cache = LRUCache::new(100);
}
