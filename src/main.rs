fn longest_substring(s:String) ->i32{
    let mut longest:i32 = 0;
    let mut actual_string:String = String::from("");
    for char in s.chars() {
        if s.len() == 1 {
            longest = 1;
            actual_string = String::from(char);
        }
        if char.eq(&' ') {
            longest = 1
        }
        if actual_string.contains(char) {
            if longest < actual_string.len() as i32 {
                println!("{}",actual_string);
                longest = actual_string.len() as i32;
            }
            if actual_string.ends_with(char) {
                actual_string = String::from(char)
            }else {
                actual_string = String::from("");
            }
         }else {
             actual_string.push(char);
         }
    }
    longest
}

fn main() {
    let sentence = String::from("au");
    let result = longest_substring(sentence);
    println!("{}",result)
}
