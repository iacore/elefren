#![cfg_attr(not(feature = "toml"), allow(dead_code))]
#![cfg_attr(not(feature = "toml"), allow(unused_imports))]
mod register;

use crate::register::MastodonClient;
use elefren::entities::event::Event;
use std::error;
use std::thread;

#[cfg(feature = "toml")]
fn main() -> Result<(), Box<dyn error::Error>> {
    let mastodon = register::get_mastodon_data()?;
    let thread = thread::spawn(move || {
        println!("Read from stream. Exit with '^C'");
        for event in mastodon
            .streaming_public()
            .expect("read message from stream")
        {
            match event {
                Event::Update(status) => {
                    let summary: String = status.content.chars().take(30).collect();
                    println!("Update: {} - {}", status.id, summary);
                },
                Event::Notification(notification) => {
                    println!("Notification: {:#?}", notification.notification_type)
                },
                Event::Delete(id) => println!("Delete: {}", id),
                Event::FiltersChanged => println!("Filterchanged"),
            };
        }
    });

    thread.join().expect("Thread failed");

    Ok(())
}

#[cfg(not(feature = "toml"))]
fn main() {
    println!(
        "examples require the `toml` feature, run this command for this example:\n\ncargo run \
         --example streaming_public --features toml\n"
    );
}
