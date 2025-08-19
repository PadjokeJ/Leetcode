impl Solution {
    pub fn remove_element(nums: &mut Vec<i32>, val: i32) -> i32 {
        if nums.is_empty() {
            return 0;
        }
        let repl = if val != 0 { 0 } else { -1 };

        let mut num = 0;
        let mut i = 0;
        while true {
            if nums[i] == val {
                nums.remove(i);
            } else {
                i += 1;
                num += 1;
            }

            if i >= nums.len() {
                break;
            }
        }
        for i in 0..num {
            nums.push(repl);
        }
        num
    }
}
