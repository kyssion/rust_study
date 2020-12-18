pub fn test1() {
    //let 和mut let mut 类型可以进行定义并且进行相关的修改
    let value1 = 10;
    let value1 = "sdfsdf";
    let mut value2 = 10;
    value2 = 11; // 可以进行修改
                 // value1 = 12;
                 /* let 之后的变量不能进行修改， let mut 之后的变量可以进行修改 let 相同的名字的变量 第二个当作新的变量使用 */

    //常量 - 常量 必须显示的标记出变量的类型
    const MAX_POINTS: i32 = 10000;
}

//常量可以定义在全局外
const MAX_LENGTH: i32 = 1000;

pub fn data_type() {
    //todo 整形
    let i: i8;
    let i: i16;
    let i: i32;
    let i: i64;
    let i: i128;
    let i: usize;

    // 整型溢出 - debug模式抛出异常 ， 发布模式 补码环绕

    //todo 浮点型
    let f:f32;
    let f:f64;

    //todo bool
    let b:bool = false;
    let b : bool = true;

    //todo 字符类型 - rust 字符使用的是Unicode编码 占用4 个字节
    let c : char = '总';
    
    // todo 复合类型
    
    // todo 元组
    let y :(i32,f32,u8) = (12,12.0,1);
    let (a,b,_) = y; // rust 这里和golang的处理方法类似使用_ 来丢弃不用的值

    let c = y.1;// 元组可以使用的后缀+ 数字来确定标记位置

    // todo 数组
    let a = [1,2,3,4,5,6]; // 数组初始化
    let b : [i32; 10]; // 初始化声明数组

    //todo 字符串

    let str = "&str"; // 这个是字符串字面量，不能进行修改
    let str = String::from("String"); //堆分配字符串
}


//rust 语句和表达式
pub fn statement_and_expression(){
    // rust中的其实大部分都是表达式，但是有些是没有的区分方法是是否有返回值， 这里列举几个例子
    let i = 123;// 赋值语句是语句不是表达式 所以不能想其他语言一样使用 a=b=12 这种方法赋值

    let i = { // 代码块中不用分号结尾的语句为表达式可以返回值
        let a = 12;
        let b = 12;
        a+b
    };

    // if swich等流程控制语句都是表达式可以直接进行赋值
    let i = if(i>3){
        12
    }else{
        13 // todo 注意使用这种特性的时候要保证 各个判断分支返回的表达式类型时相同的
    };


    //loop
    let item = loop{
        break 1+123; // todo 注意如果返回值比较多的时候要保证类型是相同的
    };
}

// todo 注意rust 使用蛇形命名方法 ， 参数和基本明明方法相同，返回值如果有多个的时候使用元组
pub fn func_test(i : i32)-> String{
    return "123123123".parse().unwrap();
}

// 控制流

pub fn process_test(){
    //todo  if else
    let number = 3;
    // 基本的流程语句结构
    if number <5 {// rust 表达式计算之后的结果必定为true或者false

    }else if number>=5 && number<10{

    }else{

    }

    // 使用控制流程语法规则 - 利用 表达式语句特性
    let a = if number<5{
        5
    }else{
        10 // todo 注意使用这种特性的时候要保证 各个判断分支返回的值类型时相同的
    };


    // todo 循环结构 rust中有三种类型 loop while  for

    //  loop 死循环
    loop{
        println!("loop");
        break;
    }

    //todo 表达式特性 - 和 if 相同 要保证返回值类型一致 使用 break+返回值抛出

    let item = loop{
        break 1+123;
    };

    // while - 没有表达式语法

    let a = while  number<5 {
        break;
    };

    //for - 特点，是使用迭代器的非常优秀的循环
    let arr = [1,2,3,4,5];
    for number in arr.iter(){
        println!("{:?}",number);
    }

}

