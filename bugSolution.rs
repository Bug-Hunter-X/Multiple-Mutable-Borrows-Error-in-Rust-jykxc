fn main() {
    let mut x = 5;
    { //Create a new scope to limit the borrow of y
        let y = &mut x;
        *y += 1;
    }
    {   //Create a new scope to limit the borrow of z
        let z = &mut x;
        *z += 1;
    }
    println!("x = {}", x); // x will be 7
}

//Alternative solution using clone
fn main(){
    let mut x = 5;
    let mut y = x.clone();
    let mut z = x.clone();
    y += 1;
    z += 1;
    println!("x = {}", x); //x will still be 5
    println!("y = {}", y); //y will be 6
    println!("z = {}", z); //z will be 6
} 