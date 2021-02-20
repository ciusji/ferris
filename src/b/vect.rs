// collection ops
pub fn coll() {
    // vector
    let mut vector = vec![1, 2, 4, 8];
    vector.push(16);
    vector.push(32);
    println!("{:?}", vector);
    // vector foreach
    for i in vector {
        println!("{}", i);
    }

    // string
    let one = 1.to_string();
    let two = 1.234.to_string();
    let ot = one + &two;
    println!("{}", ot);
    println!("{}", ot.len());

}