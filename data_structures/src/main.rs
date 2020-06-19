use std::fmt;

struct User {
    username: String, 
    counter: i64,
    active: bool
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
}
