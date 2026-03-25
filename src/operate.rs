use webbrowser;
// 使用默认浏览器打开当前链接
pub fn open_in_browser(url: String) {
    let result = webbrowser::open(&url);
    match result {
        Ok(_) => println!("已打开git远程仓库链接:{}", url),
        Err(e) => eprintln!("无法打开: {}", e),
    }
}
