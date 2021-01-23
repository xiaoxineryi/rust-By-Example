use std::fmt::{Display, Formatter};
use core::fmt;

pub fn display() {
    // In general, the `{}` will be automatically replaced with any
    // arguments. These will be stringified.
    println!("{} days", 31);

    // Without a suffix, 31 becomes an i32. You can change what type 31 is
    // by providing a suffix. The number 31i64 for example has the type i64.

    // There are various optional patterns this works with. Positional
    // arguments can be used.
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // As can named arguments.
    println!("{subject} {verb} {object}",
             object="the lazy dog",
             subject="the quick brown fox",
             verb="jumps over");

    // Special formatting can be specified after a `:`.
    println!("{} of {:b} people know binary, the other half doesn't", 1, 2);

    // You can right-align text with a specified width. This will output
    // "     1". 5 white spaces and a "1".
    println!("{number:>width$}", number=1, width=6);

    // You can pad numbers with extra zeroes. This will output "000001".
    println!("{number:>0width$}", number=1, width=6);

    // Rust even checks to make sure the correct number of arguments are
    // used.
    println!("My name is {0}, {1} {0}", "Bond","James");
    // FIXME ^ Add the missing argument: "James"

    // Create a structure named `Structure` which contains an `i32`.
    /*
        自己手动为结构体实现Display方法
    */
    #[allow(dead_code)]
    struct Structure(i32);
    impl Display for Structure{
        fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
            write!(f,"{}",self.0)
        }
    }
    // However, custom types such as this structure require more complicated
    // handling. This will not work.
    println!("This struct `{}` won't print...", Structure(3));
    // FIXME ^ Comment out this line.
}


pub fn display2(){
    use std::fmt; // Import the `fmt` module.

    // Define a structure named `List` containing a `Vec`.
    struct List(Vec<i32>);

    impl fmt::Display for List {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            // Extract the value using tuple indexing,
            // and create a reference to `vec`.
            let vec = &self.0;

            // 使用?如果有错误直接处理异常
            write!(f, "[")?;

            // Iterate over `v` in `vec` while enumerating the iteration
            // count in `count`.
            for (count, v) in vec.iter().enumerate() {
                // For every element except the first, add a comma.
                // Use the ? operator to return on errors.
                if count != 0 { write!(f, ", ")?; }
                write!(f, "{}", v)?;
            }

            // Close the opened bracket and return a fmt::Result value.
            write!(f, "]")
        }
    }

    let v = List(vec![1, 2, 3]);
    println!("{}", v);

}

#[derive(Debug)]
struct Matrix(f32,f32,f32,f32);
impl Display for Matrix{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"({},{})\n({},{})",self.0,self.1,self.2,self.3)
    }
}

fn transpose (m:Matrix) ->Matrix{
    Matrix(m.0,m.2,m.1,m.3)
}
pub fn display3(){
    let s = Matrix(1.1,1.2,1.3,1.4);
    println!("{}",s);
    println!("{}",transpose(s));
}