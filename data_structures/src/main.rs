use std::fmt;

//Note: 
//Methods are defined within the context of a struct
// and their first parameter is always “self”.

struct User {
    username: String, 
    counter: i64,
    active: bool
}
impl User {
    //This is a method, not a function
    fn multiply_counter_and_return(&self) -> i64{
        return self.counter * 2;
    }
}

pub trait SomePrinter {
    fn summarize(&self) -> String;
}


//now User implements fmt::Debug for pretty printing
impl fmt::Debug for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("User")
         .field("username", &self.username)
         .field("counter", &self.counter)
         .field("active", &self.active)
         .finish()
    }
}

impl SomePrinter for User {
    fn summarize(&self) -> String {
        return String::from("str");
    }
}

fn main() {
    let user1 = User {
        username : String::from("Mike"),
        counter: 3, 
        active:false
    };
    
    println!("user1 username: {}", user1.username);
    println!("user1 : {:#?}", user1);

    let user2 = User {
        username: String::from("Vasileios"),
        ..user1
    };

    println!("user2 : {:#?}", user2);

    let mc = user2.multiply_counter_and_return();
    println!("multiplied counter: {}",mc);
}
