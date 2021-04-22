 /*
 * @lc app=leetcode.cn id=363 lang=rust
 *
 * [363] 矩形区域不超过 K 的最大数值和
 *
 * https://leetcode-cn.com/problems/max-sum-of-rectangle-no-larger-than-k/description/
 *
 * algorithms
 * Hard (39.99%)
 * Likes:    283
 * Dislikes: 0
 * Total Accepted:    23.2K
 * Total Submissions: 48.8K
 * Testcase Example:  '[[1,0,1],[0,-2,3]]\n2'
 *
 * 给你一个 m x n 的矩阵 matrix 和一个整数 k ，找出并返回矩阵内部矩形区域的不超过 k 的最大数值和。
 * 
 * 题目数据保证总会存在一个数值和不超过 k 的矩形区域。
 * 
 * 
 * 
 * 示例 1：
 * 
 * 
 * 输入：matrix = [[1,0,1],[0,-2,3]], k = 2
 * 输出：2
 * 解释：蓝色边框圈出来的矩形区域 [[0, 1], [-2, 3]] 的数值和是 2，且 2 是不超过 k 的最大数字（k = 2）。
 * 
 * 
 * 示例 2：
 * 
 * 
 * 输入：matrix = [[2,2,-1]], k = 3
 * 输出：3
 * 
 * 
 * 
 * 
 * 提示：
 * 
 * 
 * m == matrix.length
 * n == matrix[i].length
 * 1 
 * -100 
 * -10^5 
 * 
 * 
 * 
 * 
 * 进阶：如果行数远大于列数，该如何设计解决方案？
 * 
 */

// @lc code=start
impl Solution {
    pub fn max_sum_submatrix(matrix: Vec<Vec<i32>>, x: i32) -> i32 {
        let mut max: i32 = -100000;
        let mut store: Vec<Vec<Vec<i32>>> = Vec::new();
        for m in matrix.iter() {
            let mut table: Vec<Vec<i32>> = vec![vec![0; m.len()]; m.len()];
            for i in (0..m.len()).rev() {
                for j in (i..m.len()) {
                    if i == j {
                        table[i][j] = m[i];
                    } else {
                        table[i][j] = table[i][j - 1] + m[j];
                    }
                    
                    if table[i][j] == x {
                        return x;
                    } else if table[i][j] < x {
                        max = if table[i][j] > max { table[i][j] } else { max };
                    }
                }
            }
            store.push(table);
        }
        let mlen = matrix[0].len();
        for k in (0..mlen).rev() {
            for l in (k..mlen) {
                let mut dp: Vec<Vec<i32>> = vec![vec![0; store.len()]; store.len()];
                for p in (0..store.len()).rev() {
                    for q in (p..store.len()) {
                        if p == q {
                            dp[p][q] = store[p][k][l];
                        } else {
                            dp[p][q] = dp[p][q - 1] + store[q][k][l];
                        }
                        if dp[p][q] == x {
                            return x;
                        } else if dp[p][q] < x {
                            max = if dp[p][q] > max { dp[p][q] } else { max };
                        }
                    }
                }
            }
        }
        max
    }
}
// @lc code=end

