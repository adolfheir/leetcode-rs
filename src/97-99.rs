/* 前缀树 */
#[derive(Default)]
struct Trie([Option<Box<Trie>>; 26], bool);

impl Trie {
    fn new() -> Self {
        Default::default()
    }
    fn insert(&mut self, word: String) {
        println!("word: {:?}", word);
        word.into_bytes()
            .into_iter()
            .map(|c| (c - b'a') as usize)
            .fold(self, |t, i| t.0[i].get_or_insert_with(Default::default))
            .1 = true
    }

    fn find(&self, prefix: String) -> (bool, bool) {
        let mut t = self;
        for i in prefix.into_bytes().into_iter().map(|c| (c - b'a') as usize) {
            match &t.0[i] {
                Some(s) => t = &s,
                _ => return (false, false),
            }
        }
        (true, t.1)
    }

    fn search(&self, word: String) -> bool {
        self.find(word).1
    }

    fn starts_with(&self, prefix: String) -> bool {
        self.find(prefix).0
    }
}

/*  */
struct DictTree {
    is_end: bool,
    son: Vec<Option<DictTree>>,
}

impl DictTree {
    fn new(is_end: bool) -> Self {
        let mut son = Vec::with_capacity(26);
        (0..26).for_each(|_| son.push(None));
        Self { is_end, son }
    }
}

/*  */
struct WordDictionary {
    tab: DictTree,
}

impl WordDictionary {
    fn new() -> Self {
        Self {
            tab: DictTree::new(false),
        }
    }

    fn add_word(&mut self, word: String) {
        fn dfs(dt: &mut DictTree, s: &[u8], i: usize) {
            if i < s.len() {
                let c: usize = s[i] as usize - b'a' as usize;
                let t = if let Some(t) = dt.son[c].as_mut() {
                    t
                } else {
                    dt.son[c] = Some(DictTree::new(false));
                    dt.son[c].as_mut().unwrap()
                };
                dfs(t, s, i + 1)
            } else {
                dt.is_end = true
            }
        }
        dfs(&mut self.tab, &mut word.as_bytes(), 0);
    }

    fn search(&self, word: String) -> bool {
        fn dfs(dt: &DictTree, s: &[u8], i: usize) -> bool {
            if i < s.len() {
                if s[i] == b'.' {
                    for son in dt.son.iter() {
                        if let Some(t) = son {
                            if dfs(t, s, i + 1) {
                                return true;
                            }
                        }
                    }
                } else {
                    let c = s[i] as usize - b'a' as usize;
                    if let Some(t) = dt.son[c].as_ref() {
                        return dfs(t, s, i + 1);
                    }
                }
                false
            } else {
                if dt.is_end {
                    true
                } else {
                    false
                }
            }
        }
        dfs(&self.tab, &word.as_bytes(), 0)
    }
}

#[derive(Default, Debug)]
struct Trie2 {
    is_end: bool,
    childs: [Option<Box<Trie2>>; 26],
}

impl Trie2 {
    fn new() -> Self {
        Default::default()
    }
    fn insert(&mut self, word: String) {
        let mut cur = self;
        for idx in word.bytes().map(|b| (b - b'a') as usize) {
            cur = cur.childs[idx]
                .get_or_insert(Box::new(Trie2::new()))
                .as_mut();
        }
        cur.is_end = true;
    }
}

struct Solution {}

impl Solution {
    fn find_words(board: Vec<Vec<char>>, words: Vec<String>) -> Vec<String> {
        fn dfs(
            board: &Vec<Vec<char>>,
            x: usize,
            y: usize,
            mut cur: &mut Trie2,
            s: &mut String,
            result: &mut Vec<String>,
            flags: &mut Vec<Vec<bool>>,
        ) {
            let idx = (board[x][y] as u8 - b'a') as usize;
            if cur.childs[idx].is_none() {
                return;
            }

            cur = cur.childs[idx].as_mut().unwrap().as_mut();
            flags[x][y] = true;
            s.push(board[x][y]);
            if cur.is_end {
                result.push(s.clone());
                cur.is_end = false;
            }

            if x > 0 && !flags[x - 1][y] {
                dfs(board, x - 1, y, cur, s, result, flags);
            }
            if x + 1 < board.len() && !flags[x + 1][y] {
                dfs(board, x + 1, y, cur, s, result, flags);
            }
            if y > 0 && !flags[x][y - 1] {
                dfs(board, x, y - 1, cur, s, result, flags);
            }
            if y + 1 < board[0].len() && !flags[x][y + 1] {
                dfs(board, x, y + 1, cur, s, result, flags);
            }

            flags[x][y] = false;
            s.pop();
        }

        let mut root = Trie2::new();

        // 1. insert
        for word in words.into_iter() {
            root.insert(word);
        }

        // 2. search
        let (m, n) = (board.len(), board[0].len());
        let mut s = String::new();
        let mut result = vec![];
        let mut flags = vec![vec![false; n]; m];

        for i in 0..m {
            for j in 0..n {
                dfs(&board, i, j, &mut root, &mut s, &mut result, &mut flags);
            }
        }

        // 3.
        result
    }
}

fn main() {
    /* 97. Trie */
    let mut trie = Trie::new();
    trie.insert("aasd".to_string());
    trie.insert("aasdf".to_string());
    trie.insert("aasdk".to_string());
    println!(
        "Trie find:{:?},starts_with:{:?}, seearch:{:?},",
        trie.find("aasd".to_string()),
        trie.starts_with("aasd".to_string()),
        trie.search("aasd".to_string())
    );

    /* 98 WordDictionary */
    let mut word_dictionary = WordDictionary::new();
    word_dictionary.add_word("WordDictionary".to_string());
    word_dictionary.add_word("addWord".to_string());
    word_dictionary.add_word("addWord".to_string());
    word_dictionary.add_word("addWord".to_string());
    word_dictionary.add_word("search".to_string());
    word_dictionary.add_word("search".to_string());
    word_dictionary.add_word("search".to_string());

    print!(
        "WordDictionary 
        find_1:{:?},
        find_2:{:?},
        find_3:{:?},
        find_4:{:?},
        find_5:{:?},
        find_6:{:?},
        find_7:{:?},
        find_8:{:?},
        ",
        word_dictionary.search("".to_string()),
        word_dictionary.search("bad".to_string()),
        word_dictionary.search("dad".to_string()),
        word_dictionary.search("mad".to_string()),
        word_dictionary.search("pad".to_string()),
        word_dictionary.search("bad".to_string()),
        word_dictionary.search(".ad".to_string()),
        word_dictionary.search("b..".to_string())
    );

    /* 99 find_words */
    let board = vec![
        vec!['o', 'a', 'a', 'n'],
        vec!['e', 't', 'a', 'e'],
        vec!['i', 'h', 'k', 'r'],
        vec!['i', 'f', 'l', 'v'],
    ];
    let words = vec![
        "oath".to_string(),
        "pea".to_string(),
        "eat".to_string(),
        "rain".to_string(),
    ];

    let ret = Solution::find_words(board, words);
    println!("Solution find_words{:?}", ret);
}
