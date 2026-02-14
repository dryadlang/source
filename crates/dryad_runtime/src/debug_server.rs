use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use crate::debug::{SharedDebugState, DebugCommand, DebugEvent};
use serde_json;

pub struct DebugServer {
    state: SharedDebugState,
    addr: String,
}

impl DebugServer {
    pub fn new(state: SharedDebugState, addr: &str) -> Self {
        Self {
            state,
            addr: addr.to_string(),
        }
    }

    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        let listener = TcpListener::bind(&self.addr).await?;
        println!("🚀 Debug server listening on {}", self.addr);

        loop {
            let (socket, _) = listener.accept().await?;
            let state = self.state.clone();
            
            tokio::spawn(async move {
                if let Err(e) = handle_connection(socket, state).await {
                    eprintln!("❌ Debug connection error: {}", e);
                }
            });
        }
    }
}

async fn handle_connection(mut socket: TcpStream, state: SharedDebugState) -> Result<(), Box<dyn std::error::Error>> {
    socket.set_nodelay(true)?;
    
    loop {
        // 1. Check for incoming commands (non-blocking style)
        let mut buffer = [0; 4096];
        match socket.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                let message = String::from_utf8_lossy(&buffer[..n]);
                for line in message.lines() {
                    if let Ok(command) = serde_json::from_str::<DebugCommand>(line) {
                        let mut s = state.lock().unwrap();
                        s.command_queue.push(command);
                    }
                }
            }
            Err(ref e) if e.kind() == std::io::ErrorKind::WouldBlock => {
                // No data yet
            }
            Err(e) => return Err(e.into()),
        }

        // 2. Check for outgoing events
        let event = {
            let mut s = state.lock().unwrap();
            s.event_queue.pop()
        };

        if let Some(ev) = event {
            let json = serde_json::to_string(&ev)? + "\n";
            socket.write_all(json.as_bytes()).await?;
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(20)).await;
    }

    Ok(())
}
