// loop死循环
fn fib_loop(n:i32) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2;
    loop {
        let c = a+b;
        a = b;
        b = c;
        i+=1;
        if i >= n {
            break;
        }
    }
    b
}
// while条件
fn fib_while(n:i32) -> i32 {
    let (mut a,mut b,mut i) = (1,1,2);
    while i<n {
        let c = a+b;
        a = b;
        b = c;
        i+=1;
    }
    b
}
// for迭代
fn fib_for(n:i32) -> i32 {
    let (mut a,mut b) = (1,1);
    // _i 忽略i
    for _i in 2..n {
        let c = a+b;
        a = b;
        b = c;
    }
    b
}
fn main() {
    println!("{},{},{}",fib_loop(10),fib_for(10),fib_while(10));
}
