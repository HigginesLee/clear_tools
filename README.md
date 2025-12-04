# Clear Tools

一个用于清理用户文件夹中大文件的工具，支持批量处理和自动生成清理脚本。

## 功能特性

- 支持通过配置文件指定清理参数
- 批量扫描用户文件夹，查找超过指定大小的文件
- 可选择性删除 common 文件夹
- 自动跳过 `.ipynb` 和 `canvas` 文件
- 生成可审查的 shell 脚本，确保安全执行
- 彩色输出，便于识别重要信息

## 安装

确保已安装 Rust 工具链，然后执行：

```bash
cargo build --release
```

编译后的可执行文件位于 `target/release/clear_tools`

## 配置文件格式

配置文件使用 JSON 格式，包含以下字段：

```json
{
  "tools_path": "/mnt/dolphin-fs/cc-labs-tools/labs/",
  "users": ["123456", "789012", "345678"],
  "size_limit": 50,
  "delete_common": false
}
```

### 配置项说明

| 字段 | 类型 | 必填 | 默认值 | 说明 |
|------|------|------|--------|------|
| `tools_path` | String | 否 | `/mnt/dolphin-fs/cc-labs-tools/labs/` | 需要清理的文件夹根路径 |
| `users` | Array | 是 | - | 需要处理的用户ID列表 |
| `size_limit` | Number | 否 | `50` | 文件大小限制（单位：MB），超过此大小的文件将被标记删除 |
| `delete_common` | Boolean | 否 | `false` | 是否删除用户目录下的 common 文件夹 |

### 用户列表说明

`users` 字段是一个字符串数组，包含所有需要处理的用户 ID。程序会遍历每个用户ID，处理对应的用户目录。

## 使用方法

### 基本用法

通过 `-f` 或 `--config` 参数指定配置文件：

```bash
./clear_tools -f config.json
```

或者：

```bash
./clear_tools --config config.json
```

### 完整示例

1. 创建配置文件 `my_config.json`：

```json
{
  "tools_path": "/mnt/dolphin-fs/cc-labs-tools/labs/",
  "users": ["user001", "user002", "user003"],
  "size_limit": 100,
  "delete_common": true
}
```

2. 执行清理工具：

```bash
./clear_tools -f my_config.json
```

3. 检查生成的脚本：

```bash
cat /tmp/clear_tools.sh
```

4. 确认无误后执行清理：

```bash
bash /tmp/clear_tools.sh
```

## 工作流程

1. 读取并解析配置文件
2. 验证路径的有效性
3. 从配置文件获取用户列表
4. 对每个用户进行处理：
   - 如果配置了删除 common 文件夹，添加删除命令
   - 扫描用户的 jupyter 目录
   - 查找超过大小限制的文件（跳过 .ipynb 和 canvas 文件）
   - 将删除命令写入脚本
5. 生成可执行的 shell 脚本到 `/tmp/clear_tools.sh`
6. 输出处理统计信息

## 安全特性

- **不会直接删除文件**：所有删除操作都先写入脚本，需要手动审查后执行
- **路径验证**：在执行前会验证配置的路径是否存在
- **文件保护**：自动跳过 Jupyter notebook (.ipynb) 和 canvas 相关文件
- **详细日志**：输出详细的处理信息，便于追踪

## 输出说明

程序会输出以下信息：

- 🟢 绿色：成功信息、配置信息、统计信息
- 🔴 红色：错误信息、发现的大文件
- ⚪ 白色：常规处理信息

## 示例输出

```
配置信息:
  清理文件夹路径: /mnt/dolphin-fs/cc-labs-tools/labs/
  用户数量: 3
  文件大小限制: 50MB
  删除common文件夹: 是

找到 3 个用户信息

正在处理用户 user001 的文件夹...
  添加删除common文件夹命令: /mnt/dolphin-fs/cc-labs-tools/labs/user001/common
  开始扫描目录: /mnt/dolphin-fs/cc-labs-tools/labs/user001/jupyter
  发现大文件: /mnt/dolphin-fs/cc-labs-tools/labs/user001/jupyter/data.csv - 大小：156MB
  用户 user001 的文件处理完成，找到 1 个大文件

==================== 处理完成 ====================
总计找到 5 个需要删除的大文件
总计找到 3 个common文件夹
清理脚本已生成: /tmp/clear_tools.sh
请检查脚本内容后执行清理操作
```

## 故障排除

### 配置文件读取失败

确保：
- 配置文件路径正确
- 配置文件格式为有效的 JSON
- 文件具有读取权限
- users 字段存在且为数组格式

### 路径不存在

确保：
- `tools_path` 指向的目录存在
- 程序有权限访问该路径

### JSON 解析失败

确保：
- 配置文件是有效的 JSON 格式
- users 字段是一个字符串数组
- JSON 文件使用 UTF-8 编码

## 依赖项

- `serde` - JSON 序列化/反序列化
- `serde_json` - JSON 解析
- `walkdir` - 递归目录遍历
- `clap` - 命令行参数解析

## 许可证

[根据您的需要添加许可证信息]

## 贡献

欢迎提交 Issue 和 Pull Request！