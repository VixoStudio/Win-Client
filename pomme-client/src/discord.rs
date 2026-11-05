use std::sync::mpsc;

use discord_rich_presence::activity::*;
use discord_rich_presence::{DiscordIpc, DiscordIpcClient};

const DISCORD_CLIENT_ID: &str = "1489624876909330452";

fn base_activity(version: &str) -> Activity<'static> {
    Activity::new()
        .details(format!("WinModing — {version}"))
        .assets(
            Assets::new()
                .large_image("winmoding")
                .large_text("WinModing"),
        )
}

/// Presence commands the UI thread sends to the IPC thread.
enum Command {
    Activity(String, String),
    Clear,
}

pub struct DiscordPresence {
    sender: mpsc::Sender<Command>,
}

impl DiscordPresence {
    pub fn start(version: &str) -> Result<Self, Box<dyn std::error::Error>> {
        // The IPC handshake can block for tens of seconds when Discord isn't
        // running, so it happens off the main thread; a connect failure only
        // disables rich presence, never the game.
        let (sender, receiver) = mpsc::channel::<Command>();
        let version = version.to_owned();
        std::thread::Builder::new()
            .name("discord-presence".into())
            .spawn(move || ipc_thread(version, receiver))?;
        Ok(Self { sender })
    }

    pub fn set_in_menu(&mut self, version: &str) {
        let _ = self
            .sender
            .send(Command::Activity(version.to_owned(), "In the menu".into()));
    }

    pub fn playing_multiplayer(&mut self, version: &str) {
        let _ = self
            .sender
            .send(Command::Activity(version.to_owned(), "In a server".into()));
    }

    pub fn playing_singleplayer(&mut self, version: &str) {
        let _ = self
            .sender
            .send(Command::Activity(version.to_owned(), "In a world".into()));
    }
}

/// Owns the IPC client, reconnecting once per command if the pipe dropped.
fn ipc_thread(initial_version: String, receiver: mpsc::Receiver<Command>) {
    let mut client: Option<DiscordIpcClient> = None;
    let mut current_version = initial_version;
    while let Ok(command) = receiver.recv() {
        match command {
            Command::Activity(version, state) => {
                current_version = version;
                let activity = base_activity(&current_version).state(state.as_str());
                if client.is_none() {
                    let mut fresh = DiscordIpcClient::new(DISCORD_CLIENT_ID);
                    if fresh.connect().is_ok() {
                        client = Some(fresh);
                    }
                }
                if let Some(live) = client.as_mut()
                    && live.set_activity(activity).is_err()
                {
                    client = None;
                }
            }
            Command::Clear => {
                if let Some(live) = client.as_mut() {
                    if live.clear_activity().is_err() {
                        client = None;
                    }
                }
            }
        }
    }
}

impl Drop for DiscordPresence {
    fn drop(&mut self) {
        let _ = self.sender.send(Command::Clear);
    }
}
