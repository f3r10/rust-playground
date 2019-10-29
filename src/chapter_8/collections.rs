pub fn vectors() {
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);
    let first: &i32 = &mut v[0];
    println!("first element {}", first);

    let v2 = vec![1,2,3];
    let second: &i32 = &v2[1];
    println!("second element inmutable vec {}", second);

    // mutable reference brokes the dinamic allocation 
    // let mut v3 = vec![1,2,3,4];
    // let f1  = &v3[0];
    // v3.push(6);
    // println!("The first element of v3 is: {}", f1);
    
}
