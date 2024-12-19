ARG TARGETPLATFORM=linux/amd64

FROM --platform=$TARGETPLATFORM swr.cn-north-4.myhuaweicloud.com/ddn-k8s/docker.io/ubuntu:20.04 AS builder

# 设置时区，避免交互式提示
ENV DEBIAN_FRONTEND=noninteractive

# 更新系统并安装必要的依赖
RUN apt-get update && apt-get install -y \
    curl \
    build-essential \
    git \
    libssl-dev \
    pkg-config \
    cmake \
    unzip \
    gcc-x86-64-linux-gnu \
    && rm -rf /var/lib/apt/lists/*

# 安装 Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# 设置 Rust 环境变量
ENV PATH="/root/.cargo/bin:${PATH}"

# 添加x86_64-unknown-linux-gnu目标
RUN rustup target add x86_64-unknown-linux-gnu

# 确认 Rust 已经安装成功
RUN rustc --version && cargo --version

# 创建一个工作目录
WORKDIR /workspace

COPY . .

# 使用交叉编译命令
RUN cargo build --release --target x86_64-unknown-linux-gnu

# 第二阶段：创建精简镜像
FROM scratch
WORKDIR /app
# 只复制编译好的二进制文件
COPY --from=builder /workspace/target/x86_64-unknown-linux-gnu/release/clear_tools /app/

# 设置入口点
ENTRYPOINT ["/app/clear_tools"]

