pub fn strings() {
    let data: String = "initial contents".to_string();
    let s2 = String::from(" deleted.");
    let s3 = data + &s2;
    println!("final msg: {}", s3);
    let mut a = String::from("mutable string,");
    let s4 = " operation";
    add_string(&mut a, &s4);
    println!("a: {}", a);
    println!("s4: {}", s4);
    // println!("data: {}", data); add takes ownership of the first parameter
}

fn add_string(s1: &mut String, s2: &str) {
    s1.push_str(s2);
}
