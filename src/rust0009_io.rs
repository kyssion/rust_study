
//1. rust 命令行参数

use std::*;
use std::ffi::OsString;
use std::fs;

fn test_item(){
    //这种方法获取命令行参数的时候，如果是非Unicode编码的时候会painc
    let mut itemVar:Vec<String>  = env::args().collect();
    //使用这种方法可以保证数据是 OsString - 这个是和平台相关的string 类型
    let itemVar2 :Vec<OsString> = env::args_os().collect();

    //这里获取itemVar的两种借用
    let mut arg1 = &mut itemVar[0];
    arg1.push_str("123");
    let arg2 = &itemVar[1];
    let i = [1,2,3,4,5];
    let mut p = &i[0];

    //文件读取方法
    let contents =  fs::read_to_string("xx").expect("read file error");


}