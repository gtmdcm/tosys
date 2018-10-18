# tosys
A toy operating system

## 构建
我们使用docker以便在不同平台上构建
```shell
docker run -v tosys目录:/tosys --workdir="/tosys" longfangsong/os-build:0.0.3 bootimage build --target ./target.json
```

## 运行
直接使用qemu运行：
```shell
qemu-system-x86_64 -drive format=raw,file=./target/target/debug/bootimage-tosys.bin
```