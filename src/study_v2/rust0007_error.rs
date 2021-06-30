//todo rust 错误处理

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::io;
use std::fs;

fn test_panic(){
    //rust 提供了一个panic 宏，来抛出不可修正的异常信息
    panic!("this is a panic")
    //这种错误属于不可恢复错误
}

fn test_result() -> Result<String,io::Error>{
    //rust 中的可回复错误
    //rust 中使用的Result<T,E> 结构体来描述一个error
    let f = File::open("xxx.test");
    //todo 异常接收场景样例
    match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("xxx.test") {
                Ok(fc) => fc,
                Err(e) => panic!("error")
            },
            other_error => panic!("other error")
        }
    };

    //todo 高级写法 - 使用闭包
    let f = File::open("ffff").map_err(|error|{
        if error.kind() == ErrorKind::NotFound{
            File::create("hello.txxt").unwrap_or_else(|error|{
                panic!("error")
            })
        }else{
            panic!("this is error")
        }
    });

    //todo 快速一场信息导出 - 二者几乎等价， 只不过expect 添加了错误的详细信息
    let f = File::open("fff").expect("this is error");
    let f = File::open("fff").unwrap();


    //rust 错误传播 - 其实就是在代码逻辑中是使用Result返回，然后函数定义意识使用的Result 进行定义的


    // rust 拥有一个非常好用的模板方法来快速编写模板异常链调用方法
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    // return Ok(s)
    //todo 注意，被？命中的返回值会显示的被from 函数处理
    // -- todo -- 黑科技，使用？实现错误链，链式调用

    let mut str = String::new();
    File::open("hello.txt")?.read_to_string(&mut str);

    fs::read_to_string("fff")
}