fn main() {
    ints();
    tupples_breakdown();
    arrays_print();
    functions_();
}

fn functions_(){
    print_int(3);
    
    let result = multiply_by_two(9);
    println!("result {}", result);


}

fn multiply_by_two(x:i32)->i32 {
    //without semicolon
    x*2
}

fn print_int(x:i32){
    println!("x param = {}",x);
}

fn arrays_print(){
    let people = ["Joe", "Jack", "Maria", "Kost"];
    println!("people index 0 {}", people[0]);
    println!("people, pretty print: {:#?}", people);

    // println!("people index 99 {}", people[99]);
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
