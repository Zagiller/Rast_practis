use permutohedron::Heap;

fn main() {
    let mut digits = vec![1, 2, 3, 4, 5, 6, 7, 8]; // Цифри від 1 до 8
    let heap = Heap::new(&mut digits);
    
    for perm in heap {
        let (m, u, x, a, s, l, o, n) = (perm[0], perm[1], perm[2], perm[3], perm[4], perm[5], perm[6], perm[7]);
        
        let muxa = m * 1000 + u * 100 + x * 10 + a;
        let slon = s * 1000 + l * 100 + o * 10 + n;
        
        if muxa * a == slon {
            println!("{} x {} = {}", muxa, a, slon);
        }
    }
}
