pub fn lifetimes() {
    let string1 = String::from("long string is long");
    let result;
    {
        // let string2 = String::from("xyz");
        let string2 = "xyz"; // it has the static lifetime reference so lives for ever
        result  = longest(string1.as_str(), string2);
    }

    println!("longest: {:?}", result);

}

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
