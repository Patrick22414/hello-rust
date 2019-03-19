fn main() {
    let s1 = String::from("hello");

    let (s1, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len() returns the length of a String

    (s, length)
}

fn function(immut_ref, $mut_ref, &mut_copy) {

}