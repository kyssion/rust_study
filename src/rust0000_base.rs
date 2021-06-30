
//指定变量名称的方法
pub fn test(){
    let a :i32  = 12;
    let mut b : i32 = 123;
    // b = "" 这种方法不能支持的变量名称重新制定
    const p : i32 = 123;
    let b = ""; //  合法的相当于变量名称重用

    // 多个返回值使用元祖进行支持
    let e :(i32,i32,&str) = (1,2,"ffff");
    let b = e.0;
    let c = e.1;
    let d = e.2;

    // 数组
    let mut item = [1,2,3,4,5];
    let mut c = item[0]; // 这里时直接赋值了
    item[2] = 123;
}


pub fn test2(){
    //语句 理解， 有{} 包裹的东西 ， 一些表达式或者类似表达式的东西
    let p = {
        let a = 123;
        let b = 223;
        a+b
    };
    println!("{}",p)
}

// 支持i32 返回
pub fn test3() -> i32 {
    return 123;
}

pub fn test4(){
    //loop 是死循环 ， 主要这个不是表达式子
  // let a =  loop{123};//这种写法会报错 =
    // if else 是表示
   let a=  if true{1} else  if false {2} else {3};
    let mut a = 1;

    // while 也不是表达式
    while a<10{
        a=a+1
    }

    //for 循环

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {// for循环的语法重新设置
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}