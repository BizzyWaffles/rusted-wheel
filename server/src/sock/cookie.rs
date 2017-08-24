use ws;
use std::collections::HashMap;

pub fn parse_cookies (req: &ws::Request) -> HashMap<String, String> {
    req.header("cookie")
        .and_then(|cookies_bytes| String::from_utf8(cookies_bytes.to_vec()).ok())
        .unwrap_or(String::from(""))
        .rsplit(";")
        .filter_map(|cookie_string| {
            let mut cookie_pair = cookie_string.split("=");
            match (cookie_pair.next(), cookie_pair.next()) {
                (Some(name), Some(value)) => {
                    Some((String::from(name.trim()), String::from(value.trim())))
                },
                _ => None
            }
        })
        .collect()
}

pub fn put_cookie (name: String, value: String, resp: &mut ws::Response) {
    let headers = resp.headers_mut();
    let cookie_bytes = format!("{}={}", name, value).as_bytes().to_vec();
    headers.push((String::from("Set-Cookie"), cookie_bytes));
}
