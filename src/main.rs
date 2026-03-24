use git2::Repository;
use std::path::PathBuf;
use webbrowser;
/// 获取当前 Git 仓库的远程仓库 URL（默认 origin）
fn get_git_remote_url(dir: PathBuf) -> Option<String> {
    let repo = Repository::open(&dir).ok()?;
    let remote = repo.find_remote("origin").ok()?;
    remote.url().map(|s| s.to_string())
}
// 判断是否数字ip地址
fn is_number_ip(ip: &str) -> bool {
    ip.chars().all(|c| c.is_ascii_digit())
}
// 格式化url为https开头
// 将git@github.com:替换为https://github.com/
fn format_git_url(url: String) -> String {
    let mut result_url = url.trim().to_string();
    let prefix = "git@";
    let suffix = ".git";
    if result_url.starts_with(prefix) {
        let without_git = &result_url[4..];
        if let Some((domain, path)) = without_git.split_once(':') {
            let http_prefix = if is_number_ip(domain) {
                "http://"
            } else {
                "https://"
            };
            result_url = format!("{}{}/{}", http_prefix, domain, path);
        }
    }
    if result_url.ends_with(suffix) {
        result_url = result_url[..result_url.len() - suffix.len()].to_string();
    }
    result_url
}
// 使用默认浏览器打开当前链接
fn open_in_browser(url: String) {
    let result = webbrowser::open(&url);
    match result {
        Ok(_) => println!("已打开git远程仓库链接:{}", url),
        Err(e) => eprintln!("无法打开: {}", e),
    }
}

fn main() {
    let dir = std::env::current_dir().unwrap();
    let remote_url = get_git_remote_url(dir);
    if remote_url.is_none() {
        println!("当前目录不是Git仓库 或 没有找到远程仓库名称");
        return;
    }
    let http_url = format_git_url(remote_url.unwrap());
    open_in_browser(http_url);
}
