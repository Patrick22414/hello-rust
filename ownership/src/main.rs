fn main() {
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);

    println!("The length of '{}' is {}.", s1, calculate_length2(&s1));

    println!("I can still use '{}'", s1);
    
    //**************************************************/
    let mut s2 = String::from("hello2");

    {
        let r1 = &mut s2;
        change(r1);
        println!("At inner scope, after change: {}", &(&r1)); // What is this puzzling thing??
        println!("At inner scope, after change: {}", s2);
    }

    let r1 = &mut s2;
    println!("At outer scope, after inner scope change: {}", r1);

    //****************** Slices! **********************/
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();

    (s, length)
}

fn calculate_length2(s: &String) -> usize {
    s.len()
}


fn change(some_string: &mut String) {
    some_string.push_str(", world");
}