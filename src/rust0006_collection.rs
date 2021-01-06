//集合

//动态数组
fn vec_test(){
    //创建动态数组 - rust 的 vec数组可以动态的增加指定类型的数据，而不需要关系空间是否够用
    let mut v: Vec<i32> = Vec::new();
    //通过宏 - 快速创建有初始值的动态数组
    {
        let mut v = vec![1,2,3,4];
    } // 注意，动态数组在这个时候将会被销毁 因为离开了作用域

    //获取数组中的内容

    let i = &v[1]; // 这个会有越界 问题
    let i = v.get(1); // 这个不会有
    //todo 这里要注意rust的引用规则

    //修改值和遍历
    for i in &mut v{
        *i+=5;//这里解引用并且增加值
    }

    //使用枚举来让rust的数组支持不同的类型

    enum SpreadsheetCell{
        Int(i32),
        Float(f65),
        Text(String)
    }
    let row = vec![SpreadsheetCell::Int(3),SpreadsheetCell::Float(5.4),SpreadsheetCell::Text(String::from("ffff"))];
}