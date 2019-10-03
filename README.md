# 一天学会做操作系统
> 这个标题绝对吹牛逼，连我自己都这么觉得...


## 相关资料 

1.[使用 Rust 编写操作系统](https://rust.cc/article?id=57e7ead4-7596-41cb-913e-7bd075caa7f4)

2.[使用 Rust 编写操作系统](https://github.com/rustcc/writing-an-os-in-rust)

3.[自己实现的一个基于x86的操作系统](https://bbs.pediy.com/thread-182967.htm)

4.[XV6 中译版](https://github.com/deyuhua/xv6-book-chinese/blob/master/book/zh/chap01.md)

5.[WritingOS](http://oldlinux.org/Linux.old/docs/WritingOS.pdf)

6.[Operating_Systems_From_0_to_1](./doc/Operating_Systems_From_0_to_1.pdf)

7.[blog_os](https://github.com/phil-opp/blog_os)

8.[使用 Rust 编写操作系统 原版博客](http://os.phil-opp.com/)

9.[消息队列的编写](https://github.com/johnmq/john)

10.[调试器中断点是如何工作的](https://eli.thegreenplace.net/2011/01/27/how-debuggers-work-part-2-breakpoints)

11.[使用裸函数处理异常消息](https://os.phil-opp.com/first-edition/extra/naked-exceptions/)

12.[Python 内核剖析](https://eli.thegreenplace.net/tag/python-internals)

13.[Python 开发解析](https://devguide.python.org/exploring/)

## 安装 Rust 命令
```shell
# 切换成实验版本
rustup toolchain install nightly
# 默认为实验版本
rustup default nightly
# 更新
rustup update
```

## 安装 系统开发脚手架
```shell 
# 安装构建工具
cargo install cargo-xbuild
# 安装 bootimage
cargo install bootimage --version "^0.7.7"
# 安装 llvm-tools-preview
rustup component add llvm-tools-preview
```

## 编译启动命令
```shell
# 编译系统镜像
cd ~/projects/hello && cargo bootimage
# 切换到系统目录
cd ~/projects/hello/target/os/debug
# 启动虚拟机
qemu-system-x86_64 -drive format=raw,file=bootimage-hello.bin

```

## 测试

```shell
# 测试 lib.rust
cargo xtest --lib

# 直接运行
cargo xrun
```

