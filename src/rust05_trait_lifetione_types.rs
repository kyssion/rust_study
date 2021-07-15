
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

//----- lifeStyle - 生命周期
//1. rust 的生命周期其实是针对 一个变量有效范围的标记

fn test1(){
    //一个变量可以使用的范围是不能超过他生命周期的范围的
    {
        let mut r;        // ---------+-- 'a
                                //          |
        {                       //          |
            let x = 5;     // -+-- 'b  |
            r = &x;             //  |       |
        }                       // -+       |
        r=&123;                 //          |
        println!("r: {}", r);   //          |
    }                           // ---------+
}

//2. rust 函数使用时候的生命周期
//如果不指定生命周期会报错，因为传入的参数有两个引用， 这两个引用可能有两种不同的生命周期，这种情况下可能会导致rust无法确认返回值的生命周期是否有效从而导致悬空引用

/// 如果rust 返回的是一个借用 - 这个时候， rust 需要指定生命周期，需要使用生命周期函数
/// 注意 rust的生命周期是 - 针对rust 返回值指定的方法
/// 注意， rust 这里生命周期类似一种分组的概念， 多个相同生命周期标记的方法会让最终的生命周期标记的生命周期长度最短的那个
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn test2(){
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        /// 这里使用longest方法的时候传入的两个参数的生命周期不同
        /// 但是rust方法中生命周期签名使用的都是 ‘a
        /// rust这里处理方法使用两个生命周期最短的那个来指定
        result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
    ///这里存在问题， result的生命周期和这里生命周期不匹配
    //println!("The longest string is {}", result);
}

/// 注意 rust 结构体中如果需要使用引用的时候， 需要增加生命周期参数
struct ItemInfo<'a>{
    part : &'a str,
    init: &'static str // 特殊的static 生命周期标记， 这个标记表示，这个应用在函数生命周期整个生命范围中都有效
}

impl ItemInfo{
    fn init<'a>(&self, i: &'a str) -> &'a str {
        return i;
    }
    //3. rust 的生命周期使用的 如果进行指定的话， 如果有self 这种场景， 默认就是使用self来作为默认的生命周期长度
    fn init2(&self, i: &str) -> &str {
        return i;
    }
}

/// 既指定生命周期， 又指定范型类型的写法
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str where T: Display, {
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}