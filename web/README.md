# mi7soft-daemon 官网

这是 mi7soft-daemon 项目的官方网站源码。

## 技术栈

- **React 18** - 前端框架
- **TypeScript** - 类型安全
- **Vite** - 构建工具
- **Tailwind CSS** - 样式框架
- **Framer Motion** - 动画库
- **React Router** - 路由管理
- **i18next** - 国际化

## 开发

```bash
# 安装依赖
pnpm install

# 启动开发服务器
pnpm dev

# 构建生产版本
pnpm build

# 预览生产构建
pnpm preview
```

## 部署

### 静态部署

```bash
# 构建
pnpm build

# 生成的文件在 dist/ 目录
# 可以部署到任何静态文件服务器
```

### GitHub Pages

```bash
# 安装 gh-pages
npm install -D gh-pages

# 添加到 package.json
{
  "scripts": {
    "deploy": "pnpm build && gh-pages -d dist"
  }
}

# 部署
pnpm deploy
```

## 目录结构

```
web/
├── public/              # 静态资源
├── src/
│   ├── components/      # React 组件
│   │   ├── layout/     # 布局组件（Navbar, Footer）
│   │   └── ui/         # UI 组件
│   ├── pages/          # 页面组件
│   │   ├── Home.tsx    # 首页
│   │   ├── Features.tsx # 特性页
│   │   ├── About.tsx   # 关于页
│   │   └── Contact.tsx # 联系页
│   ├── locales/        # 国际化文件
│   │   ├── en.json     # 英文
│   │   └── zh.json     # 中文
│   ├── i18n/           # i18n 配置
│   └── main.tsx        # 入口文件
├── index.html          # HTML 模板
├── vite.config.ts      # Vite 配置
├── tailwind.config.js  # Tailwind 配置
└── package.json        # 依赖配置
```

## 功能特性

### 已实现

- ✅ 响应式设计（移动端适配）
- ✅ 中英文双语支持
- ✅ 暗色主题
- ✅ 平滑动画
- ✅ GitHub 下载链接
- ✅ 文档链接
- ✅ 功能展示页面

### 待实现

- 🔧 在线配置生成器
- 🔧 实时演示
- 🔧 博客/更新日志
- 🔧 搜索功能

## 更新日志

### 2026-03-03

- 更新到 v0.1.3
- 添加 Cron 调度模式支持（6字段 cron 表达式）
- 添加进程级 check_interval 配置
- 添加表格化状态输出（类似 pm2）
- 修复 status 和 stop 命令问题
- 修复僵尸进程检测问题

### 2026-03-01

- 更新到 v0.1.2
- 添加进程组管理功能介绍
- 添加日志系统功能介绍
- 添加持久化存储功能介绍
- 添加 GitHub 下载和文档链接
- 更新中英文翻译

## 贡献

欢迎贡献代码！请查看主项目的贡献指南。

## 许可证

MIT License
