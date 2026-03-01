#!/bin/bash
set -e

echo "🚀 开始部署官网到 GitHub Pages..."

# 检查 pnpm 是否安装
if ! command -v pnpm &> /dev/null; then
    echo "❌ pnpm 未安装，正在安装..."
    npm install -g pnpm
fi

# 安装依赖
echo "📦 安装依赖..."
pnpm install

# 构建
echo "🔨 构建网站..."
pnpm build

# 部署到 gh-pages 分支
echo "📤 部署到 gh-pages 分支..."
cd dist
git init
git add .
git commit -m "Deploy website - $(date '+%Y-%m-%d %H:%M:%S')"
git push -f git@github.com:gx1727/mi7soft-daemon.git master:gh-pages

echo "✅ 部署完成！"
echo "🌐 访问地址: https://gx1727.github.io/mi7soft-daemon/"
