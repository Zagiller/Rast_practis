fn main() {
    const HEIGHT: usize = 7; // Висота половини ромбу (загальна висота = 2 * HEIGHT - 1)

    let mut diamond = String::new();

    // Верхня частина ромба
    for i in 0..HEIGHT {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;
        diamond.push_str(&" ".repeat(spaces));
        diamond.push_str(&"*".repeat(stars));
        diamond.push('\n');
    }

    // Нижня частина ромба
    for i in (0..HEIGHT - 1).rev() {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;
        diamond.push_str(&" ".repeat(spaces));
        diamond.push_str(&"*".repeat(stars));
        diamond.push('\n');
    }

    // Вивід ромбу одним print!() та одним println!()
    print!("{}", diamond);
}
