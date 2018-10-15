# tosys
A toy operating system

## 构建
我们使用docker以便在不同平台上构建
```shell
docker run -v tosys目录:/tosys --workdir="/tosys" longfangsong/os-build:0.0.1 make
```

## 运行
直接使用bochs运行，Makefile中有快捷方式：
```shell
make run 
```