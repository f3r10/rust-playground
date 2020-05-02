// a lifetime is neccesary
// struct InvalidUser{
//     name: &str,
//     age: i32,
//     status: bool
// }
#[derive(Debug)]
struct User {
    name: String,
    age: i32,
    status: bool,
}

#[derive(Debug)]
pub struct Rectangle {
    pub width: i32,
    pub height: i32,
}
impl Rectangle {
    fn area(&self) -> i32 {
        self.width & self.height
    }

    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
impl Rectangle {
    fn square(size: i32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}

pub fn structs() {
    let user = User {
        name: String::from("Nando"),
        age: 28,
        status: true,
    };
    let name = user.name;
    println!("name = {}", name);
    let mut user_mut = User {
        name: String::from("Nando"),
        age: 28,
        status: true,
    };
    user_mut.name = String::from("Fernando");
    println!("name = {}", user_mut.name);
    let user3 = build_user(String::from("Fernando L."), 29);
    let user4 = User {
        name: String::from("Roberto"),
        ..user3 // complete with the other user struct
    };
    println!("User 4 {:#?}", user4);

    // let user5 = InvalidUser {
    //     name: "Fernando Roberto",
    //     age: 29,
    //     status : false
    // }; -> a lifetime is neccesary
    let r1 = Rectangle {
        width: 32,
        height: 32,
    };
    let r2 = Rectangle {
        width: 48,
        height: 58,
    };
    println!("rectangle are is {:?}", r1.area());
    println!(
        "r1: {:#?} can hold r2: {:#?} = {}",
        r1,
        r2,
        r1.can_hold(&r2)
    );
    let square = Rectangle::square(3);
    println!("square {:#?}", square);
}

fn build_user(name: String, age: i32) -> User {
    // User {
    //     name: name,
    //     age : age,
    //     status: true
    // } -> shorter
    User {
        name,
        age,
        status: true,
    }
}
