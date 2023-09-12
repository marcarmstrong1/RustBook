fn main() {
    //COnvert String to Pig Latin
    let mut s = String::from("first");
    let mut s2 = String::from("apple");
    let mut s3 = String::from("hello");
    let mut s4 = String::from("world");

    println!("{} in Pig Latin is {}", &s, pig_latin(&mut s));
    println!("{} in Pig Latin is {}", &s2, pig_latin(&mut s2));
    println!("{} in Pig Latin is {}", &s3, pig_latin(&mut s3));
    println!("{} in Pig Latin is {}", &s4, pig_latin(&mut s4));
}

fn pig_latin(s: &mut String) -> String {
    let mut pig_latin = String::new();
    let mut first_char = String::new();
    let mut rest_of_word = String::new();
    let vowels = String::from("aeiou");

    first_char.push(s.chars().nth(0).unwrap());
    rest_of_word.push_str(&s[1..]);

    if vowels.contains(&first_char) {
        pig_latin.push_str(&s);
        pig_latin.push_str("-hay");
    } else {
        pig_latin.push_str(&rest_of_word);
        pig_latin.push_str("-");
        pig_latin.push_str(&first_char);
        pig_latin.push_str("ay");
    }

    pig_latin
}
