fn invert_the_case(s: String) -> String {
    s.chars()
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next().unwrap()
            } else if c.is_uppercase() {
                c.to_lowercase().next().unwrap()
            } else {
                c
            }
        })
        .collect()
}
#[test]
fn main() {
    let input = "Привіт Hello".to_string();
    let inverted = invert_the_case(input.clone());
    println!("Вхідний рядок: {}", input);
    println!("Інвертований рядок: {}", inverted);
}
fn test() {
    let data = [("Hello", "hELLO"), ("Привsт", "пРИВІТ")];

    data.iter().for_each(|(a, b)| {
        assert_eq!(invert_the_case(a.to_string()), b.to_string());
        assert_eq!(invert_the_case(b.to_string()), a.to_string());
    });
}


