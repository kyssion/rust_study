
pub fn ownership_main(){
    // 所有权基本逻辑
    //1. rust中每一个值都有一个对应的变量作为所有者
    //2. 在同一个书剑内，值仅有一个所有者
    //3. 当所有者离开作用域的时候将会释放这个值

    let mut string  = String::from("string item");
    string.push_str(" push start");
    //todo 和 &str 相比，&str 不可变
    let mut str = "str item";

    //todo 克隆和转移 - 基本的变量进行赋值的时候的默认使用的方法是clone
    //todo 控制这两个的方法是 Copy和Drop的trait
    let i = 5;
    let j = i;
    // --- --- ---
    let i = String::from("owner string");
    let j = i;// todo 这个时候的i 是不可使用的 i 的所有权已经转移到了j 上面了
    let v = j.clone();//使用clone 可以获取一个string字符串的clone
    // println!(i);


    // todo 函数 变量的克隆和转移
    // todo 中函数其实和 变量相同，当传入变量的时候相当于进行赋值操作 ， 返回的时候相当于赋值时候的所有权

    let s = String::from("sdf");
    let s = test_fn(s);// 此时S已经没有任何作用域的信息

    //todo 使用&标记定义，可以做到在不获取变量的所有权的时候使用这个数值
    let i = String::from("item string");
    let z = &i;  //  获取这个值的引用


}


pub fn test_fn(s:String)-> String{
    return s;
}