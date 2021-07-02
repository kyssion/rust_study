
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

    }
    // todo 注意这里有问题， for in 本质上是语法糖，是一个函数， 调用的时候所有权向下转移了
    for i in vecItem{

    }

    return
}
