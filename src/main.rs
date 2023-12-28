// 在2018之后，不需要再使用下面这句,外部crate可以直接使用
// extern crate json;

use json::ff;
use json::encode;

fn main() {
    ff(&String::from("1111"));
    encode("1111");
    println!("Hello, world!");
}
