//集合

use std::ops::Add;

//动态数组
fn vec_test(){
    //创建动态数组 - rust 的 vec数组可以动态的增加指定类型的数据，而不需要关系空间是否够用
    let mut v: Vec<i32> = Vec::new();
    //通过宏 - 快速创建有初始值的动态数组
    {
        let mut v = vec![1,2,3,4];
    } // 注意，动态数组在这个时候将会被销毁 因为离开了作用域

    //获取数组中的内容

    let i = &v[1]; // 这个会有越界 问题
    let i = v.get(1); // 这个不会有
    //todo 这里要注意rust的引用规则

    //修改值和遍历
    for i in &mut v{
        *i+=5;//这里解引用并且增加值
    }

    //使用枚举来让rust的数组支持不同的类型
    enum SpreadsheetCell{
        Int(i32),
        Float(f64),
        Text(String)
    }
    let row = vec![SpreadsheetCell::Int(3),SpreadsheetCell::Float(5.4),SpreadsheetCell::Text(String::from("ffff"))];
}


// rust 字符串
fn test_string(){
    //todo 创建一个空字符串
    let mut s  = String::new();

    let str = "this is a str";
    let str = str.to_string();//todo 如果是实现了 display trait 类型的话，就可以调用to_string 方法来获取进行序列化的之后的值
    let str = "this is a str".to_string();
    let str = String::from("this is a str");
    //todo  一个问题的- String 类型和str 类型之间的区别是什么

    //todo 可以使用 format! 宏来拼接字符串 或者直接使用+ 号来进行操作

    let mut stringVal1 = String::from("sdfsdf");
    let str1 = String::from("ffffff");
    stringVal1 = stringVal1+"ffff";
    //todo 在rust中的+ 将会被替换成stringVal1.add("xxx")
    String::from("ffff").add("ffff");//todo add函数将会吧当前函数的所有权传入并且在上层进行修改，所以调用add的方法将会失效
    stringVal1.push_str(str1.as_str());

    //todo rust 支持 format 这个和+不同，不过获取字符的所有权
    let mut string1 = String::from("A");
    let mut string2 = String::from("B");
    let mut string3 = String::from("C");
    let s = format!("{}-{}-{}",string1,string2,string3);

    //todo rust 的 字符类型处理方法
    let mut stringItem  = String::from("AABBCCDDEEFF");
    //todo rust中任意一个字符都是一个unicorn

}