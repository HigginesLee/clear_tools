use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::Path;
use walkdir::WalkDir;

const RED: &str = "\x1b[31m";
const GREEN: &str = "\x1b[32m";
const RESET: &str = "\x1b[0m";

fn main() {
    // 读取用户输入
    print!("请输入需要清理的文件夹路径('/mnt/disk2/labs-tools/'): ");
    std::io::stdout().flush().unwrap();
    let mut tools_path = String::new();
    std::io::stdin().read_line(&mut tools_path).unwrap();

    // 先处理输入字符串，移除换行符
    tools_path = tools_path.trim().to_string();

    if tools_path.is_empty() {
        tools_path = "/mnt/disk2/labs-tools/".to_string();
    }

    // 检查文件路径是否存在
    if !std::path::Path::new(&tools_path).exists() {
        println!("需要清理的文件夹路径不存在: {}", tools_path);
        return;
    }


    let current_exe = std::env::current_exe().unwrap();
    let current_parent_dir = current_exe.parent().unwrap();

    print!("请输入保存用户文件信息路径({})：", current_parent_dir.join("student.json").to_str().unwrap());
    std::io::stdout().flush().unwrap();
    let mut save_path = String::new();
    std::io::stdin().read_line(&mut save_path).unwrap();
    save_path = save_path.trim().to_string();

    if save_path.is_empty() {
        // 获取当前编译程序目录下的student.json文件
        save_path = current_parent_dir
            .join("student.json")
            .to_str()
            .unwrap()
            .to_string();
    }
    if !std::path::Path::new(&save_path).exists() {
        println!("保存用户文件信息路径不存在: {}", save_path);
        return;
    }

    print!("请输入需要清理的文件大小限制(MB，默认50): ");
    std::io::stdout().flush().unwrap();
    let mut size_limit = String::new();
    std::io::stdin().read_line(&mut size_limit).unwrap();
    let size_limit: u64 = size_limit.trim().parse().unwrap_or(50);

    print!("是否需要删除common文件夹？(y/n): ");
    std::io::stdout().flush().unwrap();
    let mut delete_common = String::new();
    std::io::stdin().read_line(&mut delete_common).unwrap();
    let delete_common = delete_common.trim().to_lowercase() == "y";

    // 读取JSON文件
    let json_content = match fs::read_to_string(&save_path) {
        Ok(content) => content,
        Err(e) => {
            println!("读取JSON文件失败: {}", e);
            return;
        }
    };

    // 解析JSON数组
    let users: Vec<String> = match serde_json::from_str(&json_content) {
        Ok(info) => info,
        Err(e) => {
            println!("解析JSON失败: {}", e);
            return;
        }
    };

    println!("找到 {} 个用户信息", users.len());

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

    // 遍历所有用户
    for user in users {
        println!("正在处理用户 {} 的文件夹...", user);

        // 构建用户目录路径
        let user_dir = Path::new(&tools_path).join(&user);
        if !user_dir.exists() {
            println!("用户目录不存在: {}", user_dir.display());
            continue;
        }

        // 处理 common 文件夹
        if delete_common {
            let common_dir = user_dir.join("common");
            if common_dir.exists() {
                println!("添加删除common文件夹命令: {}", common_dir.display());
                writeln!(script_file, "rm -rf \"{}\"", common_dir.display()).unwrap();
            }
        }

        let user_jupyter_dir = user_dir.join("jupyter");

        println!("开始扫描目录: {}", user_jupyter_dir.display());

        if !user_jupyter_dir.exists() {
            println!("用户 {} 的jupyter目录不存在,跳过执行", user);
            continue;
        }

        // 遍历文件夹
        for entry in WalkDir::new(&user_jupyter_dir) {
            let entry = match entry {
                Ok(e) => e,
                Err(e) => {
                    println!("访问文件失败: {}", e);
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
                    println!("获取文件元数据失败: {}", e);
                    continue;
                }
            };

            // 检查文件大小是否大于用户输入的限制
            let file_size = metadata.len() / 1024 / 1024;
            if file_size >= size_limit {
                println!(
                    "{}发现大文件: {} - 大小：{}MB{}",
                    RED,
                    entry.path().display(),
                    file_size,
                    RESET
                );
                writeln!(script_file, "rm \"{}\"", entry.path().display()).unwrap();
            }
        }

        println!("用户 {} 的文件处理完成", user);
    }

    // 修改脚本权限
    std::process::Command::new("chmod")
        .arg("+x")
        .arg(script_path)
        .output()
        .expect("Failed to chmod script file");

    println!("清理脚本已生成: {}{}{}", GREEN, script_path, RESET);
    println!("请检查脚本内容后执行清理操作");
}
