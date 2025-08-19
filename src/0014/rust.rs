use std::cmp::max;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        if strs.is_empty() {
            return "".to_string();
        }
        let mut common = strs[0].clone();
        for strng in strs.iter().skip(1) {
            let mut cm: String = "".to_string();

            let it = common.len();
            for i in 0..it {
                let c = strng.chars().nth(i).unwrap_or('1'); 
                if c == '1' {
                    break;
                }
                if c != common.chars().nth(i).unwrap_or('0') {
                    break;
                }
                cm.push(c);
            }
            common = cm;

            if common.is_empty() {
                break;
            }
        }
        common
    }
}
