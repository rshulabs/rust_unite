#[derive(Debug)]
// 枚举
enum Gender {
    Unspecified = 0,
    Male = 1,
    Female = 2,
}

// 结构体元组
#[derive(Debug,Copy,Clone)]
struct UserID(u32);
#[derive(Debug,Copy,Clone)]
struct TopicID(u32);

// 结构体
#[derive(Debug)]
struct User{
    id:UserID,
    name: String,
    gender:Gender,
}
#[derive(Debug)]
struct Topic{
    id:TopicID,
    name: String,
    owner:UserID,
}

#[derive(Debug)]
enum Event {
    Join((UserID,TopicID)),
    Leave((UserID,TopicID)),
    Message((UserID,TopicID,String)),
}

fn main() {
    let zhangsan = User{id:UserID(250),name: "zhangsan".into(),gender:Gender::Male};
    let topic = Topic{id:TopicID(520),name:"test".into(),owner:UserID(250)};
    let j = Event::Join((zhangsan.id,topic.id));
    let l = Event::Leave((zhangsan.id,topic.id));
    let m = Event::Message((zhangsan.id,topic.id,"hello".into()));
    println!("e1 : {:?},e2 : {:?},e3 {:?}",j,l,m);
}
