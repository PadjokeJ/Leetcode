impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        if x >= 0 {
            let v: Vec<char> = x.to_string().chars().collect();

            let l = v.len() - 1;

            for i in 0..l {
                if v[i] != v[l - i] {
                    return false;
                }
            }
            return true;
        }
        false
    }
}
