
fn longest_substring(s:String)->i32{
    let mut longest:i32 = 0;
    let mut actual_string:String = String::from("");
    for char in s.chars() {
         if actual_string.contains(char) {
             longest = actual_string.len() as i32
         }else {
             actual_string.push(char)
         }
    }
    println!("{}",actual_string);
    longest
}

fn main() {
    let sentence = String::from("pwwkew");
    let result = longest_substring(sentence);
    println!("{}",result)
}
