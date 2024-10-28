#![warn(clippy::all)]
#![warn(clippy::pedantic)]
#![warn(clippy::restriction)]
#![warn(clippy::nursery)]
#![warn(clippy::cargo)]

use std::io::stdin;

struct Visitor {
    name: String, 
    greeting: String,

}

impl Visitor {
     fn new (name: &str, greeting: &str) -> Self{

        Self{
            name: name.to_lowercase(),
            greeting: greeting.to_lowercase(),
        }
     }

     fn greet_visitor(&self){
        println!("{}", self.greeting);
     }
}


fn what_is_your_name() -> String {
    println!("Hey ! What is your name ?");
    let mut your_name = String::new();
    let failed_to_read_line = "Failed to read line !";
    let _read_line = stdin()
        .read_line(&mut your_name)
        .expect(failed_to_read_line);

    your_name.trim().to_lowercase()
}

fn main() {
    let name = what_is_your_name();
    //Replacing the {} placeholder with {:?} uses the debug placeholder.
    println!("Hello ! {:?}", name);

    /*
    let vistor_list = ["abc", "def", "ghi"];
    let mut invited = false;

    for visitor in &vistor_list {
        if visitor == &name {
            invited = true;
        }
    }

    if invited {
        println!("Welcome to the event !");
    } else {
        println!("Sorry ! You are not invited !")
    }
    */

    let visitor_list = [
        Visitor::new("abc", "how are you doing !"),
        Visitor::new("cde", "good morning !"),
        Visitor::new("rfg", "Thanks for coming !"),
        Visitor::new("sdf", "Welcome !"),
    ];

    let known_visitor = visitor_list.iter().find(|visitor|visitor.name == name);

    match known_visitor {

        Some(visitor) => visitor.greet_visitor(),
        None => print!("You are not on the Visitor's list. Please leave the premise !")
    }
}
