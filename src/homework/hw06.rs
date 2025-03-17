fn draw_tree(triangle_count: usize) {
    let mut tree = String::new();
    
    (1..=triangle_count).for_each(|t| {
        (0..t + 2).for_each(|i| {
            let spaces = triangle_count + 1 - i;
            let stars = 2 * i + 1;
            if i > 0 {
                tree.push_str(&" ".repeat(spaces));
                tree.push_str(&"*".repeat(stars));
                tree.push('\n');
            }
        });
    });

    // Виводимо ялинку одним print!
    print!("{}", tree);
}

fn main() {
    let triangle_count = 4; // Кількість трикутників
    draw_tree(triangle_count);
}
