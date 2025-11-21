use std::cell::RefCell;
use std::fs::OpenOptions;
use std::io::Write;
use std::sync::Mutex;

static SERVER_LOG: Mutex<()> = Mutex::new(());
static CLIENT_LOG: Mutex<()> = Mutex::new(());

thread_local! {
    static PIPE_CONTEXT: RefCell<PipeContext> = const { RefCell::new(PipeContext::Unknown) };
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum PipeContext {
    Server,
    Client,
    Unknown,
}

pub fn set_server_context() {
    PIPE_CONTEXT.with(|ctx| *ctx.borrow_mut() = PipeContext::Server);
}

pub fn set_client_context() {
    PIPE_CONTEXT.with(|ctx| *ctx.borrow_mut() = PipeContext::Client);
}

pub fn log_server(message: &str) {
    let _guard = SERVER_LOG.lock().unwrap();
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("test_server.log")
    {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}

pub fn log_client(message: &str) {
    let _guard = CLIENT_LOG.lock().unwrap();
    if let Ok(mut file) = OpenOptions::new()
        .create(true)
        .append(true)
        .open("test_client.log")
    {
        let timestamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let _ = writeln!(file, "[{}] {}", timestamp, message);
    }
}

pub fn log_write(message: &str) {
    PIPE_CONTEXT.with(|ctx| match *ctx.borrow() {
        PipeContext::Server => log_server(message),
        PipeContext::Client => log_client(message),
        PipeContext::Unknown => {
            // Log to both if context unknown
            log_server(&format!("[UNKNOWN_CTX] {}", message));
            log_client(&format!("[UNKNOWN_CTX] {}", message));
        }
    });
}
