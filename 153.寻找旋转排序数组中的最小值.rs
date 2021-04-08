/*
 * @lc app=leetcode.cn id=153 lang=rust
 *
 * [153] 寻找旋转排序数组中的最小值
 */

// @lc code=start
impl Solution {
    pub fn find_min(nums: Vec<i32>) -> i32 {
        if nums.len() == 1 {
            return nums[0];
        }
        if nums[0] < nums[nums.len() - 1] {
            return nums[0];
        }
        let mut l = 0;
        let mut r = nums.len() - 1;
        let mut pivot = (r - l + 1) / 2;
        while l != r {
            pivot = (r - l + 1) / 2 + l;
            if nums[pivot] >= nums[r] {
                l = pivot;
            }
            if nums[pivot] < nums[r] {
                r = pivot;
            }
        }
        nums[l]
    }
}
// @lc code=end

