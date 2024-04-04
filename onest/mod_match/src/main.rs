// 模式匹配 针对struct和enum
// struct User {
//     name: String,
//     id: i32,
// }
// match
// fn ut(u: &User) {
//     match u {
//         name => println!("{}", n),
//         id => println!("{}", t),
//     }
// }
enum Event {
    Join((String,i32)),
    Leave((String,i32)),
}
// match
fn t1(e:&Event) {
    match e {
        Event::Join((m,n)) => println!("{},{}",m,n),
        Event::Leave((m,_n)) => println!("{}",m),
    }
}
// if let
fn t2(e:&Event) {
    if let Event::Leave((_,n)) = e {
        println!("{}",n);
    }
}
fn main() {
    // let u = User{
    //     name:"rshulabs".to_string(),
    //     id:22
    // };
    // ut(&u);
    let e1 = Event::Join(("join".into(),22));
    let e2 = Event::Leave(("leave".into(),30));
    t1(&e1);
    t1(&e2);
    t2(&e2);
}
