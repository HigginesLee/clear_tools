# 构建阶段
FROM rust:1.80.1-alpine AS builder

# 安装构建依赖
RUN apk add --no-cache musl-dev

# 设置工作目录
WORKDIR /app

# 复制项目文件
COPY Cargo.toml Cargo.lock ./
COPY src ./src

# 构建 release 版本
RUN cargo build --release --target x86_64-unknown-linux-musl

# 运行阶段 - 使用 alpine 基础镜像以支持 shell 脚本生成
FROM alpine:latest

# 安装必要的运行时依赖
RUN apk add --no-cache bash

# 创建工作目录
WORKDIR /app

# 从构建阶段复制二进制文件
COPY --from=builder /app/target/x86_64-unknown-linux-musl/release/clear_tools /usr/local/bin/clear_tools

# 复制配置文件示例
COPY config.example.json /app/config.example.json
COPY README.md /app/README.md

# 创建挂载点目录
RUN mkdir -p /config /data

# 设置可执行权限
RUN chmod +x /usr/local/bin/clear_tools

# 环境变量
ENV CONFIG_FILE=/config/config.json

# 说明
LABEL maintainer="your-email@example.com" \
      description="Clear Tools - 用于清理用户文件夹中大文件的工具" \
      version="0.2.0"

# 默认命令：显示帮助信息
CMD ["clear_tools", "--help"]

# 使用示例:
# docker build -t clear_tools .
# docker run -v $(pwd)/config.json:/config/config.json -v /mnt/dolphin-fs:/mnt/dolphin-fs clear_tools clear_tools -f /config/config.json