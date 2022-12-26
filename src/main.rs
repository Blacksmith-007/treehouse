use std::io::stdin;

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String},
    Refuse,
    Probation,
}

struct Visitor {
    name: String,
    greeting: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, greeting: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            greeting: greeting.to_string(),
            action,
            age
        }
    }
    
    fn greet_visitor(&self) {
        println!("{}", self.greeting);
    }
}

fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
    your_name
    .trim()
    .to_lowercase()
} 

fn main() {
    let my_action = VisitorAction::AcceptWithNote{ note: "Give them a taco"};
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("steve", VisitorAction::AcceptWithNote{
            note: String::from("Lactose-free milk is in the fridge")
        } 15),
        Visitor::new("fred", VisitorAction::Refuse, 30),
    ];

loop {
    println!("Hello, what's your name?");
    let name = what_is_your_name();

let known_visitor = visitor_list
    .iter()
    .find(|visitor| visitor.name == name);

match known_visitor {
    Some(visitor) => visitor.greet_visitor(),
    None => {
        if name.is_empty() {
            break;
        } else {
            println!("{} is not on the visitor list.", name);
            visitor_list.push(Visitor::new(&name, "New friend"));
        }
    }
}
break;
}
println!("The final list of visitors:");
println!("{:#?}", visitor_list);

}
