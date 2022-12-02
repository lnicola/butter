use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use zbus::zvariant::Type;

#[derive(Clone, Debug, Default, PartialEq, Eq, Deserialize, Serialize, Type)]
pub struct BtrfsFilesystem {
    pub label: String,
    pub uuid: Uuid,
    // TODO: PathBuf
    pub devices: Vec<String>,
    // TODO: PathBuf
    /// mount dirs by subvol
    pub mounts: HashMap<String, Vec<String>>,
}
