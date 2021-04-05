/*
 * @lc app=leetcode.cn id=88 lang=rust
 *
 * [88] 合并两个有序数组
 */

// @lc code=start
impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        if nums2.len() == 0 || n == 0 {
            return;
        }
        // 倒序，从nums1后方空白处开始填入，正好可以规避额外的空间开销
        let mut i = m - 1;
        let mut j = n - 1;
        let mut k = m + n - 1;
        while i >= 0 && j >= 0 {
            if nums1[i as usize] > nums2[j as usize] {
                nums1[k as usize] = nums1[i as usize];
                i -= 1;
            } else {
                nums1[k as usize] = nums2[j as usize];
                j -= 1;
            }
            k -= 1;
        }
        while j >= 0 {
            nums1[k as usize] = nums2[j as usize];
            j -= 1;
            k -= 1;
        }
    }
}
// @lc code=end

// with new space O(m + n) 
pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    if nums2.len() == 0 || n == 0 {
        return;
    }
    let numsT = nums1.clone();
    let mut i = 0;
    let mut j = 0;
    let mut k = 0;
    while i < m && j < n {
        if numsT[i as usize] < nums2[j as usize] {
            nums1[k as usize] = numsT[i as usize];
            i += 1;
        } else {
            nums1[k as usize] = nums2[j as usize];
            j += 1;
        }
        k += 1;
    }
    while i < m {
        nums1[k as usize] = numsT[i as usize];
        i += 1;
        k += 1;
    }
    while j < n {
        nums1[k as usize] = nums2[j as usize];
        j += 1;
        k += 1;
    }
}