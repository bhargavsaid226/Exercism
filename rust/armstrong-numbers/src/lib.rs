pub fn is_armstrong_number(num: u32) -> bool {
    let mut sum: u32 = 0;
    let mut num_copy: u32 = num;
    let base: u32 = 10;
    let num_digits: u32 = count_digits(num);
    while num_copy != 0 {
        sum += (num_copy % base).pow(num_digits);
        num_copy /= base
    }
    println!("Sum is {}", sum);
    if sum == num{
       return true;
    }
    false
}
    
fn count_digits(mut num: u32) -> u32 {
    let mut num_digits: u32 = 0;
    while num != 0 {
        num_digits += 1;
        num /= 10;    
    }
    num_digits
}