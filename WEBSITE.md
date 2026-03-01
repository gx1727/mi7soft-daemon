# 官网访问指南

## 在线访问

官网已部署到 GitHub Pages：
**https://gx1727.github.io/mi7soft-daemon/**

## 本地预览

如果你想本地预览官网：

```bash
cd web
pnpm install
pnpm dev
```

然后访问 http://localhost:5173

## 网站内容

### 首页
- 项目介绍
- 核心特性概览
- 下载链接（GitHub Releases）
- 文档链接

### 特性页
- **多进程管理** - Swoole/Hyperf 支持，防止孤儿进程
- **高性能** - 6MB 二进制，零开销
- **日志系统** - 结构化日志，实时跟踪
- **自动重启** - 智能崩溃恢复
- **持久化存储** - SQLite 历史记录
- **灵活配置** - TOML + 热重载

### 关于页
- 项目历程
- 开源社区

### 联系页
- GitHub Issues
- 贡献指南

## 多语言支持

网站支持中英文切换：
- 中文（默认）：`/?lng=zh`
- 英文：`/?lng=en`

浏览器会自动检测语言偏好。

## 更新网站

1. 修改 `web/src/` 下的源文件
2. 更新 `web/src/locales/` 下的翻译文件
3. 本地测试：`cd web && pnpm dev`
4. 构建：`cd web && pnpm build`
5. 提交：`git add web/ && git commit`
6. 推送：`git push`

## 部署到 GitHub Pages

```bash
cd web
pnpm build
cd dist
git init
git add .
git commit -m "Deploy website"
git push -f git@github.com:gx1727/mi7soft-daemon.git master:gh-pages
```

或者使用自动化脚本：

```bash
# 创建部署脚本
cat > web/deploy.sh << 'DEPLOY'
#!/bin/bash
cd "$(dirname "$0")"
pnpm build
cd dist
git init
git add .
git commit -m "Deploy website $(date)"
git push -f git@github.com:gx1727/mi7soft-daemon.git master:gh-pages
DEPLOY

chmod +x web/deploy.sh
./web/deploy.sh
```

## 技术支持

如有问题，请在 GitHub 提交 Issue：
https://github.com/gx1727/mi7soft-daemon/issues
