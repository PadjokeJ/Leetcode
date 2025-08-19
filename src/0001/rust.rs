impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut i: i32 = 0;
        for num in nums.clone() {
            let mut k: i32 = 0;
            for num2 in nums.clone() {
                if num + num2 == target && i != k {
                    return vec![i, k];
                }
                k += 1;
            }
            i += 1;
        }
        vec![]
    }
}
