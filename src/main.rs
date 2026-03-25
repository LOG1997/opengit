mod format;
mod operate;
use crate::format::{format_git_url, get_git_remote_url};
use crate::operate::open_in_browser;
fn main() {
    let remote_url = get_git_remote_url();
    if remote_url.is_none() {
        println!("当前目录不是Git仓库 或 没有找到远程仓库名称");
        return;
    }
    let http_url = format_git_url(remote_url.unwrap());
    open_in_browser(http_url);
}
