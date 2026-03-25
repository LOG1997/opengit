use git2::Repository;
/// 获取当前 Git 仓库的远程仓库 URL（默认 origin）
pub fn get_git_remote_url() -> Option<String> {
    let repo = Repository::open_from_env().ok()?;
    let remote = repo.find_remote("origin").ok()?;
    remote.url().map(|s| s.to_string())
}
// 判断是否数字ip地址
pub fn is_number_ip(ip: &str) -> bool {
    ip.chars().all(|c| c.is_ascii_digit())
}
// 格式化url为https开头
pub fn format_git_url(url: String) -> String {
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
