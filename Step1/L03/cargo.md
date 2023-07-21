# cargo

Cargo 是 Rust 的构建系统和包管理器，它可以为你处理很多任务，比如构建代码、下载依赖库并编译这些库

## 创建新项目

`cargo new HelloWord`

## Cargo.toml

Cargo.toml 是对项目的描述，包括一些介绍，版本，依赖等，类似前端项目中的`package.json`

## Cargo 相关操作

- cargo new 创建项目。 不要用驼峰，用短横
- cargo build 构建项目。
  - --debug : 默认是 debug , 在 ./targe 目录下生成 debug 目录
  - --release : 发布构建 , 在 ./targe 目录下生成 release 目录
- cargo run 一步构建并运行项目。
  - --debug : 默认是 debug , 运行 debug 目录的二进制文件
  - --release : 发布运行 , 运行 release 目录的二进制文件
- cargo check 在不生成二进制文件的情况下构建项目来检查错误。
