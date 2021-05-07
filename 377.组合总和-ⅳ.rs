/*
 * @lc app=leetcode.cn id=377 lang=rust
 *
 * [377] 组合总和 Ⅳ
 *
 * https://leetcode-cn.com/problems/combination-sum-iv/description/
 *
 * algorithms
 * Medium (44.18%)
 * Likes:    343
 * Dislikes: 0
 * Total Accepted:    31.3K
 * Total Submissions: 68.2K
 * Testcase Example:  '[1,2,3]\n4'
 *
 * 给你一个由 不同 整数组成的数组 nums ，和一个目标整数 target 。请你从 nums 中找出并返回总和为 target 的元素组合的个数。
 * 
 * 题目数据保证答案符合 32 位整数范围。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3], target = 4
 * 输出：7
 * 解释：
 * 所有可能的组合为：
 * (1, 1, 1, 1)
 * (1, 1, 2)
 * (1, 2, 1)
 * (1, 3)
 * (2, 1, 1)
 * (2, 2)
 * (3, 1)
 * 请注意，顺序不同的序列被视作不同的组合。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [9], target = 3
 * 输出：0
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 1 
 * nums 中的所有元素 互不相同
 * 1 
 * 
 * 
 * 
 * 
 * 进阶：如果给定的数组中含有负数会发生什么？问题会产生何种变化？如果允许负数出现，需要向题目中添加哪些限制条件？
 * 
 */

// @lc code=start
use std::collections::HashMap;
impl Solution {
    pub fn combination_sum4(nums: Vec<i32>, target: i32) -> i32 {
        let mut dp: HashMap<i32, i32> = HashMap::new();
        let res = Solution::helper(&nums, target, &mut dp);
        res
    }

    pub fn helper(nums: &Vec<i32>, target: i32, dp: &mut HashMap<i32, i32>) -> i32 {
        let has = dp.get(&target);
        if has != None {
            return *has.unwrap();
        }
        let mut count: i32 = 0;
        for i in nums.iter() {
            if i < &target {
                count += Solution::helper(nums, target - i, dp);
            } else if i == &target {
                count += 1;
            }
        }
        dp.insert(target, count);
        count
    }
}
// @lc code=end

