use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "清理用户文件夹中的大文件工具",
    long_about = "清理用户文件夹中的大文件工具\n\n\
    配置文件格式 (JSON):\n\
    {\n  \
      \"tools_path\": \"/mnt/dolphin-fs/cc-labs-tools/labs/\",  // 需要清理的文件夹路径 (可选，默认值如示例)\n  \
      \"users\": [\"123456\", \"789012\", \"345678\"],           // 用户ID列表 (必填)\n  \
      \"size_limit\": 50,                                      // 文件大小限制(MB) (可选，默认50)\n  \
      \"delete_common\": false                                 // 是否删除common文件夹 (可选，默认false)\n\
    }\n\n\
    示例:\n  \
    ./clear_tools -f config.json\n  \
    ./clear_tools --config my_config.json"
)]
struct Args {
    /// 配置文件路径 (JSON格式)
    ///
    /// 配置文件必须包含以下字段:
    ///   - users: 用户ID数组 (必填)
    ///   - tools_path: 清理目录路径 (可选)
    ///   - size_limit: 文件大小限制MB (可选)
    ///   - delete_common: 是否删除common文件夹 (可选)
    #[arg(
        short = 'f',
        long = "config",
        value_name = "FILE",
        help = "配置文件路径",
        long_help = "配置文件路径 (JSON格式)\n\n\
        配置文件示例:\n\
        {\n  \
          \"tools_path\": \"/mnt/dolphin-fs/cc-labs-tools/labs/\",\n  \
          \"users\": [\"123456\", \"789012\"],\n  \
          \"size_limit\": 50,\n  \
          \"delete_common\": false\n\
        }"
    )]
    config_file: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    /// 需要清理的文件夹路径
    #[serde(default = "default_tools_path")]
    tools_path: String,

    /// 用户列表
    users: Vec<String>,

    /// 需要清理的文件大小限制(MB)
    #[serde(default = "default_size_limit")]
    size_limit: u64,

    /// 是否需要删除common文件夹
    #[serde(default)]
    delete_common: bool,
    

}

fn default_tools_path() -> String {
    "/mnt/dolphin-fs/cc-labs-tools/labs/".to_string()
}

fn default_size_limit() -> u64 {
    50
}

/// 处理指定目录下的大文件
/// 
/// # 参数
/// * `dir_path` - 要处理的目录路径
/// * `dir_name` - 目录名称（用于日志输出）
/// * `size_limit` - 文件大小限制（MB）
/// * `script_file` - 用于写入删除命令的脚本文件
/// 
/// # 返回值
/// 返回找到的大文件数量
fn process_directory(
    dir_path: &Path,
    dir_name: &str,
    size_limit: u64,
    script_file: &mut std::fs::File,
) -> usize {
    let mut files_found = 0;

    println!("  开始扫描{}目录: {}", dir_name, dir_path.display());

    // 遍历文件夹
    for entry in WalkDir::new(dir_path) {
        let entry = match entry {
            Ok(e) => e,
            Err(e) => {
                println!("  {}访问文件失败: {}{}", RED, e, RESET);
                continue;
            }
        };

        if !entry.file_type().is_file() {
            continue;
        }

        // 获取文件大小
        let metadata = match entry.metadata() {
            Ok(m) => m,
            Err(e) => {
                println!("  {}获取文件元数据失败: {}{}", RED, e, RESET);
                continue;
            }
        };

        // 检查文件大小是否大于用户输入的限制
        let file_size = metadata.len() / 1024 / 1024;
        // 获取文件名
        let file_name = entry.file_name().to_string_lossy().to_lowercase();

        // 如果文件名包含 ipynb 或 canvas，跳过处理
        if file_name.contains("ipynb") || file_name.contains("canvas") {
            continue;
        }

        if file_size >= size_limit {
            println!(
                "  {}发现大文件: {} - 大小：{}MB{}",
                RED,
                entry.path().display(),
                file_size,
                RESET
            );
            writeln!(script_file, "rm \"{}\"", entry.path().display()).unwrap();
            files_found += 1;
        }
    }

    println!(
        "  {}目录处理完成，找到 {} 个大文件",
        dir_name, files_found
    );

    files_found
}

