
[![crates.io](https://img.shields.io/badge/crates.io-0.2.7-green.svg)](https://crates.io/crates/cargo-bullet)
[![LICENSE](https://img.shields.io/badge/LICENSE-apache-green.svg)](http://www.apache.org/licenses/LICENSE-2.0)
[![Build Status](https://www.travis-ci.org/thegenius/bullet.svg?branch=master)](https://www.travis-ci.org/thegenius/bullet)

# luban
luban is a generic project generator

# install
## mac
```shell script
brew install luban-project/luban/luban
luban --version
```

## linux/unix
```shell script
curl https://sh.rustup.rs -sSf | sh # 1. install rust: 
cargo install cargo-luban           # 2. install luban: 
cargo luban --version               # 3. check version: 
```


### Known Problems
|OS|Problem|Solve|
|--|--|--|
|centos|Could not find directory of OpenSSL|yum install openssl-devel|
|ubuntu|linker `cc` not found|sudo apt-get install build-essential|
|ubuntu|Could not find directory of OpenSSL|sudo apt install libssl-dev & sudo apt instll pkg-config|  

## windows
1. install rust: following [rust-lang](https://www.rust-lang.org/tools/install)
2. install bullet: cargo install cargo-bullet
3. check version: cargo bullet --version
* please install visual studio to get the msvc compiler

# usage example
```shell script
# for the first time, install project template
luban install --name=bullet-spring-java-maven 

# generate project
luban fast-create --name=bullet-spring-java-maven --project=com.foo.example
cd example

# make shell executable
chmod 755 gen/gen.sh
chmod 755 run.sh

# generate code and run
./gen/gen.sh 
./run.sh
```

# bullet-spring-java-maven模板说明

## 基本项目结构
```text
root --|
    api --|  #对外提供的二方包，只能包含接口和POJO类，不能包含实现
             #后续dubbo接口/grpc接口也需要定义在这里
    app --|  #主体应用程序
        biz --| #业务逻辑层，应该只包含逻辑结构的组织
        clg --| #核心逻辑层，应该只包含核心的领域模型和逻辑，要求纯函数
        dal --| #数据操作层，mysql/redis/hbase/file等
        ext --| #外部防腐层，外部调用，mq接入等
        fun --| #通用方法层，纯函数
    gen --|  #自动代码生成的插件和配置
```

## 数据库代码生成
数据库相关的sql脚本在app/src/main/resouces/db/migration目录下  
代码生成命令如下：
```shell script
chmod 755 gen/gen.sh
./gen/gen.sh
```

## 项目运行
```shell script
chmod 755 run.sh
./run.sh
```


# Supported Templates
## Java Spring Gradle
```text
cargo bullet install --name=bullet-spring-java
cargo bullet create  --name=bullet-spring-java
cargo bullet build   --name=bullet-spring-java --output=out 
```

## Java Spring Maven
```text
cargo bullet install --name=bullet-spring-java-maven
cargo bullet create  --name=bullet-spring-java-maven
cargo bullet build   --name=bullet-spring-java-maven --output=out
```

## MySQL accumulator
```text
cargo bullet install --name=bullet-sql-accumulator
cargo bullet create  --name=bullet-sql-accumulator
cargo bullet build   --name=bullet-sql-accumulator --output=out 
```
