
//1. rust中的范型 - T 用来替代类型
fn largest<T: PartialOrd+Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

use std::fmt::{Debug, Display};

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

//1. rust traits - 类似其他编程语言的接口概念， 但是还是有一些区别 , 比如如果实现了覆盖操作， 那么就不能调用默认的设置了， 没有类似java的 super. 这种方法
// rust 中的traits 其实实现了 定义一组类型的通用能力

pub trait Summary {
    //存在self 表示可以使用. 的方法调用
    fn summarize(&self) -> String;
    //prn 表示可以使用Struct 名称+:: 的方法调用
    fn prn()->String;
    //test
    fn ppp(){
        return;
    }
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
        Self::ppp();//tod Self 关键字可以自己引用到
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

// traits 作为参数传入 注意这个地方， 要使用impl标记一下
pub fn test3(summary: &impl Summary){
    summary.summarize();
    //这种方法有一个弊端，就是：： 方法没办法使用
    //summary::
}

//使用范型的方法就能直接使用：： 操作运算符号了， 同样也推荐这种方法
pub fn test4<T: Summary>(item :&T){
    T::prn();
    item.summarize();
}

trait TestTrait{
    fn tttt();
}

//使用T 表示同时实现了多个trait
fn test7<T: TestTrait+Summary>(item:&T){}

//可以使用where 关键字来缩短T的描述长度
fn test6<T,Z>(item :&T) where T:TestTrait+Summary,Z:TestTrait+Debug{}

//3. rust 的返回值trait 这个地方要注意下， 针对rust的返回trait的情况， 如果是impl trait的时候， 必须保证返回值的类型一直 todo 有办法绕开待定
fn returns_summarizable(switch: bool) -> impl Summary {
    if switch {
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    } else {
        //todo 这个地方要注意下， 针对rust的返回trait的情况， 如果是impl trait的时候， 必须保证返回值的类型一直
        // Tweet {
        //     username: String::from("horse_ebooks"),
        //     content: String::from(
        //         "of course, as you probably already know, people",
        //     ),
        //     reply: false,
        //     retweet: false,
        // }
        NewsArticle {
            headline: String::from(
                "Penguins win the Stanley Cup Championship!",
            ),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from(
                "The Pittsburgh Penguins once again are the best \
                 hockey team in the NHL.",
            ),
        }
    }
}


struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Pair<T> {
        Pair { x, y }
    }
}

impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

//5. rust trait 指定高级运用 ,  为指定范型的T提供方法，同trait基本使用

impl <T:Display+Summary> TestTrait for T{
    fn tttt() {
        todo!()
    }
}