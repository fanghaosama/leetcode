/*
 * @lc app=leetcode.cn id=264 lang=rust
 *
 * [264] 丑数 II
 *
 * https://leetcode-cn.com/problems/ugly-number-ii/description/
 *
 * algorithms
 * Medium (55.55%)
 * Likes:    576
 * Dislikes: 0
 * Total Accepted:    63.7K
 * Total Submissions: 113.2K
 * Testcase Example:  '10'
 *
 * 给你一个整数 n ，请你找出并返回第 n 个 丑数 。
 * 
 * 丑数 就是只包含质因数 2、3 和/或 5 的正整数。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：n = 10
 * 输出：12
 * 解释：[1, 2, 3, 4, 5, 6, 8, 9, 10, 12] 是由前 10 个丑数组成的序列。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：n = 1
 * 输出：1
 * 解释：1 通常被视为丑数。
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * 1 
 * 
 * 
 */

// @lc code=start
impl Solution {
    pub fn nth_ugly_number(n: i32) -> i32 {
        let mut primes: Vec<i32> = vec![1];
        let mut p1 = 0;
        let mut p2 = 0;
        let mut p3 = 0;
        let mut count = 1;
        while count != n {
            let a = primes[p1] * 2;
            let b = primes[p2] * 3;
            let c = primes[p3] * 5;
            let mut min: Option<i32> = None;
            if a <= b && a <= c {
                min = Some(a);
                p1 += 1;
            } else if b <= a && b <= c {
                min = Some(b);
                p2 += 1;
            } else if c <= b && c <= a {
                min = Some(c);
                p3 += 1;
            }
            if primes.last().unwrap() != &min.unwrap() {
                primes.push(min.unwrap());
                count += 1;
            }
        }
        primes[count as usize - 1]
    }
}
// @lc code=end

// failed:
// time limit exceed
// pub fn nth_ugly_number(n: i32) -> i32 {
//     let mut count = 0;
//     let mut num = 0;
//     let mut primes: Vec<i32> = vec![2, 3, 5];
//     while count != n {
//         num += 1;
//         if Solution::is_ugly(num, &primes) {
//             count += 1;
//             // if num > primes[primes.len() -1] {
//             //     primes.push(num);
//             // }
//         }
//     }
//     num
// }

// pub fn is_ugly(n: i32, primes: &Vec<i32>) -> bool {
//     let mut m = n;
//     if m == 0 {
//         return false;
//     }
//     for p in primes.iter().rev() {
//         while m >= *p {
//             if m % *p == 0 {
//                 m = m / *p;
//             } else {
//                 break;
//             } 
//         }
//     }
//     m == 1
// }

