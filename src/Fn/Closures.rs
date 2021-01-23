// 闭包

use core::mem;

// 使用闭包可以捕获函数环境外的变量进行使用
pub fn capture(){
    let mut a = 1;
    let mut addOne = || a = a + 1;
    addOne();
    addOne();
    println!("{}",a);
}

// 使用闭包当做参数传递的时候，需要有基本的声明类型
// 函数的描述一共有三种，分别是Fn、FnOnce以及FnMut，分别对应的是&T T 以及 &mut T

fn apply<F>(f:F) where F:FnOnce(){
    f();
}
// 这里表示传进来一个函数和一个i32的数据，函数必须可以多次调用即相当于是&T，并且函数需要是参数为一个i32的数据
fn apply_with_parameter<F>(f:F) where F:Fn(i32){
    f(2);
}

pub fn f(){
    let greeting = "123";
    let e = ||{mem::drop(greeting)};
    let double = |x|{
        let e =2 * x;
        println!("{}",e);
    };

    apply(e);
    apply_with_parameter(double);
}