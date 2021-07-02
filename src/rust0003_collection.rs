
enum Item{
    String(String),
    I32(i32),
    I64(i64),
    USIZE(usize)
}

pub fn vec_test(){
    let mut vecItem:Vec<i32> = Vec::new();
    let mut vecItem = vec![1,2,3,4,5];
    vecItem.push(123);
    let a= vecItem.get(0);
    let a = vecItem[0]; //1. vec 可以通过 get 或者数组下标的方法获取 。 两种区别是， 下标引用了不是别的或出现panic get会返回None

    //2. vec 迭代方法
    for i in &vecItem{

    }
    for i in &mut vecItem{
        *i+=1;//todo 注意， 这里需要手动解引用
    }
    // todo 注意这里有问题， for in 本质上是语法糖，是一个函数， 调用的时候所有权向下转移了
    for i in vecItem{

    }

    //3. 嫂操作， vec可以和enum 结合实现多类型的数组
    let arr = vec![Item::String(String::from("now string")),Item::I32(123),Item::USIZE(333)];
    return
}
