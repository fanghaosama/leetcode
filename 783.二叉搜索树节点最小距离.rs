/*
 * @lc app=leetcode.cn id=783 lang=rust
 *
 * [783] 二叉搜索树节点最小距离
 *
 * https://leetcode-cn.com/problems/minimum-distance-between-bst-nodes/description/
 *
 * algorithms
 * Easy (56.26%)
 * Likes:    184
 * Dislikes: 0
 * Total Accepted:    63.7K
 * Total Submissions: 107.3K
 * Testcase Example:  '[4,2,6,1,3]'
 *
 * 给你一个二叉搜索树的根节点 root ，返回 树中任意两不同节点值之间的最小差值 。
 * 
 * 注意：本题与
 * 530：https://leetcode-cn.com/problems/minimum-absolute-difference-in-bst/
 * 相同
 * 
 * 
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：root = [4,2,6,1,3]
 * 输出：1
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：root = [1,0,48,null,null,12,49]
 * 输出：1
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 树中节点数目在范围 [2, 100] 内
 * 0 
 * 差值是一个正数，其数值等于两值之差的绝对值
 * 
 * 
 * 
 * 
 */

// @lc code=start
// Definition for a binary tree node.
// #[derive(Debug, PartialEq, Eq)]
// pub struct TreeNode {
//   pub val: i32,
//   pub left: Option<Rc<RefCell<TreeNode>>>,
//   pub right: Option<Rc<RefCell<TreeNode>>>,
// }
//
// impl TreeNode {
//   #[inline]
//   pub fn new(val: i32) -> Self {
//     TreeNode {
//       val,
//       left: None,
//       right: None
//     }
//   }
// }
use std::rc::Rc;
use std::cell::RefCell;
impl Solution {
    pub fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
        let (s, l, m) = Solution::get_min(root.as_ref());
        m.unwrap()
    }
    pub fn get_min(node: Option<&Rc<RefCell<TreeNode>>>) -> (Option<i32>, Option<i32>, Option<i32>) {
        if node == None {
            return (None, None, None);
        }
        let (lefts, leftl, m) = Solution::get_min(node.unwrap().borrow().left.as_ref());
        let (rights, rightl, n) = Solution::get_min(node.unwrap().borrow().right.as_ref());
        let mut min: Option<i32> = None;
        let val = node.unwrap().borrow().val;
        if leftl != None {
            if min == None || val - leftl.unwrap() < min.unwrap() {
                min = Some(val - leftl.unwrap());
            }
            if m != None && m.unwrap() < min.unwrap() {
                min = m;
            }
        }
        if rights != None {
            if min == None || rights.unwrap() - val < min.unwrap() {
                min = Some(rights.unwrap() - val);
            }
            if n != None && n.unwrap() < min.unwrap() {
                min = n;
            }
        }
        let s = if lefts != None { lefts } else { Some(val) };
        let l = if rightl != None { rightl } else { Some(val) };
        (s, l, min)
    }
}
// @lc code=end

