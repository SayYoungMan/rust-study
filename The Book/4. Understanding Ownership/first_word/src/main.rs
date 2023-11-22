fn main() {
    let mut s = String::from("hello world");

    let word = wrong_first_word(&s); // word will get the value 5

    s.clear(); // this empties the string, making it equal to ""

    println!("s is:{s} and word is:{word}");
    // word still has the value 5 so the word is now invalid.

    let my_string = String::from("hello world");
    // 'first_word' works on slices of Strings
    let _word = first_word(&my_string[0..6]);
    // Also works on references to Strings, which are equivalent to whole slices of Strings
    let _word = first_word(&my_string);

    let my_string_literal = "hello world";
    // works on slices of string literals
    let _word = first_word(&my_string_literal[..]);
    // Because string literals are string slices:
    let _word = first_word(my_string_literal);
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn wrong_first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}
