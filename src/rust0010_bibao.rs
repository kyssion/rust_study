use std::*;

fn simulated_expensive_calculation(intensity:i32) -> i32{
    println!("start");
    thread::sleep(time::Duration::from_secs(2));
    return intensity2
}

fn test1(){
    let num = 123;
    //定义一个闭包
    //todo  注意这种闭包有一个问题，没有制定变量类型
    //todo 所以，rust 将会用的第一次调用的类型当作这个闭包的类型
    let bItem = |params|{
        params
    };

    let bItem = |params|params;// 简化写法

    //todo 定义 闭包类型 和函数一样
    let bItem = |param : i32|->i32{
        param
    };

    //todo 支持范型

}