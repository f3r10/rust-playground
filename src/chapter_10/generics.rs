use crate::chapter_10::traits::Summary;

pub fn generics() {
    println!("generics");
    let point = Point { x: 5, y: 10};
    println!("point: {:#?}", point);
    println!("point: {:#?}", point.summarize());
    let point2 = PointM { x: 5, y: 5.5};
    println!("point2: {:#?}", point2);
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);
}

 fn largest<T: PartialOrd>(list: &[T]) -> &T {
     let mut largest = &list[0];

     for item in list.iter() {
         if item > largest {
             largest = item;
         }
     }
     &largest
 }

fn largest_with_copy<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];

    for &item in list.iter() {
        if item > largest {
            largest = item;
        }
    }
    largest
}
#[derive(Debug)]
struct Point<T> {
    pub x: T,
    pub y: T
}

impl Summary for Point<i32> {
    fn summarize(&self) -> String {
        format!("({}, {})", self.x, self.y)
    }

}

#[derive(Debug)]
struct PointM<T, U> {
    x: T,
    y: U
}

