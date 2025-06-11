use std::io;
// recursive function and recursion tree

fn tower(n:i32,left:i32,right:i32,middle:i32,moves:&mut Vec<(i32, i32)>) {
    if n == 0 {
        return;
    }
    tower(n-1, left,middle,right, moves);
    moves.push((left,right)); //push the move manually
    tower(n-1,middle,right,left, moves);
}

fn main(){
    println!("Enter the number of disks(1 to 16)");

    let mut input = String::new();

    io::stdin().read_line(&mut input).unwrap(); // look for proper error handing

    let n:i32 = input.trim().parse().unwrap();

    if n< 1 || n > 16 {
        println!("please enter number btn 1 and 16");
        return;
    }

    let mut moves = Vec::new();
    tower(n,1,3,2, &mut moves);
// printing the minimium moves
    println!("{} moves", moves.len());

// printing the moves 
    for (from, to) in moves {
        println!("from {} to {}",from, to);
    }
}