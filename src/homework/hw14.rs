fn occupied_area(rectangles: &Vec<(i32, i32, i32, i32)>) -> i32 {
    use std::collections::HashSet;

    let mut occupied_cells = HashSet::new();

    for &(x1, y1, x2, y2) in rectangles {
        for x in x1..x2 {
            for y in y1..y2 {
                occupied_cells.insert((x, y));
            }
        }
    }

    occupied_cells.len() as i32
}

fn main() {
    let rectangles = vec![
        (1, 1, 4, 4), // Прямокутник (x1, y1, x2, y2)
        (2, 2, 5, 5),
    ];

    println!("Occupied area: {}", occupied_area(&rectangles));
}

