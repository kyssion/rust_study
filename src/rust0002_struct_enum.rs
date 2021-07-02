
struct  User{
    name : String,
    age : i32
}

struct Item(i32,i32,i32);

pub fn test(){
    //1. rust中的结构体声明的时候需要保证所有的变量都被初始化数字
    let user = User{
        name : String::from("jack"),
        age: 12
    };

    //2.rust 结构体也有可变性
    let mut user = User{
        name : String::from("jack"),
        age: 12
    };
    user.name = String::from("Tom");

    //3. rust 展开初始化
    let mut user = User{
        name:String::from("jason"),
        ..user
    };

    //4. rust 变体元组结构体支持
    let mut item = Item(32,32,32);
    let p = item.0;


    let c = (1,3,2);
    struct Type();//4.1 rust 声明一个空的结构体 ， 用来表示类型
    //5. todo rust 结构体， 中的引用 - 需要标记生命周期


    //6. rust struct 相同变量名称快速初始化
    let name = String::from("ffff");
    let age = 123;
    let c = User{
        name,age
    };
}

impl User{
    //用偶self 标记的表示 结构体方法
    fn show_name(&self){
        println!("{}",self.name)
    }

    // 没有self 标是关联函数， 需要使用impl块名称：： 方法名称来使用 ，  类似String::from
    fn init()->User{
        return User{
            name:String::from(""),
            age:0
        }
    }
}

//1. rust enum 可以类型不同
enum IpAddr{
    IPV4(i32,i32,i32,i32),
    IPV6(String)
}
//2. rust 的enum 支持任意类型的数据
enum Message{
    Quit,
    Move {x:i32,y:i32}, //  3. rust enum 支持匿名结构体
    Write (i32),
    ChangeColor
}

pub fn test_enum(){
    //4. rust enum 不同的数据类型状态但是类型可以不同
    ans(Message::Quit);
    ans(Message::Write(13));
    //5. rust enum 初始化
    let item = Message::Move {x:123,y:334};
}

fn ans(item:Message){
}

pub enum Coin{
    One,
    Two,
    Three,
    Four(CoinStatus),
    Five {x:i32}
}

//todo 这个注解可以让println打印出来，其他的功能待定
#[derive(Debug)]
pub enum CoinStatus{
    Use,
    NoUser
}

fn match_support(){

}

pub fn test3(c : Coin)-> i32{
    match c {
        Coin::One=> 1,
        Coin::Two=>2,
        Coin::Three=>3,
        Coin::Four(status)=>{ //6. rust 可以直接解析数据
            println!("{:?}",status);
            4
        },
        Coin::Five{x} =>{
            println!("{:?}",x);
            x
        }
    }
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),//option 相当于一个特殊的枚举， 返回一个新的数据
    }
}

//7. rust if let => 简化版match ， 下面两种写法等价
fn if_let(c : coin){

    let p = match c {
        Coin::Five {x}=>x,
        _=>0
    };

    let p = if let Coin::Five {x}=c{
        x
    }else{
        0
    };
}