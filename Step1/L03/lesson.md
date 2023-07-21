# Lesson

- use : 导入模块 ，类似 import
- cargo.toml \[dependencies\] : 第三方依赖
- let : 变量 （默认不可变）
- let mut : 可变变量
- fn(){} : 函数
- println! : 宏 ，打印 console
- format! : 宏， 创建包含变量值的字符
- rand::thread_rng() ：随机数的生成器
- loop 表达式。 类似 while 。 可以通过 continue break 来退出本次循环，退出循环
- match 表达式。 执行表达式，并对不同的结果，执行不同的分支语句。 有点类似 switch case ，但更高级

```
let value = 1;

match value {
    1 => println!("one"),
    2 => println!("two"),
    _ => println!("something else"),
}
```

## 疑问？

- 什么时候用`::` , 什么时候用`.`
- 为什么`result`的`OK` 有括号，`Ordering` 没有

```
Ok(num) => num

Ordering::Equal => {}

```

- 为什么函数都有个第一个参数是 `&self`
- .expect 有什么作用
