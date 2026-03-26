use webbrowser;
// 使用默认浏览器打开当前链接
pub fn open_in_browser(url: String) -> Option<String> {
    let result = webbrowser::open(&url);
    match result {
        Ok(_) => {
            println!("already open git url:{}", url);
            Some(url)
        }
        Err(e) => {
            eprintln!("sorry: {}", e);
            None
        }
    }
}
