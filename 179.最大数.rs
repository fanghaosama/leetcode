/*
 * @lc app=leetcode.cn id=179 lang=rust
 *
 * [179] 最大数
 *
 * https://leetcode-cn.com/problems/largest-number/description/
 *
 * algorithms
 * Medium (38.20%)
 * Likes:    644
 * Dislikes: 0
 * Total Accepted:    85.5K
 * Total Submissions: 214.2K
 * Testcase Example:  '[10,2]'
 *
 * 给定一组非负整数 nums，重新排列每个数的顺序（每个数不可拆分）使之组成一个最大的整数。
 * 
 * 注意：输出结果可能非常大，所以你需要返回一个字符串而不是整数。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [10,2]
 * 输出："210"
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [3,30,34,5,9]
 * 输出："9534330"
 * 
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [1]
 * 输出："1"
 * 
 * 
 * 示例 4：
 * 
 * 
 * 输入：nums = [10]
 * 输出："10"
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 0 
 * 
 * 
 */

// @lc code=start
use std::cmp::Ordering;
impl Solution {
    pub fn largest_number(nums: Vec<i32>) -> String {
        let mut numS: Vec<String> = nums.iter().map(|a| a.to_string()).collect();
        numS.sort_by(|a, b| Solution::compare(a, b));
        let mut res: String = String::from("");
        let mut zero = true;
        for i in numS.iter() {
            if i.parse::<i32>().unwrap() > 0 || !zero {
                res += i;
                zero = false;
            }
        }
        if res.len() == 0 {
            return String::from("0");
        }
        res
    }

    pub fn compare(f: &String, s: &String) -> Ordering {
        let fc: Vec<char> = f.chars().collect();
        let sc: Vec<char> = s.chars().collect();
        let cl: usize = fc.len() * sc.len();
        let mut fi: usize = 0;
        let mut si: usize = 0;
        for i in (0..cl) {
            fi = i % fc.len();
            si = i % sc.len();
            if sc[si] > fc[fi] {
                return Ordering::Greater;
            }
            if fc[fi] > sc[si] {
                return Ordering::Less;
            }
            
        }
        Ordering::Equal
    }
}
// @lc code=end

