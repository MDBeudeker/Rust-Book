
/* // Inefficient way of doing a conditional match
fn main() {
    let config_max = Some(3u8);
    match config_max{
        Some(max) => println!("the maximum configured is to be {}", max),
        _ => (),
    }
}
 */

 // If let

 fn main(){
    let config_max = Some(3u8)
    if let Some(max) = config_max{
        println!("The maximum is configured to be {}", max)
    }
 }