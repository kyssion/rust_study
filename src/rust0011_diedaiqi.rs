
fn test1(){
    //创建迭代器
    let mut itemArr = vec![1,2,3,];
    //生成不可变化迭代器
    let itemIter = itemArr.iter();
    // 元素可变迭代器
    let itemIterMut = itemArr.iter_mut();
    //使用迭代器进行遍历数据 - 注意这个for in  会强制让 itemArr变成可变引用
    for varItem in itemIter{
        println!("val : {}",varItem)
    }


    //获取了 itemArr的所有权 并且 元素可变的迭代器
    // let itemIntoIter = itemArr.into_iter();

    //ps : 所有的可以使用迭代器的对象都是实现了Iterator trait 的对象


}