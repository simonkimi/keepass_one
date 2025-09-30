use serde::{Deserialize, Serialize};
use crate::kdbx::xml::entities::group::Group;

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Root {
    #[serde(rename = "Group")]
    pub group: Group,
    /// When the user deletes an object (group, entry, ...), an item is created in this list. When synchronizing/merging database files, this information can be used to decide whether an object has been deleted.
    ///
    /// 当用户删除一个对象（组、条目…）时，会在此列表中创建一个项目。在同步/合并数据库文件时，此信息可用于确定对象是否已被删除。
    #[serde(rename = "DeletedObjects")]
    #[serde(default)]
    pub deleted_objects: DeletedObjects,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Default)]
pub struct DeletedObjects {
    #[serde(rename = "DeletedObject", default)]
    pub deleted_object: Vec<DeletedObject>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct DeletedObject {
    #[serde(rename = "UUID")]
    pub uuid: String,
    #[serde(rename = "DeletionTime")]
    pub deletion_time: String,
}
