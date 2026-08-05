//! jarvis-app — фоновий демон/оркестратор поверх jarvis-core.
//!
//! TASK 001: порожній каркас, без бізнес-логіки. Наступні задачі додають
//! сюди IPC Boundary (WebSocket-сервер) і сам event loop, що звʼязує
//! Audio Capture → Listener → Stt → IntentEngine → Commands → Voices
//! через SessionCoordinator (jarvis-core-architecture-spec.md, §4, §6).

fn main() {
    println!("jarvis-app (skeleton)");
}
