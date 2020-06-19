fn main() {
   
    ints();
    tupples_breakdown();
}

fn tupples_breakdown(){
    let tup: (i64, i32, u8) = (400,200,3);
    let big_tupple: (i64, i32, u8,u8,u8) = (400,200,3,2,1);
    // breakdown
    let (x,y,z) = tup;
    println!("x = {}", x);
    println!("y = {}", y);
    println!("z = {}", z);

    let a = tup.0;
    println!("a = {}", a);

    println!("big_tupple.4 = {}", big_tupple.4);
}

fn ints(){
    let x = 4;
    println!("X is {}", x);

    let mut x2 = 4;
    println!("X2 is {}", x2);
    x2 = 55;
    println!("X2 chanaged to {}", x2);


    let x: i64 = 3;
    println!("x={} 😋😋😋",x);
}
