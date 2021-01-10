// 范型

//结构体范型

//在结构体中的相同的类型需要保证同样的使用时候的类型，比如都是T ，那么在真实使用的时候都必须是 相同的类型
//todo rust 的范型就是符号相同
struct Point<T,U>{
    x: T,
    y: U
}

impl <T,U> Point<T,U> {
    fn test(&mut self) ->&T{
        return &(self.x);
    }
}

//在枚举中的范型

enum TestEnum<T,U>{
    One(T),Two(U)
}


//trait 共享行为 -  和接口相同，但是又何接口不同

pub trait Summary {
    fn summarize(&self) -> String;
    //todo 制定一个默认的实现方法
    fn summarize2(&self) -> String{
        println!("heheh");
        return String::from("ttt");
    }

}

//类型实现trait

pub struct TestSummary{

}

impl Summary for TestSummary{
    fn summarize(&self) -> String {
        unimplemented!()
    }
}


fn test_generics(){

}
