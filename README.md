# 相关资料

1.[使用 Rust 编写操作系统](https://rust.cc/article?id=57e7ead4-7596-41cb-913e-7bd075caa7f4)

2.[使用 Rust 编写操作系统](https://github.com/rustcc/writing-an-os-in-rust)

编译启动命令
```shell
# 编译系统镜像
cd ~/projects/hello && cargo bootimage
# 切换到系统目录
cd ~/projects/hello/target/os/debug
# 启动虚拟机
qemu-system-x86_64 -drive format=raw,file=bootimage-hello.bin

```

