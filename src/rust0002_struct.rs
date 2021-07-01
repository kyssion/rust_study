
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


}

impl User{
    //用偶self 标记的表示 结构体方法
    fn show_name(&self){
        println!(self.name)
    }

    // 没有self 标是关联函数， 需要使用impl块名称：： 方法名称来使用 ，  类似String::from
    fn init()->User{
        return User{
            name:String::from(""),
            age:0
        }
    }
}

pub fn test_func(){

}