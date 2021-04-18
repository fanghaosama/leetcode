/*
 * @lc app=leetcode.cn id=208 lang=rust
 *
 * [208] 实现 Trie (前缀树)
 *
 * https://leetcode-cn.com/problems/implement-trie-prefix-tree/description/
 *
 * algorithms
 * Medium (70.05%)
 * Likes:    738
 * Dislikes: 0
 * Total Accepted:    113.7K
 * Total Submissions: 158.9K
 * Testcase Example:  '["Trie","insert","search","search","startsWith","insert","search"]\n' +
  '[[],["apple"],["apple"],["app"],["app"],["app"],["app"]]'
 *
 * Trie（发音类似 "try"）或者说 前缀树
 * 是一种树形数据结构，用于高效地存储和检索字符串数据集中的键。这一数据结构有相当多的应用情景，例如自动补完和拼写检查。
 * 
 * 请你实现 Trie 类：
 * 
 * 
 * Trie() 初始化前缀树对象。
 * void insert(String word) 向前缀树中插入字符串 word 。
 * boolean search(String word) 如果字符串 word 在前缀树中，返回 true（即，在检索之前已经插入）；否则，返回
 * false 。
 * boolean startsWith(String prefix) 如果之前已经插入的字符串 word 的前缀之一为 prefix ，返回 true
 * ；否则，返回 false 。
 * 
 * 
 * 
 * 
 * 示例：
 * 
 * 
 * 输入
 * ["Trie", "insert", "search", "search", "startsWith", "insert", "search"]
 * [[], ["apple"], ["apple"], ["app"], ["app"], ["app"], ["app"]]
 * 输出
 * [null, null, true, false, true, null, true]
 * 
 * 解释
 * Trie trie = new Trie();
 * trie.insert("apple");
 * trie.search("apple");   // 返回 True
 * trie.search("app");     // 返回 False
 * trie.startsWith("app"); // 返回 True
 * trie.insert("app");
 * trie.search("app");     // 返回 True
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * word 和 prefix 仅由小写英文字母组成
 * insert、search 和 startsWith 调用次数 总计 不超过 3 * 10^4 次
 * 
 * 
 */

// @lc code=start
use std::collections::HashMap;
struct Trie {
    isWord: bool,
    children: HashMap<char, Trie>,
}


/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl Trie {

    /** Initialize your data structure here. */
    fn new() -> Self {
        Trie {
            isWord: false,
            children: HashMap::new(),
        }
    }
    
    /** Inserts a word into the trie. */
    fn insert(&mut self, word: String) {
        let b: Vec<char> = word.chars().collect();
        let mut cur = &mut self.children;
        for i in (0..b.len()) {
            let mut chi = cur.get_mut(&b[i]);
            match chi {
                None => {
                    let mut t = Trie::new();
                    if i == b.len() - 1 {
                        t.isWord = true;
                    }
                    cur.insert(b[i], t);
                },
                Some(cc) => {
                    if i == b.len() - 1 {
                        cc.isWord = true;
                    }
                }
            }
            cur = &mut cur.get_mut(&b[i]).unwrap().children;
        }

    }
    
    /** Returns if the word is in the trie. */
    fn search(&mut self, word: String) -> bool {
        let b: Vec<char> = word.chars().collect();
        let mut cur = &mut self.children;
        for i in (0..b.len()) {
            let mut chi = cur.get_mut(&b[i]);
            match chi {
                None => {
                    return false;
                },
                Some(cc) => {
                    if i == b.len() - 1 {
                        return cc.isWord;
                    } 
                    cur = &mut cc.children;
                }
            }
        }
        false
    }
    
    /** Returns if there is any word in the trie that starts with the given prefix. */
    fn starts_with(&mut self, prefix: String) -> bool {
        let b: Vec<char> = prefix.chars().collect();
        let mut cur = &mut self.children;
        for i in (0..b.len()) {
            let mut chi = cur.get_mut(&b[i]);
            match chi {
                None => {
                    return false;
                },
                Some(cc) => {
                    if i == b.len() - 1 {
                        return true;
                    } 
                    cur = &mut cc.children;
                }
            }
        }
        false
    }
}

/**
 * Your Trie object will be instantiated and called as such:
 * let obj = Trie::new();
 * obj.insert(word);
 * let ret_2: bool = obj.search(word);
 * let ret_3: bool = obj.starts_with(prefix);
 */
// @lc code=end

