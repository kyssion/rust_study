
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

    //todo 引用可以具有可变的特性
    let mut i = String::from("can change string");
    let mut j = &mut i;
    j.push_str("test");

    // todo 注意在rust中，rust的规则保证一个变量一个时间只能有一个标记进行引用
    // todo 也就是说针对这种可以进行修改的引用，在同一个时间内只有一个有效果
    // todo rust 的引用规则其实简单的说有一下的几个规则
    // todo 以可变引用为分界线，形成代码快，每一个代码块中的所有不可变引用都应该保证在块外没有使用
    let mut i = String::from("sdfsdf");
    let k = &i;
    let j = &i;
    println!("{},{}",k,j);
    let mut p = &mut i;
    println!("{}",p);// true
    i.push_str("ssss");
    //println!("{}",p);// todo error 这里因为i.push 方法内部使用的&mut self 等价于调用了 &mut i 所以这里发生了数据迁移，导致问题了

    let j = &mut i;

    //todo 这里还涉及到一个生命周期的问题，这个暂时不进行讨论

    // --- ---- ---- 切片 --- --- ---- ---
    //todo 切片和 引用非常类似，切片是针对连续存储空间的结合进行操作，同样不会发生所有权转移

    let mut str = String::from("test string");
    let itemStr = &str[0..1];

}


//todo 使用切片技术来解释一下这种场景
fn test_main() {
    let mut string = String::from("test item");
    let word = test3_fn(&string);
    println!(word);
    string.clear(); // todo -> 这里是一个可变的引用，使用这种方法将会导致这个变量对应之前的所有引用都无效化
}

pub fn test3_fn(s:&String) -> &str{
    let bytes = s.as_bytes();
    for (i,&item) in bytes.iter().enumerate(){
        if item == b' '{
            return &s[0..i];
        }
    }
    return &s[..];
}


pub fn test_fn(s:String)-> String{
    return s;
}

//使用引用的方法进行直接的参数传递
pub fn test2_fn(s: &String)->usize{
    return s.len();
}