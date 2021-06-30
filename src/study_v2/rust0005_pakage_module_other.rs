//todo rust 中有单元包和包的区别
//一个单元包中可以有很多的包，但是只有一个库单元包，可以有任意多个二进制单元包

//todo 的包管理方法中，src/main.rs 和 src/lib.rs 分别作为枯荷二进制程序的单元包
//todo 如果rust项目中 src/main.rs src/lib.rs src/bin(有源文件) - 的时候，rust项目 会同时创建main 二进制包，和lib库包 和 任意bin数量的二进制包


//todo ---- rust 模块系统 -----

//模块树 - todo 注意 ： 任何一个rust项目都有一个由 main.rs 或者lib.rs 构建的全局包 crate 所有的内部的包又是源于这个跟节点的
// todo 注意了，所有的代码都有一个本文件名称的默认包名

pub mod front_of_house{
    pub mod hosting{
       pub fn  add_to_wait_list(){}
        fn seat_at_table(){}
    }

    pub mod serving {
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

//todo 使用use 关键字将mod 路径引入当前作用域 - self 相对路径 crate 绝对路径 使用as 重新命名
use crate::rust0005_pakage_module_other::front_of_house::hosting as two;
use self::front_of_house2::hosting::add_to_wait_list as one;

//todo 指定一个完成的枚举直接引入
use std::collections::HashMap;
use std::fmt;

//todo 如果 名称相同的时候，需要指明父作用域

// fn test1()->Result<()>{
//     fmt::Result
// }


mod front_of_house2{
    pub mod hosting{
        pub fn add_to_wait_list(){}
    }
}

use front_of_house2::hosting::add_to_wait_list;

pub fn eat_at_restaurant2(){
    add_to_wait_list()
}


//todo 外部包

use rand::Rng;

//todo 使用嵌套路径整理包
use std::{cmp::Ordering,io};
//todo 等效于
// use std::cmp::Ordering;
// use std::io

//todo 使用通配符引用所有的数据到指定的作用域
use std::*;

//todo 一个疑问？ rust 的mod 的权限管理是啥样子的pub

//rust 的逻辑是这样的- 先找到公共的跟路径，然后从这个跟路径开始 只能找到pub标记的值
//todo 像一个树杈
// 比如 A的路径 a-b-c-d-e-f B 的路径 a-b-c-f-f - 公共根是c -> c 无需考虑是否是pub 引用放都是可见的， 剩下的子路径调用方必须保证是pub标记才能被使用
mod sync_mod_test{
    pub mod sync_mod_child_one{
        mod sync_mod_cc_one_1{
            use super::super::sync_mod_child_two::sync_mod_cc_one;
            // use super::super::sync_mod_child_two::sync_mod_cc_two;
            fn test(){
                sync_mod_cc_one::gggg()
            }
        }
    }


    mod sync_mod_child_two{
        pub mod sync_mod_cc_one{
           pub fn gggg(){

            }
        }
        mod sync_mod_cc_two{
            pub fn gggg(){

            }
        }
    }




}

