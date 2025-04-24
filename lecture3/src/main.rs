mod calc;
fn main(){
    let addition = calc::add::addition(5,3);
    println!("sum is {}", addition);

    let  subtract= calc::subtract::sub(5,3);
    println!("sub is {}", subtract);

    let div = calc::div::division(5,3);
    println!("Division is {}", div);
}