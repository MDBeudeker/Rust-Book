fn main() {
    let v1 = vec![1, 2, 3];

    let v1_iter = v1.iter();

    for val in v1_iter{
        println!("Got {}", val)
    }

    let mut v2_iter = v1.iter();

    println!("Got {:?}", v2_iter.next().unwrap());
    println!("Got {:?}", v2_iter.next().unwrap_or_else(|| &0)); // prints the next value
    println!("Got {:?}", v2_iter.next().unwrap_or_else(|| &0));
    println!("Got {:?}", v2_iter.next().unwrap_or_else(|| &0));
    println!("Got {:?}", v2_iter.next().unwrap_or_else(|| &0));

    let mut v3_iter = v1.iter();

    let mut n = 0;

    while n<10 {
        println!("V3 got {:?}", v3_iter.next().unwrap_or_else(|| &0 ));
        n += 1;
    }

    let mut total: i32 = v3_iter.sum();
    println!("{:?}", total);
    
    let v4_iter = v1.iter();
    total = v4_iter.sum();
    println!("{:?}", total);

    let v11: Vec<i32> = vec![1,2,3];

    let v12: Vec<i32> =v11.iter().map(|x| x+1).collect();

    println!("{:?}", v12);
}
