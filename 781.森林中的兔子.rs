/*
 * @lc app=leetcode.cn id=781 lang=rust
 *
 * [781] 森林中的兔子
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        let mut rabbits = HashMap::new();
        for element in answers.iter() {
            let count = rabbits.entry(element).or_insert(0);
            *count += 1;
        }
        let mut result = 0;
        for (key, value) in rabbits.iter() {
            let mut count = value / (*key + 1);
            if value % (*key + 1) > 0 {
                count += 1;
            }
            result += count * (*key + 1);
        }
        result
    }
}
// @lc code=end

