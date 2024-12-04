# blooming

BT/PT 站 RSS 更新通知器。

## 支持的站点

- [蜜柑计划](https://mikanani.me/)
- [BYRBT](https://byr.pt/)
- [北洋园PT](https://tjupt.org/)

## 通知方式

- QQ 官方频道机器人。
- QQ 机器人: [NapCatQQ](https://github.com/NapNeko/NapCatQQ).

## Install

### Download and Install from crates.io

```bash
cargo install blooming
```

### Download from Github Release

https://github.com/RinChanNOWWW/blooming/releases

### Build from source

```bash
cargo build --release
```

## 使用

```bash
blooming -c config.toml

# or

blooming -c config.toml -d # daemon mode
```

配置文件请参考 [config.toml](examples/config.toml).

通知效果：

![1](./docs/pic1.png)
![2](./docs/pic2.png)
