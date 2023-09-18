use serde_json::Value;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub enum Status {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "error")]
    Error,
}

#[derive(Debug, PartialEq, Deserialize)]
pub enum Color {
    Red,
    Orange,
    Green,
    Yellow,
    Aqua,
    Blue,
    Purple,
    Pink,
}

#[derive(Debug, Deserialize)]
pub struct GetApplicationInfoResult {
    pub status: Status,
    pub data: ApplicationData,
}

#[derive(Debug, Deserialize)]
pub struct ApplicationData {
    pub version: String,
    pub prerelease_version: Option<String>,
    #[serde(rename = "buildVersion")]
    pub build_version: String,
    #[serde(rename = "execPath")]
    pub exec_path: Option<String>,
    pub platform: String,
}

#[derive(Debug, Deserialize)]
pub struct Child {
    pub id: String,
    pub name: String,
    pub images: Option<Vec<Value>>,
    pub folders: Option<Vec<Value>>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    pub editable: Option<bool>,
    // pub imagesMappings: Option<Vec<Value>>,
    pub tags: Vec<String>,
    pub children: Vec<Child>,
    #[serde(rename = "isExpand")]
    pub is_expand: Option<bool>,
    pub size: Option<u64>,
    pub vstype: Option<String>,
    pub styles: Option<Styles>,
    #[serde(rename = "isVisible")]
    pub is_visible: Option<bool>,
    pub index: Option<u64>,
    #[serde(rename = "newFolderName")]
    pub new_folder_name: Option<String>,
    #[serde(rename = "imageCount")]
    pub image_count: Option<u64>,
    #[serde(rename = "descendantImageCount")]
    pub descendant_image_count: Option<u64>,
    pub pinyin: Option<String>,
    #[serde(rename = "extendTags")]
    pub extend_tags: Option<Vec<Value>>,
    pub covers: Option<Vec<Value>>,
    pub parent: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Styles {
    pub depth: u64,
    pub first: bool,
    pub last: bool,
}


#[derive(Debug, Deserialize)]
pub struct CreateFolderResult {
    pub status: Status,
    pub data: CreateFolderData,
}

#[derive(Debug, Deserialize)]
pub struct CreateFolderData {
    pub id: String,
    pub name: String,
    pub images: Vec<Value>,
    pub folders: Vec<Value>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    #[serde(rename = "imageMappings")]
    pub image_mappings: Value,
    pub tags: Vec<String>,
    pub children: Vec<Child>,
    #[serde(rename = "isExpand")]
    pub is_expand: bool,
}
    

#[derive(Debug, Deserialize)]
pub struct RenameFolderResult {
    pub status: Status,
    pub data: RenameFolderData,
}

#[derive(Debug, Deserialize)]
pub struct RenameFolderData {
    pub id: String,
    pub name: String,
    pub images: Vec<Value>,
    pub folders: Vec<Value>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    #[serde(rename = "imageMappings")]
    pub image_mappings: Value,
    pub tags: Vec<String>,
    pub children: Vec<Child>,
    #[serde(rename = "isExpand")]
    pub is_expand: bool,
    pub size: u64,
    pub vstype: String,
    pub styles: Styles,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    #[serde(rename = "$$hashKey")]
    pub hash_key_: String,
    #[serde(rename = "newFolderName")]
    pub new_folder_name: String,
    pub editable: bool,
    pub pinyin: String,
}


#[derive(Debug, Deserialize)]
pub struct UpdateFolderResult {
    pub status: Status,
    pub data: UpdateFolderData,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFolderData {
    pub id: String,
    pub name: String,
    pub images: Vec<Value>,
    pub folders: Vec<Value>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    #[serde(rename = "imagesMappings")]
    pub images_mappings: Value,
    pub tags: Vec<String>,
    pub children: Vec<Child>,
    #[serde(rename = "isExpand")]
    pub is_expand: bool,
    pub size: u64,
    pub vstype: String,
    pub styles: Styles,
    #[serde(rename = "isVisible")]
    pub is_visible: bool,
    #[serde(rename = "$$hashKey")]
    pub hash_key_: String,
    #[serde(rename = "newFolderName")]
    pub new_folder_name: String,
    pub editable: bool,
    pub pinyin: String,
}

// TODO: Implement this DeleteFolderResult

#[derive(Debug, Deserialize)]
pub struct GetFolderListResult {
    pub status: Status,
    pub data: Vec<Child>,
}


#[derive(Debug, Deserialize)]
pub struct FolderListData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub children: Option<Vec<Child>>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    pub tags: Vec<String>,
    #[serde(rename = "imageCount")]
    pub image_count: u64,
    #[serde(rename = "descendantImageCount")]
    pub descendant_image_count: Option<u64>,
    pub pinyin: String,
    #[serde(rename = "extendTags")]
    pub extend_tags: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GetRecentFolderListResult {
    pub status: Status,
    pub data: Vec<RecentFolderListData>,
}

#[derive(Debug, Deserialize)]
pub struct RecentFolderListData {
    pub id: String,
    pub name: String,
    pub description: String,
    pub children: Vec<Child>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    pub tags: Vec<String>,
    pub password: String,
    #[serde(rename = "passwordTips")]
    pub password_tips: String,
    pub images: Vec<Value>,
    #[serde(rename = "isExpand")]
    pub is_expand: bool,
    #[serde(rename = "newFolderName")]
    pub new_folder_name: String,
    #[serde(rename = "imagesMappings")]
    pub images_mappings: Value,
    #[serde(rename = "imageCount")]
    pub image_count: u64,
    #[serde(rename = "descendantImageCount")]
    pub descendant_image_count: Option<u64>,
    pub pinyin: String,
    #[serde(rename = "extendTags")]
    pub extend_tags: Vec<String>,
}


#[derive(Debug, Deserialize)]
pub struct AddItemFromUrlResult {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct Item {
    pub url: String,
    pub name: Option<String>,
    pub website: Option<String>,
    pub annotation: Option<String>,
    pub tags: Option<Vec<String>>,
    #[serde(rename = "modificationTime")]
    pub modification_time: Option<u64>,
    // OutgoingHttpHeaders is a type alias for OutgoingHttpHeaders
    pub headers: Option<OutgoingHttpHeaders>,
}

pub type OutgoingHttpHeaders = HashMap<String, String>;



#[derive(Debug, Deserialize)]
pub struct AddItemFromUrlsResult {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct AddItemFromPathResult {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct AddItemFromPathsResult {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct AddBookmarkResult {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct GetItemInfoResult {
    pub status: Status,
    pub data: ItemInfoData,
}

#[derive(Debug, Deserialize)]
pub struct ItemInfoData {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub ext: String,
    pub tags: Vec<String>,
    pub folders: Vec<String>,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    pub url: String,
    pub annotation: String,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    pub width: u64,
    pub height: u64,
    #[serde(rename = "noThumbnail")]
    pub no_thumbnail: bool,
    #[serde(rename = "lastModified")]
    pub last_modified: u64,
    pub palettes: Vec<Palettes>,
}

#[derive(Debug, Deserialize)]
pub struct Palettes {
    pub color: Vec<u64>,
    pub ratio: u64,
    #[serde(rename = "$$hashKey")]
    pub hash_key_: String,
}

#[derive(Debug, Deserialize)]
pub struct GetItemThumbnailResult {
    pub status: Status,
    pub data: String,
}

// TODO: Implemen Order

// export type Order = {
//     manual: "MANUAL";
//     createDateAsc: "CREATEDATE";
//     createDateDesc: "-CREATEDATE";
//     fileSizeAsc: "FILESIZE";
//     fileSizeDesc: "-FILESIZE";
//     nameAsc: "NAME";
//     nameDesc: "-NAME";
//     resolutionAsc: "RESOLUTION";
//     resolutionDesc: "-RESOLUTION";
// };
#[derive(Debug, Deserialize)]
pub enum Order {
    MANUAL,
    CREATEDATE,
    CREATEDATEDESC,
    BTIME,
    MTIME,
    FILESIZE,
    FILESIZEREVERSE,
    NAME,
    NAMEREVERSE,
    RESOLUTION,
    RESOLUTIONREVERSE,
}

impl Order {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Order::MANUAL => "MANUAL",
            Order::CREATEDATE => "CREATEDATE",
            Order::CREATEDATEDESC => "-CREATEDATE",
            Order::BTIME => "BTIME",
            Order::MTIME => "MTIME",
            Order::FILESIZE => "FILESIZE",
            Order::FILESIZEREVERSE => "-FILESIZE",
            Order::NAME => "NAME",
            Order::NAMEREVERSE => "-NAME",
            Order::RESOLUTION => "RESOLUTION",
            Order::RESOLUTIONREVERSE => "-RESOLUTION",
        }
    }
}
    
#[derive(Debug, Deserialize)]
pub struct GetItemListResult {
    pub status: Status,
    pub data: Vec<ItemListData>,
}

#[derive(Debug, Deserialize)]
pub struct ItemListData {
    pub id: String,
    pub name: String,
    pub size: u64,
    pub ext: String,
    pub tags: Vec<String>,
    pub folders: Vec<String>,
    #[serde(rename = "isDeleted")]
    pub is_deleted: bool,
    pub url: String,
    pub annotation: String,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    pub height: u64,
    pub width: u64,
    #[serde(rename = "lastModified")]
    pub last_modified: u64,
    pub palettes: Vec<Palettes>,
}

#[derive(Debug, Deserialize)]
pub struct MoveItemToTrashResult {
    pub status: Status,
}


#[derive(Debug, Deserialize)]
pub struct RefreshItemPaletteResult {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct RefreshThumbnailResult {
    pub status: Status,
}

#[derive(Debug, Deserialize)]
pub struct UpdateItemResult {
    pub status: Status,
    pub data: ItemInfoData,
}



/// Get Library Info
#[derive(Debug, Deserialize)]
pub struct GetLibraryInfoResult {
    pub status: Status,
    pub data: LibraryInfoData,
}

#[derive(Debug, Deserialize)]
pub struct LibraryInfoData {
    pub folders: Vec<Folder>,
    #[serde(rename = "smartFolders")]
    pub smart_folders: Vec<SmartFolders>,
    #[serde(rename = "quickAccess")]
    pub quick_access: Vec<Value>,
    #[serde(rename = "tagsGroups")]
    pub tags_groups: Vec<Value>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    #[serde(rename = "applicationVersion")]
    pub application_version: String,
}

#[derive(Debug, Deserialize)]
        // folders: {
        //     id: string;
        //     name: string;
        //     description: "";
        //     children: {
        //         id: string;
        //         name: string;
        //         description: string;
        //         children: unknown[];
        //         modificationTime: number;
        //         tags: string[];
        //         iconColor: string;
        //         password: string;
        //         passwordTips: string;
        //         coverId: string;
        //         orderBy: Order;
        //         sortIncrease: boolean;
        //     }[];
        //     modificationTime: number;
        //     tags: string[];
        //     iconColor: string;
        //     password: string;
        //     passwordTips: string;
        //     coverId: string;
        //     orderBy: Order;
        //     sortIncrease: boolean;
        // }[];
        //
pub struct Folder {
    pub id: String,
    pub name: String,
    pub description: String,
    pub children: Vec<Child>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    pub tags: Vec<String>,
    #[serde(rename = "iconColor")]
    pub icon_color: Option<String>,
    pub password: String,
    #[serde(rename = "passwordTips")]
    pub password_tips: String,
    #[serde(rename = "coverId")]
    pub cover_id: Option<String>,
    #[serde(rename = "orderBy")]
    pub order_by: Option<Order>,
    #[serde(rename = "sortIncrease")]
    pub sort_increase: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct SmartFolders {
    pub id: String,
    pub icon: Option<String>,
    pub name: String,
    pub description: Option<String>,
    #[serde(rename = "modificationTime")]
    pub modification_time: u64,
    pub conditions: Vec<Conditions>,
}

#[derive(Debug, Deserialize)]
pub struct Conditions {
    #[serde(rename = "match")]
    pub match_: String,
    pub rules: Vec<Rules>,
}

#[derive(Debug, Deserialize)]
pub struct Rules {
    pub method: String,
    pub property: String,
    pub value: Value,
}

#[derive(Debug, Deserialize)]
pub struct QuickAccess {
    #[serde(rename = "type")]
    pub type_: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct TagsGroups {
    pub id: String,
    pub name: String,
    pub tags: Vec<String>,
    pub color: Color,
}

#[derive(Debug, Deserialize)]
pub struct GetLibraryHistoryResult {
    pub status: Status,
    pub data: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SwitchLibraryResult {
    pub status: Status,
}

// pub struct EagleClient {
//     host: String,
//     port: u16,
//     url: String,
// }
