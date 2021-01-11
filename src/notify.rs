use notify_rust::Notification;

pub fn notify(title: &str, body: &str) {
    let result = Notification::new()
        .summary(title)
        .body(body)
        .appname("autoclip - PfnGP")
        .show();
    if result.is_err() {
        println!("Failed to send notification.");
    }
}
