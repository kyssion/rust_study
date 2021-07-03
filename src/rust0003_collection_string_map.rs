use std::collections::HashMap;

pub fn vec_test(){
    let mut vecItem:Vec<i32> = Vec::new();
    let mut vecItem = vec![1,2,3,4,5];
    vecItem.push(123);
    let a= vecItem.get(0);
    let a = vecItem[0]; //1. vec 可以通过 get 或者数组下标的方法获取 。 两种区别是， 下标引用了不是别的或出现panic get会返回None

    //2. vec 迭代方法
    for i in &vecItem{

    }
    for i in &mut vecItem{

    }
    // todo 注意这里有问题， for in 本质上是语法糖，是一个函数， 调用的时候所有权向下转移了
    for i in vecItem{

    }

    return
}

pub fn string_test(){
    let a = String::from("fsdf");
    let b = String::from("xxxx");
    let c= &b;
    let a = a+&b;
    let a = a+"--"+&b;
    let a = format!("{}--{}--{}",a,b,c); //1. string 可以使用 format 宏来强制的让格式字符串便利化
    string_f(&a);//todo rust 的deref coercing 特性， 这里会强制的将&String引用转化成&str

    // rust 字符串不支持索引操作 . 可以使用下面的方法获取索引
    let e = a.chars();
    e.count();
    let e = a.as_bytes();
    e.len();
}

pub fn string_f(strItem : &str) ->&str{
    strItem
}

pub fn map_f(){
    //1. map 的一般初始化
    let mut myMap = HashMap::new();
    myMap.insert(String::from("ffff"),1); //注意所有权的变化， 传入的值的所有权将会交给myMap
    myMap.insert(String::from("fffff"),2);

    //2. 使用语法糖初始化
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    //3. get
    myMap.get("ffff");

    //4. 遍历 --  问题和vec 便利一样， 注意使用引用的方法
    for (key,value) in &scores{

    }
    //5. 条件插入 - 注意这个地方返回的是当前值的可变引用
    myMap.entry(String::from("ffff")).or_insert(50);
}