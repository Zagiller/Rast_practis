use std::collections::HashSet;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn calculate_occupied_area(rectangles: Vec<((i32, i32), (i32, i32))>) -> usize {
    let mut occupied_points: HashSet<Point> = HashSet::new();
    
    for &((x1, y1), (x2, y2)) in &rectangles {
        for x in x1.min(x2)..x1.max(x2) {
            for y in y1.min(y2)..y1.max(y2) {
                occupied_points.insert(Point { x, y });
            }
        }
    }
    
    occupied_points.len()
}

fn main() {
    let rectangles = vec![
        ((2, 9), (5, 3)), // Червоний прямокутник
        ((5, 3), (10, 7)), // Синій прямокутник
        ((3, 6), (8, 5)), // Зелений прямокутник
    ];
    
    let total_area = calculate_occupied_area(rectangles);
    println!("Загальна зайнята площа: {}", total_area);
}
