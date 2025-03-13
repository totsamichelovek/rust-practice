#[test]
fn main() {
    let num_triangles = 5;
    let max_width = 2 * (num_triangles + 1) - 1;
    println!("{:^width$}", "*", width = max_width as usize);
    (1..=num_triangles).for_each(|i| {
        (1..=i+1).for_each(|j| {
            let stars = 2 * j - 1;
            println!("{:^width$}", "*".repeat(stars as usize), width = max_width as usize);
        });
    });
}