fn main() {
    let args = Args::parse();

    // 读取配置文件
    let config_content = match fs::read_to_string(&args.config_file) {
        Ok(content) => content,
        Err(e) => {
            println!("{}读取配置文件失败: {}{}", RED, e, RESET);
            println!("配置文件路径: {}", args.config_file);
            return;
        }
    };

    // 解析配置文件
    let config: Config = match serde_json::from_str(&config_content) {
        Ok(cfg) => cfg,
        Err(e) => {
            println!("{}解析配置文件失败: {}{}", RED, e, RESET);
            return;
        }
    };

    println!("{}配置信息:{}", GREEN, RESET);
    println!("  清理文件夹路径: {}", config.tools_path);
    println!("  用户数量: {}", config.users.len());
    println!("  文件大小限制: {}MB", config.size_limit);
    println!(
        "  删除common文件夹: {}",
        if config.delete_common { "是" } else { "否" }
    );
    println!();

    // 检查文件路径是否存在
    if !Path::new(&config.tools_path).exists() {
        println!(
            "{}需要清理的文件夹路径不存在: {}{}",
            RED, config.tools_path, RESET
        );
        return;
    }

    // 检查用户列表是否为空
    if config.users.is_empty() {
        println!("{}错误: 用户列表为空，请在配置文件中添加用户{}", RED, RESET);
        return;
    }

    println!("{}找到 {} 个用户信息{}", GREEN, config.users.len(), RESET);
    println!();

    // 创建shell脚本文件
    let script_path = "/tmp/clear_tools.sh";
    let mut script_file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(script_path)
        .unwrap();

    // 写入脚本头部
    writeln!(script_file, "#!/bin/bash").unwrap();
    writeln!(script_file, "# 自动生成的清理脚本\n").unwrap();

    let mut total_files_found = 0;
    let mut total_common_dirs = 0;

    // 遍历所有用户
    for user in config.users {
        println!("正在处理用户 {} 的文件夹...", user);

        // 构建用户目录路径
        let user_dir = Path::new(&config.tools_path).join(&user);
        if !user_dir.exists() {
            println!("  用户目录不存在: {}", user_dir.display());
            continue;
        }

        // 处理 common 文件夹
        if config.delete_common {
            let common_dir = user_dir.join("common");
            if common_dir.exists() {
                println!(
                    "  {}添加删除common文件夹命令: {}{}",
                    GREEN,
                    common_dir.display(),
                    RESET
                );
                writeln!(script_file, "rm -rf \"{}\"", common_dir.display()).unwrap();
                total_common_dirs += 1;
            }
        }

        // 处理 jupyter 文件夹
        let user_jupyter_dir = user_dir.join("jupyter");
        if user_jupyter_dir.exists() {
            let files_found = process_directory(
                &user_jupyter_dir,
                "jupyter",
                config.size_limit,
                &mut script_file,
            );
            total_files_found += files_found;
        } else {
            println!("  用户 {} 的jupyter目录不存在，跳过", user);
        }

        // 处理 dify 文件夹
        let user_dify_dir = user_dir.join("dify");
        if user_dify_dir.exists() {
            let files_found = process_directory(
                &user_dify_dir,
                "dify",
                config.size_limit,
                &mut script_file,
            );
            total_files_found += files_found;
        } else {
            println!("  用户 {} 的dify目录不存在，跳过", user);
        }

        println!(
            "  用户 {} 的文件处理完成",
            user
        );
        println!();
    }

    // 修改脚本权限
    std::process::Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .output()
        .expect("Failed to chmod script file");

    println!(
        "{}==================== 处理完成 ===================={}",
        GREEN, RESET
    );
    println!("总计找到 {} 个需要删除的大文件", total_files_found);
    if config.delete_common {
        println!("总计找到 {} 个common文件夹", total_common_dirs);
    }
    println!("清理脚本已生成: {}{}{}", GREEN, script_path, RESET);
    println!("{}请检查脚本内容后执行清理操作{}", GREEN, RESET);
}
