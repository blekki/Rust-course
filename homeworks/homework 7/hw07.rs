
// fn to_lower(string: String) -> String
// {
//     return string.to_lowercase();
// }

// fn to_upper(string: String) -> String
// {
//     return string.to_uppercase();
// }

fn change_case(string: String)
{
    let mut new_str :String = "".to_string();
    for c in string.chars() {
        if c.is_lowercase() {
            new_str.push_str(c.to_uppercase().to_string());
        }
        else {
            new_str.push_str(c.to_lowercase().to_string());
        }

        new_str.push_str(&c.to_string());
    }
    println!("{}", new_str);
}

fn main(){
    
    let string = "Hello".to_string();
    
    // println!("{}", string);
    // println!("{}", to_lower(string.clone()));
    // println!("{}", to_upper(string.clone()));

    change_case(string.clone());

}