impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut i = 1;
        if s.len() == 0 {
            return 0;
        }
        while s.as_bytes()[s.len() - i] == 32u8 {
            i += 1;
        }

        let mut length = 0;
        while s.as_bytes()[s.len() - i] != 32u8 {
            length += 1;
            i += 1;
            if i >= s.len() + 1 {
                break;
            }
        }

        length
    }
}
