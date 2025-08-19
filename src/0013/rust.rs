use std::collections::HashMap;

impl Solution {
    pub fn roman_to_int(s: String) -> i32 {
        let v: Vec<char> = s.chars().collect();
        let mut rom: HashMap<char, i32> = HashMap::from([
            ('I', 1), 
            ('V', 5), 
            ('X', 10), 
            ('L', 50), 
            ('C', 100), 
            ('D', 500), 
            ('M', 1000),
        ]);

        let mut prev = rom[&v[0]];
        let mut num = 0;
        for c in v {
            let curr = rom[&c];
            if prev < curr {
                num -= prev * 2;
            } 
            num += curr;
            prev = curr;
        }

        num
    }
}
