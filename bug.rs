fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    let mut iter = vec.iter();
    println!("First element: {}", iter.next().unwrap());
    //Try to print vec[1], it will panic
    println!("Second element: {}", vec[1]); 
}