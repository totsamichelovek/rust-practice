fn rotate(s: String, n: isize) -> String {
    if s.is_empty() {
        return s;
    }

    let len = s.len() as isize;
    let shift = (n % len + len) % len; // Ensure positive shift

    let (left, right) = s.split_at(len as usize - shift as usize);
    format!("{}{}", right, left)
}
#[test]
fn main() {
    let s = "abcdefgh".to_string();
    let n = 2;
    let rotated = rotate(s.clone(), n);
    println!("{} передвинуто на {} = {}", s, n, rotated);
}
fn test() {
    let s = "abcdefgh";
    let shifts = [
        (0, "abcdefgh"),
        (8, "abcdefgh"),
        (-8, "abcdefgh"),
        (1, "habcdefg"),
        (2, "ghabcdef"),
        (10, "ghabcdef"),
        (-1, "bcdefgha"),
        (-2, "cdefghab"),
        (-10, "cdefghab"),
    ];

    shifts.iter().for_each(|(n, exp)| {
        assert_eq!(rotate(s.to_string(), *n), exp.to_string())
    });
}