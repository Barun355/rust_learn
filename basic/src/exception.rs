
pub fn divide(num1: i32, num2: i32) -> Result<f64, String> {
    
    if num2 == 0 {
        return Err("ZeroDivision Error".to_string());
    }
    let ans: f64 = (num1 / num2).into();

    return Ok(ans);
}
