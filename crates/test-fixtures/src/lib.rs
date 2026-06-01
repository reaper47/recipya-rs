use std::fs::File;
use std::io::{Cursor, Read};

use axum_test::{TestResponse, TestWebSocket};
use uuid::Uuid;

type Result<T> = core::result::Result<T, Box<dyn std::error::Error>>;

/// The HTML returned when the websocket notification should be hidden.
pub const HIDDEN_WS_NOTIFICATION: &str = r#"<div id="ws-notification-container" class="z-20 fixed bottom-0 right-0 p-6 cursor-default hidden"><div class="bg-blue-500 text-white px-4 py-2 rounded shadow-md"><p class="font-medium text-center pb-1"></p><div class="flex justify-between items-center text-sm mb-2"><span class="font-semibold">-1 of -1</span><span class="font-semibold">0.0%</span></div><div id="export-progress"><progress max="100" value="0.00"></progress></div></div></div>"#;

pub struct RecipeImages {
    pub main: Uuid,
    pub additional: Uuid,
    pub video: Uuid,
}

impl Default for RecipeImages {
    fn default() -> Self {
        Self {
            main: Uuid::new_v4(),
            additional: Uuid::new_v4(),
            video: Uuid::new_v4(),
        }
    }
}

/// Asserts that the response HTML contains all of the expected strings.
pub fn assert_html(got: &TestResponse, want: &[&str]) {
    for s in want {
        got.assert_text_contains(s);
    }
}

/// Asserts that the response HTML does not contain any of the unwanted strings.
///
/// # Panics
///
/// Panics if any of the unwanted strings are found in the response HTML.
pub fn assert_not_in_html(got: &TestResponse, not_want: &[&str]) -> Result<()> {
    let text = got.text();
    for s in not_want {
        assert!(
            !text.contains(s),
            "expected `{s}` not to be in html: {}",
            got.text()
        );
    }

    Ok(())
}

/// Asserts that the websocket server sent the wanted message.
pub async fn assert_ws_message(server: &mut TestWebSocket, want: &str) {
    let _ = server.receive_message().await;
    server.assert_receive_text_contains(want).await;
}

/// Collects a specified number of messages from the websocket server.
pub async fn collect_ws_messages(server: &mut TestWebSocket, count: usize) -> Vec<String> {
    let mut messages = Vec::with_capacity(count);
    for _ in 0..count {
        let _ = server.receive_message().await;
        messages.push(server.receive_text().await);
    }
    messages
}

/// Asserts that the websocket server sent all wanted messages, in any order.
///
/// # Panics
///
/// Panics if any of the wanted messages are not received.
pub async fn assert_ws_messages_any_order(server: &mut TestWebSocket, wants: &[&str]) {
    let count = wants.len();
    let mut received: Vec<String> = Vec::with_capacity(count);

    for _ in 0..count {
        let _ = server.receive_message().await;
        received.push(server.receive_text().await);
    }

    for want in wants {
        assert!(
            received.iter().any(|msg| msg.contains(want)),
            "Failed to find '{}' in received messages:\n{}",
            want,
            received.join("\n")
        );
    }
}

/// Opens a data test file and returns its contents as a `Cursor`.
///
/// # Panics
///
/// Panics if the file does not exist or cannot be read.
pub fn open_test_file(filename: &str) -> Cursor<Vec<u8>> {
    let path = format!("./tests/data/{filename}");
    let mut file = File::open(path).expect("File to exist");
    let mut buf = Vec::new();
    file.read_to_end(&mut buf).expect("Failed to read file");
    Cursor::new(buf)
}
