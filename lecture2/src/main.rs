// Here we are learning to use println macro and writing functions in rust
// We also have different data types we have in rust 
// Next lecture we shall talk about heap and stack
fn data_types() {
    let a: i8 = 10; //positive and negative
    let b: u32 = 100; //only positives
    let c: f64 = 3.14; // floats or decimals
    let d: bool = true; //True or False
    let e: char = 'A'; //letter or character

    println!("{a},{b},{c},{d},{e}");
    println!("{},{},{},{},{}",a,b,c,d,e);

}

fn data_structures(){
    let numbers = vec![1,2,3,4,5,6];
    println!("{:?}", numbers);
    println!("{:#?}", numbers);
}

fn main() {
    data_types();
    data_structures();  
}
