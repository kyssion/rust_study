
//1. rust中的范型 - T 用来替代类型
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

//2. 范型支持struct 结构体
struct Point<T> {
    x: T,
    y: T,
}

//2.1 rust 针对struct 定义的方法 ， 可以做到指定类型的存在各自参数的能力
impl<T> Point<T>{
    fn show(&self)->&T{
        return &self.x
    }
}

impl Point<i32>{
    fn show_i32(&self)->i32{
        return self.x
    }
}

fn struct_type(){
    let a = Point{
        x:1,y:2
    };
    let b= Point{
        x:"fff",y:"xxxx"
    };
}
//3. 枚举类型也支持范型

enum Test<T>{
    One(T),
    None
}

fn enum_type(){
    let a = Test::One(String::from("ffff"));
}

//1. rust traits - 类似其他编程语言的接口概念， 但是还是有一些区别
// rust 中的traits 其实实现了 定义一组类型的通用能力

pub trait Summary {
    //存在self 表示可以使用. 的方法调用
    fn summarize(&self) -> String;
    //prn 表示可以使用Struct 名称+:: 的方法调用
    fn prn()->String;
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }

    fn prn() -> String {
        return String::from("prn new article")
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
    fn prn() -> String {
        return String::from("prn tweet")
    }
}
//2.rust 针对 traits 使用的限制 ： 不能出现三处差异， 比如 traits定义的问题，需要升级的struct位置和定义引用的位置存在3不同
impl Summary for String{
    fn summarize(&self) -> String {
        return String::from(self);
    }

    fn prn() -> String {
        return String::from("z")
    }
}


pub fn test(){
    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    };

    println!("1 new tweet: {}", tweet.summarize());
}
