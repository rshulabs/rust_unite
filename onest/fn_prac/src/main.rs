fn compute(val:i32,f:fn(i32)->i32) -> i32 {
    // return f(val);
    f(val)
} 

fn square(val:i32) -> i32 {
    // return val*val;
    val*val
}

fn main() {
    let s = compute(5,square);
    println!("the square is {}",s);
}
