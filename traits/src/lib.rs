//! Common traits.

#[allow(dead_code)]

mod flush;
pub use flush::Flush as Flush;

mod save;
pub use save::Save as Save;

mod savetofile;
pub use savetofile::SaveToFile as SaveToFile;

mod setfile;
pub use setfile::SetFile as SetFile;

mod empty;
pub use empty::Empty as Empty;
