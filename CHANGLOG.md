# 0.4.0-beta.1 (unpublished)

- MessageContent extra 扩展字段变更为由 MessageEventDetail 持有
- MessageContent 增加 D 泛型，可支持更多 DetailType 模型
- RespContent 增加 E 泛型，为支持扩展 Event 模型
- ExtendedValue 添加 `Bytes(Vec<u8>)` 枚举类型
- 移除部分无用 Error
- Bot Api 变更为 BotActionExt trait 重新重构实现
- 添加 RespError 和 RespStatusExt trait
- rfc #154 添加两级群组支持

# 0.3.1

- 修复心跳包无 type 字段 bug

# 0.3.0

- 修复 `tokio-tungstenite 0.17` 默认不再为 request 添加 headers 的问题
- Hanlder 变更为一个泛型传入实例。