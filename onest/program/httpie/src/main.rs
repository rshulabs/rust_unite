use clap::{AppSettings,Clap};

// 定义httpie主入口，包含若干子命令
// ///三斜杠注释是帮助文档 clap解析

/// httpie-the http cli with rust, now let we use it quickly?
#[derive(Clap,Debug)] 
#[clap(version = "1.0",author = "rshulabs <rshulabs@gmail.com>")]
#[clap(setting = AppSettings::ColoredHelp)]
struct Opts{
    #[clap(subcommand)]
    subcmd:SubCommand,
}

// 子命令分别对应不同http方法，目前仅支持get/post
#[derive(Clap,Debug)]
enum SubCommand {
    Get(Get),
    Post(Post),
}

// get子命令
/// feed get with an url and it will retrieve the response for you
#[derive(Clap,Debug)]
struct Get{
    /// http请求url
    url:String,
}

// post子命令
/// feed post with an url and optional key=value pairs.it will retrieve the response for you with json
#[derive(Clap,Debug)]
struct Post {
    /// http请求url
    url:String,
    /// http请求body
    body: Vec<String>,
}
fn main() {
    let opts:Opts = Opts::parse();
    println!("{:?}", opts);
}
