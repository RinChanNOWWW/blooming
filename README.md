# bloom

BT/PT 站 RSS 更新通知器。

## 支持的站点

- [蜜柑计划](https://mikanani.me/)
- [BYRBT](https://byr.pt/)
- [北洋园PT](https://tjupt.org/)

## 通知方式

- QQ 机器人: [go-cqhttp](https://github.com/Mrs4s/go-cqhttp).

## 使用

```bash
./bloom -c config.toml

# or

./bloom -c config.toml -d # daemon mode
```

配置文件请参考 [config.toml](examples/config.toml).

通知内容示例：

```
NEWLY UPDATE:
【豌豆字幕组】[海盗战记 / 冰海战记 第二季 / Vinland_Saga_S2][03][繁体][1080P][MP4] (2023-01-24 14:34:19.989 +08:00)
```