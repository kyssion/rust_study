//todo 结构体增加方法支持

//定义一个正方形结构体
#[derive(Debug)]//显示的开启 debug 功能
struct Rectangle{
    width:u32,
    height:u32
}

// rust中使用impl 来关联 结构体
impl  Rectangle{
    // 这里使用self来表示当前的结构体引用
    fn area(&self)->u32{
        return self.width*self.height;
    }

    //rust 结构体的内置函数可以支持 可变引用
    fn add_with(&mut self,i:i32){
        self.width+=i;
    }

    //rust多参数例子
    fn can_hold(&self,other : &Rectangle)->bool{
        return self.width>other.width&&self.height>other.height;
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
    println!("{:?}",rect1.area());// todo rect1.area() 本身可以使用是因为rust提供了自动解引用和自动引用的功能
    // rect1 本身其实是一个指针，上面的代码等价于 (&rect1).area();
}

