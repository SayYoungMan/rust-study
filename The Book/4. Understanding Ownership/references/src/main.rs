fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("The length of '{}' is {}.", s1, len);

    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);

    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    // variables r1 and r2 will not used after this point
    let r3 = &mut s; // no problem
    println!("{r3}");
}

fn calculate_length(s: &String) -> usize {
    // s is a reference to a String
    s.len()
} // Here s goes out of scope. But because it does not have ownership of what it refers to, it's not dropped.

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}
