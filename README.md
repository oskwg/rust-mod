# Rust 模块化：深入了解 Rust 中的代码组织

本文是一篇Rust基础文章，如果下面的问题对你不是问题，就不要浪费时间阅读这篇文章了，做些更有意义的事情吧。

**关键字`mod、pub、crate、self、super、use`都表示什么含义，如何使用？**

# 模块化

模块化是对代码一层一层的封装。`面向对象`语言中提供的`class`也算是一种模块化技术，有些语言使用`namespace`定义的`命名空间`也是一种模块化技术，让我们看看Rust中的模块化是如何设计的。

本文基于示例项目[rust-mod](https://github.com/oskwg/rust-mod)讲解，目录结构如下：
```shell
│  Cargo.lock
│  Cargo.toml
│  crate.json
│
├─json
│  │  Cargo.toml
│  │
│  └─src
│      │  json_decode.rs
│      │  json_encode.rs
│      │  lib.rs
│      │
│      └─codec
│              handle.rs
│              mod.rs
│
├─src
│      main.rs
│
└─xml
    │  Cargo.toml
    │
    └─src
            lib.rs
```

根目录下是一个workspace项目，对于`workspace`陌生可以看[上篇文章](https://mp.weixin.qq.com/s/d3xNNPpHD_WbJUCP-FjMCA)，我们使用`Root package`这种方式定义根项目，它依赖json和xml两个crate。


在Rust中模块有 4 种方式表示：
- crate可以表示一个模块
- 文件夹也可以表示模块
- 文件也可以表示模块
- 文件内使用关键字`mod`定义模块，并且可以嵌套

## 1. 用crate表示模块

我们使用命令`cargo new`创建的项目是一个crate。其它语言中通常叫做package，例如java中的jar package，js中的npm package。因此，我们可以把rust中的依赖包叫做`crate package`，但Rust社区中习惯上都是叫crate。

在根项目的Cargo.toml中引入依赖：
```toml
# /Cargo.toml
[dependencies]
json={path = "./json",version = "0.1.0"}
```

```rust
// src/main.rs
fn main() {
    json::encode("1111");
    println!("Hello, world!");
}
```

加入依赖之后，我们就可以在代码中使用`json::__`的方式来调用json模块内部`公开`的方法或结构了。

#### 关键字`use`

```rust
// src/main.rs
use json::*;
use crate::codec::handle::codec as ff;

fn main() {
    encode("1111");
    println!("Hello, world!");
}
```
使用`use json::*`批量引入json模块中的全部方法或结构。这样依赖我们就可以直接调用encode方法了，use主要减少重复写模块名的问题，类似其它语言中的`import`引入包名。使用`as`关键字可以为引入的模块定义一个别名。

#### 关键字`crate、super、self`
我们类比Linux中目录的相关概念：
- `crate`相当于根 '/'，当前包的根路径，通常是`use`一个绝对路径。
- `super`相当于 ‘../’，当前模块的上一级路径。
- `self`相当于 './'，就表示当前模块，通常省略。

## 2. 文件或文件夹表示一个模块

要把文件或文件夹作为模块，需要在lib.rs/main.rs/mod.rs文件中进行声明，例如在`/json/src/lib.rs`中声明了三个模块，在`json/src/codec/mod.rs`中声明了一个模块。

```rust
// json/src/lib.rs
mod json_encode;
mod json_decode;
mod codec;

// json/src/codec/mod.rs
pub mod handle;
```

其中json_encode和json_decode是文件，codec是文件夹。handle是文件。关键字`pub` 用于标注在模块、方法或结构体上，表示对模块外部公开。

## 3. 文件内部定义模块

在文件内部定义模块，使用关键字`mod`。

```rust
// json/src/json_encode.rs
mod inner_mod {
    use crate::codec::handle::codec;

    pub fn encode(s: &String) -> String {
        let cc: String = codec(&s);
        println!("private: {}", cc);
        cc
    }
}

mod inner_mod2 {
    
}

pub fn encode(s: &String) -> String {
    inner_mod::encode(s)
}
```

上面定义了模块`inner_mod`和`inner_mod2`，在模块内部可以使用`mod`无限嵌套模块，。



## 总结
最好结合代码仓库[rust-mod](https://github.com/oskwg/rust-mod)来理解本文内容。如果你也喜欢Rust,欢迎加微`code2c`交流。


模块是一种组织代码的方式，允许你将相关的功能分组在一起，提高代码的可读性和可维护性。通过`mod`关键字，你可以在Rust中创建模块并构建模块层次结构。本文知识点如下，看看你都掌握了吗？
1. 四种模块类型分别是crate、文件、文件夹、文件内部使用mod定义
2. pub关键字用于公开模块内部的各种结构
3. use关键字类似与其他语言的import，引入模块路径。
4. self、super、crate结合Linux中相对/绝对路径概念来理解。

<img src="docs/8b7f3e339f07a43d66dd696a3c66858.jpg" height="400px">
