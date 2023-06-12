use anyhow::{Context, Ok};
use omegle_rs::{
    omegle::Omegle, status::OmegleStatus, types::chat_event::ChatEvent, types::lang::LangCode,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let status = OmegleStatus::get_omegle_status()
        .await
        .context("Could not get omegle status")?;

    let omegle = Omegle::new(status, vec![], LangCode::English);
    let session = omegle
        .new_chat()
        .await
        .context("Could not start a new chat")?;
    'mainloop: loop {
        let events = session
            .get_events()
            .await
            .context("Failed getting events")?;
        for event in events {
            match event {
                ChatEvent::Waiting => println!("Waiting"),
                ChatEvent::Connected => println!("Connected"),
                ChatEvent::StartedTyping => print!("Typing\r"),
                ChatEvent::Message(msg) => println!("Got message: {msg}"),
                ChatEvent::Disconnected => {
                    println!("Disconnected");
                    break 'mainloop;
                }
                _ => {}
            }
        }
    }

    Ok(())
}
