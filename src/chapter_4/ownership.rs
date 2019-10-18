pub fn ownership() {
    let word = String::from("hello world");
    let word = "hello word";
    let a = first_world(&word);
    // word.clear();
    println!("first world {}", a);
}

fn first_world_wrong(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn first_world(word: &str) -> &str {
    let bytes = word.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

        if item == b' ' {
            return &word[0..i]
        }
    }
    &word[..]
}
