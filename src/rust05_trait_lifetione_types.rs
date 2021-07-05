
//1. rust中的范型 - T 用来替代类型
// fn largest<T>(list: &[T]) -> T {
//     let mut largest = list[0];
//     for &item in list {
//         if item > largest {
//             largest = item;
//         }
//     }
//
//     largest
// }

//2. 范型支持struct 结构体
struct Point<T> {
    x: T,
    y: T,
}

fn structType(){
    let a = Point{
        x:1,y:2
    };
    let b= Point{
        x:"fff",y:"xxxx"
    };
}
