use std::fmt;

fn main() {
    // working on i32
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("The largest number is {}", result);

    // working on char
    let char_list = vec!['y', 'm', 'a', 'q'];

    let result = largest(&char_list);
    println!("The largest char is {}", result);

    // working on String
    let string_list = vec![
        String::from("abc"),
        String::from("def"),
        String::from("ada"),
        String::from("wxyz"),
        String::from("z"),
    ];

    let result = largest(&string_list);
    println!("The largest String is {}", result);

    // lifetimes
    let s1 = "something".to_string();
    let s2 = "something longer".to_string();
    let result = longer_string(&s1, &s2);
    println!("The longer string is {}", result);
}

fn largest<T: PartialOrd + fmt::Display>(list: &[T]) -> &T {
    let mut max = &list[0];

    for item in list.iter() {
        if item > max {
            println!("{} is larger than {}", item, max);
            max = item;
        }
    }

    max
}

fn longer_string<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
