#[derive(Debug)]
enum Message {
    Text { from: String, to: String }, //anomymous struc
    Empty,
}
impl Message {
    fn call(from: String, to: String) -> Message {
        Message::Text { from, to }
    }
}

enum Maybe<T> {
    Full(T),
    Empty,
}
pub fn enums() {
    let a = Maybe::Full("a");
    match a {
        Maybe::Full(v) => println!("value is: {}", v),
        Maybe::Empty => println!("the value is empty"),
    };
    let x = Some("value");
    println!("x is {}", x.is_some());
    let text_length: Option<usize> = x.as_ref().map(|s| s.len());
    println!("size of x is {:?}", text_length);
    let m = Message::call(String::from("fr@gmail.com"), String::from("gt@gmail.com"));
    println!("message: {:?}", m)
}
