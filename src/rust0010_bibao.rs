use std::*;

fn simulated_expensive_calculation(intensity:i32) -> i32{
    println!("start");
    thread::sleep(time::Duration::from_secs(2));
    return intensity;
}

fn test1(){
    let num = 123;
    //定义一个闭包
    //todo  注意这种闭包有一个问题，没有制定变量类型
    //todo 所以，rust 将会用的第一次调用的类型当作这个闭包的类型
    let bItem = |params|{
        params
    };

    let v = bItem(123);

    let bItem = |params|params;// 简化写法
    let v = bItem(123);

    //todo 定义 闭包类型 和函数一样
    let bItem = |param : i32|->i32{
        param
    };
}

//todo rust 闭包，惰性求值 -  rust可以使用结构体 + 闭包传参 - 动态创建闭包封装
//rust 中的闭包，可能会实现 Fn FnMut FnOnce 三种中的一种
struct Cache<T> where T:Fn(u32)->u32{
    c:T,
    value:Option<u32>
}

impl<T> Cache<T> where  T:Fn(u32)->u32{
    fn new(ca :T) ->Cache<T>{
        return Cache{
            c:ca,
            value:None
        }
    }

    fn value(&mut self , arg:u32)->u32{
        match self.value {
            Some(v)=>v,
            None =>{
                let v = (self.c)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}
