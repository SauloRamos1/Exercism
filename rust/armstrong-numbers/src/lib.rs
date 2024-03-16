pub fn is_armstrong_number(num: u32) -> bool {
   
    let str = num.to_string();
    let len = str.len();

    let mut sum = 0;

    for c in str.chars() {
        let digit = c.to_digit(10);
        sum += digit.unwrap().pow(len as u32);
    }
    if sum == num {
        return true;
    }
    return false;
}
