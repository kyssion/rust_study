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
    //整形
    let i: i8;
    let i: i16;
    let i: i32;
    let i: i64;
    let i: i128;
    let i: usize;

    // 整型溢出 - debug模式抛出异常 ， 发布模式 补码环绕

    //浮点型
    let f:f32;
    let f:f64;

    //bool
    let b:bool = false;
    let b : bool = true;

    //字符类型 - rust 字符使用的是Unicode编码 占用4 个字节
    let c : char = '总';
    
    //复合类型
    
    //元组
    let y :(i32,f32,u8) = (12,12.0,1);
    let (a,b,_) = y; // rust 这里和golang的处理方法类似使用_ 来丢弃不用的值

    let c = y.1;// 元组可以使用的后缀+ 数字来确定标记位置

    //数组
    let a = [1,2,3,4,5,6]; // 数组初始化
    let b : [i32; 10]; // 初始化声明数组



}