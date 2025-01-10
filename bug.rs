fn main() {
    let mut x = 5;
    let y = &mut x; 
    let z = &mut x; // This will cause an error because of multiple mutable borrows
    *y += 1;
    *z += 1;
}