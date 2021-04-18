/*
 * @lc app=leetcode.cn id=213 lang=rust
 *
 * [213] 打家劫舍 II
 *
 * https://leetcode-cn.com/problems/house-robber-ii/description/
 *
 * algorithms
 * Medium (41.27%)
 * Likes:    651
 * Dislikes: 0
 * Total Accepted:    117.5K
 * Total Submissions: 276.5K
 * Testcase Example:  '[2,3,2]'
 *
 * 你是一个专业的小偷，计划偷窃沿街的房屋，每间房内都藏有一定的现金。这个地方所有的房屋都 围成一圈
 * ，这意味着第一个房屋和最后一个房屋是紧挨着的。同时，相邻的房屋装有相互连通的防盗系统，如果两间相邻的房屋在同一晚上被小偷闯入，系统会自动报警 。
 * 
 * 给定一个代表每个房屋存放金额的非负整数数组，计算你 在不触动警报装置的情况下 ，今晚能够偷窃到的最高金额。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：nums = [2,3,2]
 * 输出：3
 * 解释：你不能先偷窃 1 号房屋（金额 = 2），然后偷窃 3 号房屋（金额 = 2）, 因为他们是相邻的。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：nums = [1,2,3,1]
 * 输出：4
 * 解释：你可以先偷窃 1 号房屋（金额 = 1），然后偷窃 3 号房屋（金额 = 3）。
 * 偷窃到的最高金额 = 1 + 3 = 4 。
 * 
 * 示例 3：
 * 
 * 
 * 输入：nums = [0]
 * 输出：0
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
impl Solution {
    pub fn rob(nums: Vec<i32>) -> i32 {
        match nums.len() {
            1 => { return nums[0]; },
            2 => {
                if nums[0] > nums[1] {
                    return nums[0];
                } else {
                    return nums[1];
                }
            },
            3 => {
                return Solution::maximize(nums[0], nums[1], nums[2]);
            },
            _ => {
                let a = Solution::robMost(nums[0..nums.len()-1].to_vec());
                let b = Solution::robMost(nums[1..nums.len()].to_vec());
                if a > b {
                    return a;
                } else {
                    return b;
                }
            }
        }
        0
    }
    pub fn robMost(nums: Vec<i32>) -> i32 {
        let mut dp: Vec<i32> = nums.clone();
        dp[0] = nums[0];
        dp[1] = nums[1];
        dp[2] = Solution::maximize(nums[0] + nums[2], nums[1], nums[2]);
        for i in (3..nums.len()) {
            dp[i] = Solution::maximize(dp[i - 1], dp[i - 2] + nums[i], dp[i - 3] + nums[i]);
        }
        Solution::maximize(dp[dp.len() -1], dp[dp.len() - 2], dp[dp.len() - 3])
    }
    pub fn maximize(n1: i32, n2: i32, n3: i32) -> i32 {
        let m = if n1 > n2 { n1 } else { n2 };
        if m > n3 {
            m
        } else {
            n3
        }
    }
}
// @lc code=end

