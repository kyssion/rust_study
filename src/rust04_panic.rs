use std::fs::File;
use std::io::{ErrorKind, Read};
use std::error::Error;

pub fn test_panic(){
    //1. 抛出一个异常
    //panic!("fffff");
    //2. result 枚举来处理潜在错误信息 , 比如下面的File 的open方法
    let file = File::open("hello text");
    //2.1 可以使用match 方法处理 Result
    let file = match file {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            e => { //todo 这里是match的一个特殊的用法 - 可以制定一个变量名称来将match的值直接传入， 注意这个时候match的所有权会转移到里面
                panic!("Problem opening the file: {:?}", e)
            }
        },
    };
    //2.2 使用unwrap_or_else 简化这个操作
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
    //2.3 使用 unwrap 和expect来简化异常操作
    let f = File::open("hello.txt").unwrap();//这个将会直接吧错误抛出panic
    let f = File::open("hello.txt").expect("Failed to open hello.txt");//这个会在panic前面加上一个自己的提示文案
}


pub fn test_result()->Result<String, dyn Error>{
    //1. result 一般错误链写法
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
    //2. 特殊错误链写法 ? 关键字
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s);
    //3. ？关键字的链式调用
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
