//todo rust 中有单元包和包的区别
//一个单元包中可以有很多的包，但是只有一个库单元包，可以有任意多个二进制单元包

//todo 的包管理方法中，src/main.rs 和 src/lib.rs 分别作为枯荷二进制程序的单元包
//todo 如果rust项目中 src/main.rs src/lib.rs src/bin(有源文件) - 的时候，rust项目 会同时创建main 二进制包，和lib库包 和 任意bin数量的二进制包


//todo ---- rust 模块系统 -----

//模块树 - todo 注意 ： 任何一个rust项目都有一个由 main.rs 或者lib.rs 构建的全局包 crate 所有的内部的包又是源于这个跟节点的
// todo 注意了，所有的代码都有一个本文件名称的默认包名

mod front_of_house{
    pub mod hosting{
       pub fn  add_to_wait_list(){}
        fn seat_at_table(){}
    }

    mod serving {
        fn take_order() {
            //todo 使用super调用父相对作用域
            super::hosting::add_to_wait_list();
        }
        fn server_order() {}
    }
}

pub fn  eat_at_restaurant(){
    //使用绝对路径调用包
    crate::rust0005_pakage_module_other::front_of_house::hosting::add_to_wait_list();
    //使用相对路径调用包
    front_of_house::hosting::add_to_wait_list();
}
