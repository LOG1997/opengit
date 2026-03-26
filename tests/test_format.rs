use opengit::format::{AddressType, format_git_url, get_domain_type, get_git_remote_url};

#[test]
fn domain_type_test() {
    let input = "github.com";
    let expected = AddressType::DomainName;
    assert_eq!(get_domain_type(input), expected);
}
#[test]
fn ip_type_test() {
    let input = "192.168.1.1";
    let expected = AddressType::IpAddress;
    assert_eq!(get_domain_type(input), expected);
}
#[test]
fn invalid_domain_type_test1() {
    let input = "localhost";
    let expected = AddressType::Invalid;
    assert_eq!(get_domain_type(input), expected);
}
#[test]
fn invalid_domain_type_test2() {
    let input = "192-189-11-111";
    let expected = AddressType::Invalid;
    assert_eq!(get_domain_type(input), expected);
}
#[test]
fn invalid_domain_type_test3() {
    let input = "255.255.255.259";
    let expected = AddressType::Invalid;
    assert_eq!(get_domain_type(input), expected);
}

#[test]
fn format_git_url_https_test() {
    let input = "https://github.com/log1997/opengit.git".to_string();
    let expected = "https://github.com/log1997/opengit";
    assert_eq!(format_git_url(input), expected);
}

#[test]
fn format_git_url_git_test() {
    let input = "git@github.com:log1997/opengit.git".to_string();
    let expected = "https://github.com/log1997/opengit";
    assert_eq!(format_git_url(input), expected);
}

#[test]
fn format_git_url_gitee_test() {
    let input = "https://gitee.com/log1997/opengit.git".to_string();
    let expected = "https://gitee.com/log1997/opengit";
    assert_eq!(format_git_url(input), expected);
}

#[test]
fn format_git_url_no_git_test() {
    let input = "https://gitee.com/log1997/opengit".to_string();
    let expected = "https://gitee.com/log1997/opengit";
    assert_eq!(format_git_url(input), expected);
}

#[test]
fn format_git_url_number_ip_test() {
    let input = "git@10.2.3.12:log1997/opengit.git".to_string();
    let expected = "http://10.2.3.12/log1997/opengit";
    assert_eq!(format_git_url(input), expected);
}

#[test]
fn get_git_remote_url_test() {
    let expected = "git@github.com:LOG1997/opengit.git";
    assert_eq!(get_git_remote_url(), Some(expected.to_string()));
}
