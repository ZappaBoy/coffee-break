use std::env;
use std::process::Command;
use notify_rust::{Notification, Timeout};

fn main() {
    let username = env::var("USER").unwrap_or("User".to_string());

    Notification::new()
        .summary(&*format!("Dear {username}"))
        .body("Would you like a coffee?")
        .icon("/usr/share/icons/custom/coffee.png")
        .timeout(Timeout::Milliseconds(60000))
        .action("clicked", "Sure!")
        .show().unwrap()
        .wait_for_action(|action| match action {
            "clicked" => prepare_coffee(),
            _ => ()
        });
}


fn prepare_coffee() {
    Command::new("domo")
        .arg("coffee")
        .arg("press")
        .spawn()
        .expect("Cannot prepare coffee");
}
