use rand::Rng;

// Генерує вектор випадкових чисел у діапазоні [10..99]
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..100)).collect()
}

// Знаходить пару сусідніх елементів із мінімальною сумою
fn min_adjacent_sum(data: &[i32]) -> (usize, usize, i32, i32, i32) {
    let mut min_sum = i32::MAX;
    let mut min_index = 0;

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_index = i;
        }
    }

    (min_index, min_index + 1, data[min_index], data[min_index + 1], min_sum)
}

// Форматований вивід у консоль
fn print_result(data: &[i32]) {
    // Вивід індексів
    print!("indexes: ");
    for i in 0..data.len() {
        print!("{:>3}.", i);
    }
    println!();

    // Вивід даних
    print!("data:    ");
    for &num in data {
        print!("{:>3},", num);
    }
    println!();

    // Знаходження мінімальної пари
    let (idx1, idx2, val1, val2, sum) = min_adjacent_sum(data);

    // Візуалізація індексів
    print!("indexes: ");
    for i in 0..data.len() {
        if i == idx1 {
            print!(" \\__");
        } else if i == idx2 {
            print!("__/ ");
        } else {
            print!("    ");
        }
    }
    println!();

    // Вивід результату
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        val1, val2, sum, idx1, idx2
    );
}

fn main() {
    let data = gen_random_vector(20);
    print_result(&data);
}
