use lambda_flows::{request_received, send_response};
use flowsnet_platform_sdk::logger;
use std::collections::HashMap;
use serde_json::Value;
use std::io;

#[no_mangle]
#[tokio::main(flavor = "current_thread")]
pub async fn run() -> anyhow::Result<()> {
    request_received(|headers, qry, body| {
        handler(headers, qry, body)
    }).await;
    Ok(())
}

async fn handler(headers: HashMap<String, String>, qry: HashMap<String, String>, body: String) -> HashMap<String, String> {
    // Process the request data and generate a response
    let response = format!("Received request with headers: {:?}, query: {:?}, and body: {}", headers, qry, body);

    let mut response_map = HashMap::new();
    response_map.insert(String::from("response"), response);
    response_map
}


fn main() {
    let mut valid_users: HashMap<String, String> = HashMap::new();
    valid_users.insert(String::from("user1"), String::from("password1"));
    valid_users.insert(String::from("user2"), String::from("password2"));
    valid_users.insert(String::from("user3"), String::from("password3"));

    println!("Enter your username: ");
    let mut username = String::new();
    io::stdin().read_line(&mut username).expect("Failed to read line");
    let username = username.trim();

    println!("Enter your password: ");
    let mut password = String::new();
    io::stdin().read_line(&mut password).expect("Failed to read line");
    let password = password.trim();

    let (success, message) = login(username, password, &valid_users);

    if success {
        println!("{}", message);
    } else {
        println!("{}", message);
    }
}

fn login(username: &str, password: &str, valid_users: &HashMap<String, String>) -> (bool, String) {
    match valid_users.get(username) {
        Some(actual_password) if actual_password == password => (true, String::from("Login successful")),
        _ => (false, String::from("Login failed")),
    }
}

