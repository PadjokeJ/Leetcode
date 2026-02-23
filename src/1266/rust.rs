impl Solution {
    pub fn min_time_to_visit_all_points(points: Vec<Vec<i32>>) -> i32 {
        let mut m = 0;

        for i in 1..points.len() {
            let dx = points[i][0] - points[i - 1][0];
            let dy = points[i][1] - points[i - 1][1];

            m += if dx.abs() > dy.abs() { dx.abs() } else { dy.abs() };
        }

        m
    }
}

