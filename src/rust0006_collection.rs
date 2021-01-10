//集合

use std::ops::Add;
use std::collections::HashMap;

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
    //todo rust中任意一个字符都是一个unicode
    //todo rust中的string类型是一个vec<u8> 封装 - 当时一个字符并不一定只有8字符
    //todo rust中针对字符串使用下表操作的时候直接宝座

    //let p = string1[0];// 非法

    //todo 简单的讲其实是len的长度和字符的长度不同

    //todo string 类型有 chars和char_indices 方法返回 char类型数组
    for (size,c) in s.char_indices(){
        println!("{:?} - {:?}",size,c)
    }

    //todo 使用bytes方法获取byte类型的数组
    for c in s.bytes(){
        println!("{:?}",c as char)
    }
}

fn test_hash_map(){
    //todo 直接使用new的方法创建 hashmap
    let mut my_hash_map = HashMap::new();
    //todo 这里注意一下所有权的问题一旦键值对被插入了，所有权也就转移到了map中了
    my_hash_map.insert("key1", "value1");
    my_hash_map.insert("key2", "value2");

    //todo 使用数组列表的方法创建hashmap
    let mut keys = vec!["key1","key2"];
    let mut values = vec!["value1","value1"];
    let my_hash_map :HashMap<&&str,&&str> = keys.iter().zip(values.iter()).collect();
    //todo 这种方法以后在尽心研究


    //todo 获取值
    let mut my_hash_map = HashMap::new();
    my_hash_map.insert("key",0);
    let mut valueOption = my_hash_map.get("key");
    let mut value = *valueOption.unwrap();

    //todo rust map 遍历 - 注意这里要使用 引用，不然就所有权转移了
    for (key,value) in &my_hash_map{
    }

    let string = [String::from("aaa"),String::from("bbb"),String::from("ccc")];
    for str in string{
        //使用 entry+ or_insert  - 进行判断是否为空并且加入必要哦的值
        let value_str = my_hash_map.entry(str).or_insert(0);
        *value+= 1;
    }

}