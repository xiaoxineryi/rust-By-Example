// 对应于iter的三种不同的遍历方法

// iter直接进行遍历的话，是借用
pub fn iter(){
    let e = vec!["a","b","c"];
    for i in e.iter(){
        println!("{}",i);
        match *i {
            "a" =>{
                println!("yes");
            },
            _ =>{
                println!("no");
            }
        }
    }
}
// 直接获取数组中的数据，使用后所有权被变更，无法再次在数组内使用
pub fn into_iter(){
    let e = vec!["a","b","c"];
    for i in e.into_iter() {
        println!("{}",i);
    }
    // 会报错，因为所有权被改变
    // for i in e.into_iter() {
    //     println!("{}",i);
    // }
}
// 借用但是 可以改变
pub fn iter_mut(){
    let mut e = vec!["a","b","c"];
    for i in e.iter_mut() {
        // 因为借用所以得到的是地址，需要解引用来修改对应的数据
        *i = "d";
        println!("{}",i);
    }
    for i in e.iter() {
        println!("{}",i);
    }
}