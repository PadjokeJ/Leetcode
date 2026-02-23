impl Solution {
    pub fn binary_gap(n: i32) -> i32 {
        let mut max = 0;
        let mut li = 0;

        while !Self::is_bit(n, li) {
            li += 1;
        }

        for i in li..32 {
            if Self::is_bit(n, i) {
                max = if max > i - li { max } else { i - li };
                li = i;
            }
        }

        max
    }

    fn is_bit(n: i32, index: i32) -> bool {
        let mask = 1 << index;

        return (mask & n) != 0;
    }
}

