use std::ops::Deref;

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Color(pub u8, pub u8, pub u8);

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ItemId(pub String);

impl Deref for ItemId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct FolderId(pub String);

impl Deref for FolderId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct TagsGroupId(pub String);

impl Deref for TagsGroupId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Application {
    pub version: String,
    pub prerelease_version: Option<String>,
    pub build_version: String,
    pub platform: String,
    pub preferences: Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Palette {
    pub color: Color,
    pub ratio: f64,
    #[serde(rename = "$$hashKey")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hash_key: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub id: ItemId,
    pub name: String,
    pub ext: String,
    pub tags: Vec<String>,
    pub folders: Vec<FolderId>,
    pub is_deleted: bool,
    pub url: String,
    pub annotation: String,
    pub width: usize,
    pub height: usize,
    pub size: usize,
    pub no_thumbnail: Option<bool>,
    pub modification_time: i64,
    pub last_modified: i64,
    pub palettes: Vec<Palette>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FolderColor {
    Red,
    Orange,
    Green,
    Yellow,
    Aqua,
    Blue,
    Purple,
    Pink,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Folder {
    pub id: FolderId,
    pub name: String,
    pub description: Option<String>,
    pub modification_time: i64,
    pub children: Vec<Folder>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SmartFolder {
    pub id: FolderId,
    pub name: String,
    pub description: Option<String>,
    pub modification_time: i64,
    pub children: Vec<SmartFolder>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickAccess {}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TagsGroup {
    pub id: TagsGroupId,
    pub name: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryInfo {
    pub path: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    pub folders: Vec<Folder>,
    pub smart_folders: Vec<SmartFolder>,
    pub quick_access: QuickAccess,
    pub tags_groups: Vec<TagsGroup>,
    pub modification_time: i64,
    pub application_version: String,
    pub library: LibraryInfo,
}
