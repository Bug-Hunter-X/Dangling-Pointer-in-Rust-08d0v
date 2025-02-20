fn main() {
    let mut v = vec![1, 2, 3];
    // Instead of using raw pointers, we can use safe methods to modify the vector.
    v[0] = 4; 
    println!("The first element is: {}", v[0]);
    // Or using iterators:
    for x in v.iter_mut(){
        *x = *x * 2;
    }
    println!("The modified vector is: {:?}", v);
} 