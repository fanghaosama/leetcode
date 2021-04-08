/*
 * @lc app=leetcode.cn id=81 lang=rust
 *
 * [81] 搜索旋转排序数组 II
 */

// @lc code=start
impl Solution {
    pub fn search(nums: Vec<i32>, target: i32) -> bool {
        Solution::search2(&nums, target, 0, nums.len() - 1)
    }

    pub fn search2(nums: &Vec<i32>, target: i32, l: usize, r: usize) -> bool {
        let len = r - l + 1;
        if len == 0 {
            return false;
        }
        if len == 1 {
            return nums[r] == target;
        }
        let m = len / 2 + l;
        if nums[l] < nums[m - 1] && nums[m] < nums[r] {
            if (target < nums[l] || target > nums[m-1]) &&
                (target < nums[m] || target > nums[r]) {
                    return false;
                }
        }
        if nums[l] < nums[m - 1] {
            if (target < nums[l] || target > nums[m-1]) {
                return Solution::search2(nums, target, m, r);
            }
        }
        if nums[m] < nums[r] {
            if (target < nums[m] || target > nums[r]) {
                return Solution::search2(nums, target, l, m - 1);
            }
        }
        Solution::search2(nums, target, l, m - 1) || Solution::search2(nums, target, m, r)
    }
}
// @lc code=end

 