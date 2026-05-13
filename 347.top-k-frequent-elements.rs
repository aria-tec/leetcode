// @leet start
impl Solution {
    pub fn top_k_frequent(nums: Vec<i32>, k: i32) -> Vec<i32> {
        let mut count = HashMap::new();
        for n in nums {
            // Check entry n, if not present, insert 0, evaluation result + 1
            *count.entry(n).or_insert(0) += 1;
        }

        let mut freq: Vec<Vec<i32>> = vec![vec![]; nums.len() + 1];
        
    }
}
// @leet end
