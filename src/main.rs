fn get_first_word(s: &str) -> &str {
    for (i, c) in s.chars().enumerate() {
        if c == ' ' {
            return &s[..i];
        }
    }
    return s;
}

fn main() {
    let s = "Hello World!!! This is a sentence.";
    let s = String::from(s);
    let first_word = get_first_word(&s);
    let l = first_word.len();
    println!("{first_word} {l}");
    println!("{s}");
    let w = "what the hell?";
    let haha = get_first_word(w);
    println!("{haha}");
    println!("{w}");
}
