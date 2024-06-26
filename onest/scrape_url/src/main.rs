use std::fs;
fn main() {
    let url = "https://www.rust-lang.org/";
    let output = "rust.md";
    println!("fetching url: {}",url);
    let body = reqwest::blocking::get(url).unwrap().text().unwrap();
    println!("converting html2md");
    let md = html2md::parse_html(&body);
    fs::write(output,md.as_bytes()).unwrap();
    println!("converted md has saved in {}",output);
}
