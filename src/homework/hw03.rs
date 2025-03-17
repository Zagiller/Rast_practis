fn main() {
    const WIDTH: usize = 20;  // Ширина конверта
    const HEIGHT: usize = 10; // Висота конверта

    let mut envelope = vec![vec![' '; WIDTH]; HEIGHT];

    // Малюємо рамку
    for x in 0..WIDTH {
        envelope[0][x] = '*';
        envelope[HEIGHT - 1][x] = '*';
    }
    for y in 0..HEIGHT {
        envelope[y][0] = '*';
        envelope[y][WIDTH - 1] = '*';
    }

    // Малюємо діагональні лінії
    for i in 0..HEIGHT {
        let left_diag_x = i * (WIDTH - 1) / (HEIGHT - 1);
        let right_diag_x = (HEIGHT - 1 - i) * (WIDTH - 1) / (HEIGHT - 1);
        envelope[i][left_diag_x] = '*';
        envelope[i][right_diag_x] = '*';
    }

    // Вивід результату одним print!()
    print!("{}", envelope.iter()
        .map(|row| row.iter().collect::<String>() + "\n")
        .collect::<String>());
}
