


pub fn parse_int(input: &mut String) -> Option<i32> {
    match input.trim().parse::<i32>() {
        Ok(num) => {
            return Some(num);
        }
        Err(_) => {
            return None;
        }
    }
}


pub fn parse_char(input: &mut String) -> Option<char> {
    match input.trim().parse::<char>() {
        Ok(character) => {
            return Some(character);
        }
        Err(_) => {
            return None;
        }
    }
}
