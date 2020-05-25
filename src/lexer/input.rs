use std::io::Read;

pub fn read() -> Option<char> {
    let input: Option<char> = std::io::stdin().bytes().next()
        .and_then(|result| result.ok()).map(|byte| byte as char);
    input
}
