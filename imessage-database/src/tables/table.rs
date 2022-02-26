use std::collections::HashMap;

use rusqlite::{Connection, OpenFlags, Result, Row, Statement};

/// Defines behavior for SQL Table data
pub trait Table {
    /// Serializes a single row of data to an instance of the struct that implements this Trait
    fn from_row(row: &Row) -> Result<Self>
    where
        Self: Sized;
    /// Gets a statment we can exectue to iterate over the data in the table
    fn get(db: &Connection) -> Statement;
}

/// Defines behavior for table data that can be cached in memory
pub trait Cacheable {
    type T;
    fn cache(db: &Connection) -> HashMap<i32, Self::T>;
}

/// Defines behavior for printing diagnostic information for a table
pub trait Diagnostic {
    /// Emit diagnostic data about the table to `stdout`
    fn run_diagnostic(db: &Connection);
}

/// Get a connection to the iMessage SQLite database
pub fn get_connection(path: &str) -> Connection {
    match Connection::open_with_flags(&path, OpenFlags::SQLITE_OPEN_READ_ONLY) {
        Ok(res) => res,
        Err(why) => panic!("Unable to read from chat database: {}\nEnsure full disk access is enabled for your terminal emulator in System Preferences > Security and Privacy > Full Disk Access", why),
    }
}

// Table Names
pub const HANDLE: &str = "handle";
pub const MESSAGE: &str = "message";
pub const CHAT: &str = "chat";
pub const ATTACHMENT: &str = "attachment";
pub const CHAT_MESSAGE_JOIN: &str = "chat_message_join";
pub const MESSAGE_ATTACHMENT_JOIN: &str = "message_attachment_join";
pub const CHAT_HANDLE_JOIN: &str = "chat_handle_join";

// Default information
pub const ME: &str = "Me";
pub const DEFAULT_PATH: &str = "Library/Messages/chat.db";