/*
 * @lc app=leetcode.cn id=80 lang=rust
 *
 * [80] 删除有序数组中的重复项 II
 */

// @lc code=start
impl Solution {
    pub fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
        if nums.len() <= 2 {
            return nums.len() as i32;
        }
        let mut l = 2;
        let mut r = 2;
        while r < nums.len() {
            if nums[l - 2] != nums[l - 1] || nums[l - 1] != nums[r] {
                nums[l] = nums[r];
                l += 1;
            }
            r += 1;
        }
        (nums.len() - (r - l)) as i32
    }
}
// @lc code=end

