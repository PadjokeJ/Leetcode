impl Solution {
    pub fn has_all_codes(s: String, k: i32) -> bool {
        if (s.len() as i32) < k * 2 {
            return false;
        }

        let mut hash = vec![false; 2usize.pow(k as u32)];
        let cu = s.as_bytes();
        for i in 0..(s.len() - (k as usize) + 1) {
            let mut bin = 0;

            for j in 0..k {
                bin += if (cu[j as usize + i]) == 49u8 { 1 << j } else { 0 };    
            }

            hash[bin] = true;
        }

        for i in hash.iter() {
            if *i == false {
                return false;
            }
        }
        true
    }
}

