// 设计一个链表

use crate::Enums::linkedList::List::{Cons,Nil};

// 这里的Box就是一个指针，指向对应数据的地址，如果没有指针的话，编译时无法知道递归类的数据的大小，但是通过指针，
// 每个对象的大小是固定的，从而保证编译时能明确知道数据大小。
enum List{
    Cons(u32,Box<List>),
    Nil
}

impl List {
    fn new() -> List {
        List::Nil
    }

    fn prepend(self,u:u32) -> List{
        Cons(u,Box::new(self))
    }

    // 这里要使用*self,因为self是自己的指针，对应的是&List的对象，所以需要解引用
    fn len(&self) ->u32{
        match *self{
            // 这里使用ref 借用自己内部的对象，否则所有权会被更改，无法通过编译
            Cons(_,ref tail)=> 1 + tail.len(),
            Nil => 0
        }
    }

    fn stringify(&self)->String{
        match *self {
            Cons(head,ref tail) => {
                format!("{}, {}", head, tail.stringify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
// static修饰的对象就是全局变量，可以使用mut修饰来更改，但是如果是mut的话使用的时候必须要用unsafe的语句块
static mut X:&str = "Hello";

pub fn listTest(){
    let mut list = List::new();
    list = list.prepend(2);
    list = list.prepend(3);
    println!("{}",list.stringify());
    unsafe {
        println!("{}",X);
        X = "yes";
        println!("{}",X);
    }

}

