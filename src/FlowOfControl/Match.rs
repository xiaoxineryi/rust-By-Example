
// 当match对应判断的数据没有变量名，但是后面要做的需要用到的时候，可以自己赋值一个名字并用@来表示等于


fn t() ->i32{
    2
}

pub fn M(){
    match t() {
        n @ 1=>{
            println!("is {}",n);
        },
        n @ 2..=4 =>{
            println!("it is {}",n);
        },
        _ =>{
            println!("nothing happen")
        }
    }
}
enum L{
    Foo,
    Bar
}
// if let 就是简化的match _ ,
pub fn iflet(){
    let a = L::Bar;
    if let L::Bar = a {
        println!("a is Bar")
    }else{
        println!("a is another");
    }

// while let 就是一直执行 一直到对应的let条件不对
    let mut a = Some(1);
    while let Some(i) = a{
        if i > 9 {
            println!("end");
            a = None
        }else {
            println!("the value is {}",i);
            a = Some(i + 1);
        }
    }
}

