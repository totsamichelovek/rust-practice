#[test]
fn main() {
    const H: u32 = 6;
    const W: u32 = 2 * H - 1;

    for y in 0..(2 * H - 1) {
        let mirror_y = 2 * H - 2 - y;
        let row = if y < H { y } else { mirror_y };
        let stars = 2 * row + 1;
        let spaces = (W - stars) / 2;

        for x in 0..W {
            let sym = match x {
                _ if x < spaces => ' ',
                _ if x >= spaces + stars => ' ',
                _ => '*',
            };
            print!("{}", sym);
        }
        println!();
    }
}




