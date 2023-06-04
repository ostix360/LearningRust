use std::fmt::Display;

struct Extract<'a> {
    part: &'a str,
}


fn main() {
    let str1 = String::from("abcd");
    let str2 = "xyz";

    let result = longest_announced(str1.as_str(), str2, String::from("This is the longest String !!!!"));
    println!("The longest is {}", result);


    let roman = String::from("Appelez-moi Ismaël. Il y a quelques années ...");

    let first_sentence = roman.split(".").next().expect(". don't found");
    let i = Extract { part: first_sentence };
}


fn longest<'a>(s1: &'a str, s2: &'a str) -> &'a str {
    if s1.len() > s2.len() {
        s1
    } else {
        s2
    }
}

fn longest_announced<'a, T>(x: &'a str, y: &'a str, announcement: T) -> &'a str
    where T: Display
{
    println!("Emergency /!\\ \n{}", announcement);
    longest(x,y)
}
