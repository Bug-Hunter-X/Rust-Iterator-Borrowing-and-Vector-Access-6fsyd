fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //Solution 1: Consume the iterator
    let mut iter = vec.iter();
    println!("First element: {}", iter.next().unwrap());
    let remaining_vec: Vec<_> = iter.collect();
    println!("Second element: {}", remaining_vec[0]);

    //Solution 2:  Access via direct indexing
    let mut vec2 = Vec::new();
    vec2.push(1);
    vec2.push(2);
    vec2.push(3);
    println!("Second element (direct indexing): {}", vec2[1]);
} 