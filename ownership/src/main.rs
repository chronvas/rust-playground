fn main() {
    with_strings();
    
    with_strings2();
    with_ints2();

    with_strings3();
}


fn with_strings3(){
    let string33 = String::from("init 33");
    alter_and_print_it(string33);
    // not possible to use string33 again. 
    
    let string34 = String::from("init 34");
    borrowed_print(&string34);
    borrowed_print(&string34);
    
    //even weirder
    let mut string35 = String::from("init 35");
    borrowed_mutable_print(&mut string35);
    borrowed_mutable_print(&mut string35);

    let mut string36 = String::from("init 36");
    mutable_print(string36);
    // not possible to use string36 again
}
fn mutable_print(mut string: String ){
    string.push_str("altered");
    println!("string: {}", string);
}
fn borrowed_mutable_print(string: &mut String){
    string.push_str("altered");
    println!("string: {}", string);
}
fn borrowed_print(string34: &String){
    println!("string33: {}", string34);
}
fn alter_and_print_it(mut string33: String){
    string33.push_str("-altered");
    println!("string33: {}", string33);
    string33.push_str("-again");
    println!("string33: {}", string33);
}

fn with_ints2(){
    let int1 = 4;
    println!("{}", int1);
    print_it_int(int1);
    //this works for ints, but not for strings? ??
    println!("{}", int1);
}
fn print_it_int(int1: i32){
    println!("{}", int1);
}


fn with_strings2(){
    let string1 = String::from("s111");
    println!("{}", string1);
    print_it(string1);
    //this crashes, value borrowd after move ??
    // println!("{}", string1);
}
fn print_it(string1:String){
    println!("{}", string1);
}


fn with_strings(){
    let s1 = String::from("Owowowow");

    println!("s1: {}",s1);

    let s2 = s1;

    // this crashes for some reason
    // println!("s1: {}",s1);
    println!("s2: {}",s2);


    let i1 = 2;
    println!("i1: {}",i1);

    let i2 = i1;
    // but this doesn't. w???
    println!("i1: {}",i1);
    println!("i2: {}",i2);
}