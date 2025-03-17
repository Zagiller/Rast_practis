fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let n = shipments.len();
    let total: u32 = shipments.iter().sum();
    
    // Якщо не можна поділити порівну, повертаємо None
    if total as usize % n != 0 {
        return None;
    }
    
    let avg = total / n as u32;
    let mut moves = 0;
    
    for &ship in shipments {
        if ship > avg {
            moves += (ship - avg) as usize;
        }
    }
    
    Some(moves)
}

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut shipments = vec![10; n]; // початково всі кораблі мають 10
    shipments[0] += 2; // робимо нерівномірність
    shipments[1] -= 2;
    shipments
}

fn main() {
    let shipments = vec![8, 2, 2, 4, 4];
    match count_permutation(&shipments) {
        Some(moves) => println!("Мінімальна кількість переміщень: {}", moves),
        None => println!("Неможливо вирівняти вагу між кораблями"),
    }
    
    let gen_ships = gen_shipments(5);
    println!("Згенеровані вантажі: {:?}", gen_ships);
}
