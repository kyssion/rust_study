//rust 迭代器和闭包

pub fn test<T>(i:T){
    //1. 声明一个闭包 - 支持范型但是这个支持需要外部函数的支持， 同理生命周期也是一样的
    let test_fn = |num : T| ->T {
        return num;
    };

    //2. 函数变量
    let c= ppp;
    c(111);
    //3. 内部闭包其实可以使用类型推单来避免参数声明
    let test_fn = |num|{
        return num;
    };
    let b = test_fn(123);
    println!("{}",b);


    //4. 闭包获取上下文环境
    let num_info = 1233;
    let fn_test = || {
        println!("{}",num_info);
    };
    //5. 闭包和函数的三种实现trait
    //5.1 FnOnce todo 暂时不知道干啥的 感觉是使用move标记的变量信息

    //5.2 FnMut 可变的借用
    let mut string_item = String::from("this is test");
    let mut fn_mut = || {//生命闭包的时候 带上一个前缀mut 就能保证可变化了
        string_item.push_str("fffdsf");//todo 待确认。。 这里应该是rust 自己将所有权转移的变量变成引用类型修改了- 应该是编译器优化的
        println!("{}",string_item);
    };
    fn_mut();
    //5.3 Fn 不可变借用
    let num = 123;
    let fn_ = ||{
        println!("{}",num); // 不可变引用不能修改
    };
    fn_();
    //6. 闭包和move联合使用的时候 可以使用move 来转移变量的所有权 - ！！！ 注意 ， 这里是在定义的时候转移所有权的
    let x = vec![1,2,3,4,5];
    let fn_move = move ||{
        let y = x; // 这里会导致x在代码定义的时候转移到闭包里 todo 暂时不知道为啥有这个特性
        return 0;
    };
}

fn ppp<'a,T>(i : T){
    println!("dfsdfsf");
}