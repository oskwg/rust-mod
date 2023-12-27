// 将codec方法引入到当前mod下，同时公开对外部公开
pub use crate::codec::handle::codec;

// 声明mod，模块只有使用mod关键字声明后才可以使用。通常在lib.rs/mod.rs/main.rs中文件中进行声明
mod json_encode;
mod json_decode;
mod codec;

// 类比linux中的概念，crate相当于根'/',super相当于‘../’,self相当于'./'
// crate super self

// pub # 用于标注在模块或方法上，表示对外部模块公开
// mod
// use # 类似javascript中的import

pub fn encode(s: &str) -> String {
    json_encode::encode(&String::from(s))

    // codec(&String::from(s))
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//
//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
