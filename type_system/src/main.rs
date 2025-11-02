fn safe_add(a: &str, b:&str) -> Option<i32>{
    // Understanding Option
    let a_num = a.parse::<i32>().ok()?;
    let b_num = b.parse::<i32>().ok()?;
    return Some(a_num+b_num);
}

fn safe_add2(a: &str, b: &str) -> Result<i32, String> {
    let a_num = a.parse::<i32>().map_err(|_| "Invalid string".to_string())?;
    let b_num = b.parse::<i32>().map_err(|_| "Invalid string".to_string())?;
    return Ok(a_num+b_num);
}

fn main() {
    let a = String::from("4");
    let b = String::from("9k");

    // Understanding match with option
    let sum = match safe_add(&a, &b) {
        Some(sum) => sum,
        None => 0
    };

    // Understanding match with result
    let sum2 = match safe_add2(&a, &b) {
        Ok(sum)=> sum,
        Err(_) => -1
    };

    println!("{}", sum2);
}
