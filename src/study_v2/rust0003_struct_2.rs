//todo 结构体增加方法支持

//定义一个正方形结构体
//todo 这里注意一个结构的可见性细节，结构体pub 只是标志这个结构体可以被外部引用，但是里面的变量还是私有的，需要加上pub才能被外部使用
#[derive(Debug)]//显示的开启 debug 功能
pub struct Rectangle{
    pub width:u32,
    height:u32
}

// rust中使用impl 来关联 结构体
impl  Rectangle{
    // 这里使用self来表示当前的结构体引用
    fn area(&self)->u32{
        return self.width*self.height;
    }

    //rust 结构体的内置函数可以支持 可变引用
    fn add_with(&mut self,i:u32){
        self.width+=i;
    }

    //rust多参数例子 -  第一个参数必须
    fn can_hold(&self,other : &Rectangle)->bool{
        return self.width>other.width&&self.height>other.height;
    }
}

//任何一个结构体都支持多个关联的impl块
impl Rectangle{
    //todo 关联函数可以不初始化对象直接调用函数 - 特征： 没有self
    fn square(size: u32)->Rectangle{
        Rectangle{
            width:size,
            height:size
        }
    }
}

fn area(rectangle: &Rectangle)->u32{
    return rectangle.height*rectangle.width;
}

pub fn example(){
    let rect1 = Rectangle{
        width:12,
        height:10
    };
    let area = area(&rect1);
    println!("{:#?}",rect1);//todo 使用{:#?}格式化输出 struct中的内容 只在debug模式中有效

    //todo 使用结构体内置的方法输出数字
    println!("{:?}",rect1.area());// todo rect1.area() 本身可以使用是因为rust提供了自动解引用和自动引用的功能 ？？anwser 这里有问题 rust 中的& 和 × 的问题
    // rect1 本身其实是一个指针，上面的代码等价于 (&rect1).area() rust 自动的添加了react的解引用

    //todo 使用关联函数闯劲啊Rectangle
    let p =Rectangle::square(12);
}

