/*
 * @lc app=leetcode.cn id=368 lang=rust
 *
 * [368] 最大整除子集
 *
 * https://leetcode-cn.com/problems/largest-divisible-subset/description/
 *
 * algorithms
 * Medium (40.26%)
 * Likes:    309
 * Dislikes: 0
 * Total Accepted:    30K
 * Total Submissions: 68K
 * Testcase Example:  '[1,2,3]'
 *
 * 给你一个由 无重复 正整数组成的集合 nums ，请你找出并返回其中最大的整除子集 answer ，子集中每一元素对 (answer[i],
 * answer[j]) 都应当满足：
 * 
 * answer[i] % answer[j] == 0 ，或
 * answer[j] % answer[i] == 0
 * 
 * 
 * 如果存在多个有效解子集，返回其中任何一个均可。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [1,2,3]
 * 输出：[1,2]
 * 解释：[1,3] 也会被视为正确答案。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1,2,4,8]
 * 输出：[1,2,4,8]
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 1 
 * nums 中的所有整数 互不相同
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        if nums.len() == 1 {
            return nums;
        }
        nums.sort();
        let mut last: Vec<i32> = vec![-1; nums.len()];
        let mut count: Vec<i32> = vec![1; nums.len()];
        let mut max: i32 = 0;
        let mut index: i32 = -1;
        for i in (1..nums.len()) {
            for j in (0..i) {
                if nums[i] % nums[j] == 0 {
                    if count[j] + 1 > count[i] {
                        count[i] = count[j] + 1;
                        last[i] = j as i32;
                    }
                }
            }
            if count[i] > max {
                max = count[i];
                index = i as i32;
            }
        }
        let mut result: Vec<i32> = Vec::new();
        while index > -1 {
            result.push(nums[index as usize]);
            index = last[index as usize];
        }
        result
    }
}
// @lc code=end

