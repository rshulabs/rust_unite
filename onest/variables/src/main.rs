// use num::complex::Complex;
// // 变量
// struct Struct {
//     e:i32
// }

// fn add(a:u32) ->u32 {
//     if a<10 {
//         return a+100;
//     }
//     a-6
// }

fn main() {
    // let mut v = 1;
    // v = 2;
    // println!("{}", v);

    // let a = 1;
    // println!("{}", a);
    // let _b = 2;

    // 解构 元组 切片 结构体
    // let (a,b,c,d,e);
    // (a,b) = (1,2);
    // [c,..,d,_] = [3,4,5,6,7,8];
    // Struct {e,..} = Struct{e:10};
    // assert_eq!([1,2,3,4,5],[a,b,c,d,e]);

    // let arr = [
    //     5.0,
    //     432f32,
    //     7.0f32,
    // ];
    // println!("{:.2}",arr[1]);

    // let a = 9;
    // let a = a+1;
    // {
    //     let a = 99;
    //     println!("{}",a);
    // }
    // println!("{}",a);

    // for i in 'A'..='Z' {
    //     println!("{}",i);
    // }

    // num库
    // let a = Complex{re:2.2,im:-0.9};
    // let b = Complex::new(55.8,9.87);
    // let res = a+b;
    // println!("{}  {}",res.re,res.im);
    
    // 语句和表达式
    // let _a = println!("{}","vvv");

    // let t = add(5);
    // println!("{}",t);

    // tes();

    // 作用域自上而下
    // let mut a = 9;
    // println!("{}",a);
    // {
    //     a = 44;
    //     println!("{}",a);
    // }
    // println!("{}",a);
}

// 发散函数 无返回值
// fn tes() -> ! {
//     panic!("你已经到了穷途末路，崩溃吧！");
// }