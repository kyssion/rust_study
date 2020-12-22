use std::intrinsics::arith_offset;

//定义一个基本rust结构体
struct User{
    user_name:String,
    email:String,
    sign_in_count: u64,
    active: bool
}

//元组结构体，没有变量名称只有值，通过给User进行使用
struct YStruct(i32, String, f64);

//空结构体 - rust 允许不存在任何数据的结构体
struct YStructEmpty();

pub fn test_01(){
    // 初始化 结构体
    //todo 注意 一旦结构体被定义为可变的那么所有的数据的都将可变
    let mut user = User{
        user_name:String::from("wang"),
        email: String::from("kyssion@kys.com"),
        active:false,
        sign_in_count:1
    };
    //修改结构体的信息
    user.email = String::from("kys@kyssion.com");

    //todo 结构体简易构建方法
    let mut user_temp = test_02(String::from("kyssion@kys.net"),String::from("jetty"));

    //todo 通过一个结构体快速构建另一个结构体
    let mut user = User{
        active:true,
        ..user_temp  // todo 这里rust 会自动的扫描其他的结构体，找到当前结构体不存在的字段，并通过其他结构体初始化当前结构体
    };

    //todo 元组结构体的初始化
    let mut item_y = YStruct(12, String::from("test"), 1.2);
    let i = item_y.0;

    //todo struct 生命周期 - 暂时略过

    //
}

pub fn test_02(email : String,user_name:String)-> User{
    //rust中的可以直接使用的名称相同的字段来赋值结构体中名称相同的字段
    return User{
        email,
        user_name,
        active:false,
        sign_in_count:1
    }
}

//定义一个正方形结构体
#[derive(Debug)]//显示的开启 debug 功能
struct Rectangle{
    width:u32,
    height:u32
}

fn area(rectangle: &Rectangle)->u32{
    return rectangle.height*rectangle.width;
}

pub fn example(){
    let rect1 = Rectangle{
        width:12,
        height:10
    };
    let area = area(&rect1);
    println!("{:#?}",rect1);//todo 使用{:#?}格式化输出 struct中的内容 只在debug模式中有效
}

