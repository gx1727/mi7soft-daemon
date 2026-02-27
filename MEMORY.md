# mi7soft-daemon 项目记忆

## 项目概述

- **名称**: mi7soft-daemon (别名 m7d)
- **用途**: Linux 进程守护工具，让服务持续运行
- **技术栈**: Rust + tokio + React/TypeScript (前端开发中)
- **GitHub**: https://github.com/gx1727/mi7soft-daemon

## 当前版本

- **v0.1.2** - 最新已发布

## 已完成功能

| 功能 | 状态 | 说明 |
|------|------|------|
| 进程管理 | ✅ | 启动/停止/重启/监控 |
| 自动拉起 | ✅ | 进程崩溃后自动重启 |
| 状态持久化 | ✅ | daemon 重启后自动接管 |
| 日志系统 | ✅ | tracing + 文件轮转(按天，保留7天) |
| 二进制别名 | ✅ | 编译为 m7d |

## 使用方法

```bash
# 启动
m7d -c config.toml start

# 状态
m7d status

# 日志
MI7SOFT_LOG_FILE=/var/log/mi7soft-daemon.log m7d start
```

## 生产环境待办

- [ ] **告警机制** - 推荐 Webhook（钉钉/飞书/Slack）
- [ ] 健康检查 - 防假死（进程在但卡住）
- [ ] 资源监控 - CPU/内存超限告警
- [ ] API/Web UI - 远程管理
- [ ] 配置校验 - 启动时检查命令是否存在

## 技术决策

1. **状态持久化方式**: daemon 退出不杀业务进程，通过 .state 文件记录 PID
2. **日志方案**: tracing + tracing-appender，按天轮转
3. **命名**: 二进制重命名为 m7d 方便使用

## 踩过的坑

1. 最初业务进程是 daemon 子进程 → 后来改为独立运行，daemon 只监控
2. daemon 重启后无法恢复监控 → 添加了状态持久化

## 相关文件

- 配置: `config/daemon.toml`
- 测试脚本: `/tmp/test-script.php`
