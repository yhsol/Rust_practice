fn main() {
    let mut d = [2, 4, 5, 1, 3];
    let mut n = d.len();
    for i in (0..n - 1).rev() {
        let mut min_idx = i;
        for j in (i + 1..n).rev() {
            if d[j] < d[min_idx] {
                min_idx = j;
                println!("{}", d[min_idx])
            }
        }
    }
}
