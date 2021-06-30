
pub fn test(){
    //1. rust 的所有权一次只能有一个
    //2. 只是针对使用copy方式的变量， 如果是使用的赋值的方法 ，  堆上的数据基本上都是使用的引用复制
    //3. rust 区别使用copy还是使用的drop 是通过rust 内部的一个trait 支持的 ， Copy trait 和drop trait
    let mut strItem = String::from("ffff");
    let mut p = strItem;//所有权转移到p 上面了

    //只要发生了赋值操作， 所有权就发生转移， 包括变量赋值， 函数调用 ， 函数返回值接受

    //todo rust 这里很严谨，如果是类似对象的这种情况- rust本身应该是禁止所有权转移的 ， 数组验证过， 结构体待验证 ， 元祖代验证

    //4. 借用和可变引用 , 规则： 一旦出现mut引用的时候，之前的所有非mut引用将会无效，任何一个变量只能有一个可变引用
    let mut a = [String::from("fff"),String::from("fff"),String::from("fff"),String::from("fff")];

    let p = &a[0];//这里指定了一个引用
    let p = &mut a[0];//这里指定了一个可变引用
}