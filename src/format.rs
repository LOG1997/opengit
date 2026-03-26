use addr::parse_domain_name;
use git2::Repository;
use std::net::IpAddr;

#[derive(Debug, PartialEq)]
pub enum AddressType {
    IpAddress,
    DomainName,
    Invalid,
}
/// 获取当前 Git 仓库的远程仓库 URL（默认 origin）
pub fn get_git_remote_url() -> Option<String> {
    let repo = Repository::open_from_env().ok()?;
    let remote = repo.find_remote("origin").ok()?;
    remote.url().map(|s| s.to_string())
}
// 获取域名类型
pub fn get_domain_type(s: &str) -> AddressType {
    let domain = parse_domain_name(s);
    if domain.is_ok() && domain.unwrap().has_known_suffix() {
        return AddressType::DomainName;
    } else if s.parse::<IpAddr>().is_ok() {
        return AddressType::IpAddress;
    }
    AddressType::Invalid
}
// 格式化url为https开头
pub fn format_git_url(url: String) -> String {
    let mut result_url = url.trim().to_string();
    let prefix = "git@";
    let suffix = ".git";
    if result_url.starts_with(prefix) {
        let without_git = &result_url[4..];
        if let Some((domain, path)) = without_git.split_once(':') {
            let http_prefix = match get_domain_type(domain) {
                AddressType::DomainName => "https://",
                AddressType::IpAddress => "http://",
                AddressType::Invalid => "",
            };
            result_url = format!("{}{}/{}", http_prefix, domain, path);
        }
    }
    if result_url.ends_with(suffix) {
        result_url = result_url[..result_url.len() - suffix.len()].to_string();
    }
    result_url
}
