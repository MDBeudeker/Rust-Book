

fn main() {
    let mut list: [i32; 16] =[1,7,5,1,7,6,9,8,3,2,1,7,7,3,5,6];
    println!("{:?}",list);

    let m = median(&mut list);
    println!("Median is {:?}!", m);

    let mo = mode(&mut list);
    println!("Mode is {:?}!", mo);


}

fn median(array: &mut [i32]) -> i32 {
    let mut sorted:Vec<i32> = Vec::new();
    for item in array{
         sorted.push(*item);
    }
    sorted.sort();
     let mut s: usize = sorted.len();
     let s = s/2;
     sorted[s]
}

fn mode(array: &mut [i32]) -> i32 { // change to hashmap
    use std::collections::HashMap;
    let mut countmap = HashMap::new();
    
    for item in array{
         let count = countmap.entry(item).or_insert(0);
         *count +=1;
    }

    // magic https://stackoverflow.com/questions/62525693/how-do-i-get-the-key-associated-with-the-maximum-value-of-a-rust-hashmap
    let s = countmap
        .iter()
        .max_by(|a, b| a.1.cmp(&b.1))
        .map(|(k, _v)| k);
    
    **s.unwrap()
}
