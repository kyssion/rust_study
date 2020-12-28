
//定义一个枚举类型
//注意枚举类型的所有的变量都位于枚举名称的标识空间中
//这也就是说，任何一个枚举值都可以使用的枚举名称+:: 使用
enum IpAddrKind{
    V4,
    V6
}

fn test1(){
    //使用枚举类型
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

}

//todo 枚举在rust中其实是一种类型，可以直接在变种中使用
fn route(ip_type: IpAddrKind){

}