fn is_prime(n: &u32) -> bool {
    if *n <= 1 {
        return false;
    }
    for i in 2..=(*n as f64).sqrt() as u32 {
        if *n % i == 0 {
            return false;
        }
    }
    true
}
#[test]
fn main() {
    let numbers = [0, 1, 2, 3, 4, 5, 6, 69, 1337];
    for number in numbers.iter() {
        let result = is_prime(number);
        println!("{} просте число?: {}", number, result);
    }
}


fn test_is_prime() {
    let test_data = [
        (0, false),
        (1, false),
        (2, true),
        (3, true),
        (4, false),
        (5, true),
        (6, false),
        (69, false),
        (1337, false),
    ];

    test_data.iter().for_each(|(n, prime)| assert_eq!(is_prime(n), *prime));
}