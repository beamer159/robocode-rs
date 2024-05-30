mod messages;

use messages::{BotHandshake, GameStarted, Message, ServerHandshake};
use tungstenite::connect;

struct Robot;

impl Robot {
    fn run(&self) {
        let (mut socket, _) = connect("ws://localhost:7654").expect("Cannot connect");
        loop {
            println!("Listening...");
            let message = socket.read().expect("Cannot read from socket");
            if message.is_empty() {
                continue;
            }
            let message: Message = serde_json::from_str(message.to_text().expect("Cannot convert message to text")).expect("Cannot determine the message type");
            let response = match message {
                Message::ServerHandshake(server_handshake) => self.handle_server_handshake(server_handshake),
                Message::GameStartedEventForBot(game_started) => self.handle_game_started(game_started),
                _ => None
            };

            if let Some(response) = response {
                let response = serde_json::to_string(&response).expect("Cannot serialize message");
                socket.send(tungstenite::Message::Text(response)).expect("Cannot send message");
            }
        }
    }

    fn handle_server_handshake(&self, server_handshake: ServerHandshake) -> Option<Message> {
        let bot_handshake = BotHandshake {
            session_id: server_handshake.session_id.clone(),
            name: "Rusty".into(),
            version: "0.1".into(),
            authors: vec!["Brian".into()],
            secret: Some("6mCpzXkkO1quMYUN94Gq9g".into()),
            country_codes: None,
            description: None,
            homepage: None,
            is_droid: None,
            platform: None,
            programming_lang: None,
            team_id: None,
            team_name: None,
            team_version: None
        };
        Some(Message::BotHandshake(bot_handshake))
    }

    fn handle_game_started(&self, game_started: GameStarted) -> Option<Message> {
        Some(Message::BotReady)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect() {
        let bot = Robot {};
        bot.run();
    }
}
