
pub fn test(){
    //1. rust 的所有权一次只能有一个
    //2. 只是针对使用copy方式的变量， 如果是使用的赋值的方法 ，  堆上的数据基本上都是使用的引用复制
    //3. rust 区别使用copy还是使用的drop 是通过rust 内部的一个trait 支持的 ， Copy trait 和drop trait
    let mut strItem = String::from("ffff");
    let mut p = strItem;//所有权转移到p 上面了

    //只要发生了赋值操作， 所有权就发生转移， 包括变量赋值， 函数调用 ， 函数返回值接受

    //todo rust 这里很严谨，如果是类似对象的这种情况- rust本身应该是禁止所有权转移的 ， 数组验证过， 结构体待验证 ， 元祖代验证

    //4. 借用和可变引用 , 规则： 一旦出现mut引用的时候，之前的所有非mut引用将会无效，任何一个变量只能有一个可变引用
    // 借用要注意下： 必须保证有效， 否则借就没有意义了
    // 注意这里使用的时候 可能会有隐形引用的场景
    let mut a = [String::from("fff"),String::from("fff"),String::from("fff"),String::from("fff")];

    let p = &a[0];//这里指定了一个引用
    let p = &mut a[0];//这里指定了一个可变引用


    //5. rust 切片 - 数组类型 字符串或者数组 ，
    //todo 可变性的支持， 是否可变??

    let mut str= String::from("fffff");
    let p = &str[..1];

    //6. 引用本身是一种指针， 但是可以使用.xxx() 调用方法， 本质上是因为它实现了自动接引用的能力
    let z = &mut str;
    z.push('z');
    (*z).push('z')
}