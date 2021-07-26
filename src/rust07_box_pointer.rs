use crate::rust07_box_pointer::List::{Cons, Nil};

// 定义一个List - 老描述一种嵌套类型的结构
enum List{ // todo 这里一定是Box 类型的， 因为rust box 类型中储存的是指针
    Cons(i32,Box<List>),//todo rust 一定会计算当前的结构需要的内存大小， 如果使用Box 它将会计算box指针的大小 ， 否则将会循环引用导致出错
    Nil
}

pub fn test(){
    // box 堆上分配内存 -  通过不停的new 来创建需要的数据
    let list = Cons(1,Box::new(Cons(2,Box::new(Cons(3,Box::new(Nil))))));
}