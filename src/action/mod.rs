use crate::{extra_struct, message::MSVistor, ExtendedMap};
use serde::{de::Visitor, Deserialize, Deserializer, Serialize};

#[cfg(feature = "app")]
mod ext;
#[cfg(feature = "app")]
pub use ext::*;

/// ## OneBot 12 标准动作
///
/// **动作请求**是应用端为了主动向 OneBot 实现请求服务而发送的数据。
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "action", content = "params", rename_all = "snake_case")]
pub enum StandardAction {
    // meta action
    GetLatestEvents(GetLatestEvents),
    GetSupportedActions(ExtendedMap),
    GetStatus(ExtendedMap),
    GetVersion(ExtendedMap),

    // message action
    SendMessage(SendMessage),
    DeleteMessage(DeleteMessage),
    GetMessage(GetMessage),

    // user action
    GetSelfInfo(ExtendedMap),
    GetUserInfo(GetUserInfo),
    GetFriendList(ExtendedMap),

    // group action
    GetGroupInfo(GetGroupInfo),
    GetGroupList(ExtendedMap),
    GetGroupMemberInfo(GetGroupMemberInfo),
    GetGroupMemberList(GetGroupMemberList),
    SetGroupName(SetGroupName),
    LeaveGroup(LeaveGroup),
    KickGroupMember(KickGroupMember),
    BanGroupMember(BanGroupMember),
    UnbanGroupMember(UnbanGroupMember),
    SetGroupAdmin(SetGroupAdmin),
    UnsetGroupAdmin(UnsetGroupAdmin),

    // guild action
    GetGuildInfo(GetGuildInfo),
    GetGuildList(ExtendedMap),
    GetChannelInfo(GetChannelInfo),
    GetChannelList(GetChannelList),
    GetGuildMemberInfo(GetGuildMemberInfo),
    GetGuildMemberList(GetGuildMemberList),
    SetGuildName(SetGuildName),
    SetChannelName(SetChannelName),
    LeaveGuild(LeaveGuild),

    // file
    UploadFile(UploadFile),
    UploadFileFragmented(UploadFileFragmented),
    GetFile(GetFile),
    GetFileFragmented(GetFileFragmented),
}

/// OneBot 12 扩展动作
///
/// 任何符合 OneBot 12 格式的动作均可序列化为该 struct
///
/// 如果需要使用该动作，可以使用 untagged enum 包裹该动作：
///
/// ```rust
/// use onebot_12::{StandardAction, ExtendedAction};
/// use serde::{Serialize, Deserialize};
///
/// #[derive(Serialize, Deserialize)]
/// #[serde(untagged)]
/// pub enum YourAction {
///     Standard(StandardAction),
///     Extended(ExtendedAction),
/// }
/// ```
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ExtendedAction {
    pub action: String,
    pub params: ExtendedMap,
}

macro_rules! impl_from(
    ($to:ident => $($sub: ident),*) => {
        $(impl From<$sub> for $to {
            fn from(from: $sub) -> Self {
                $to::$sub(from)
            }
        })*
    };
);

/// Action content for SendMessage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SendMessage {
    pub detail_type: String,
    pub group_id: Option<String>,
    pub user_id: Option<String>,
    pub guild_id: Option<String>,
    pub channel_id: Option<String>,
    #[serde(deserialize_with = "deserialize_message")]
    pub message: crate::Message,
    #[serde(flatten)]
    pub extra: ExtendedMap,
}

struct MessageVisitor;

impl<'de> Visitor<'de> for MessageVisitor {
    type Value = crate::Message;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("a message")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::SeqAccess<'de>,
    {
        let mut message = Vec::new();
        while let Some(segment) = seq.next_element()? {
            message.push(segment);
        }
        Ok(message)
    }

    fn visit_map<A>(self, map: A) -> Result<Self::Value, A::Error>
    where
        A: serde::de::MapAccess<'de>,
    {
        MSVistor::_visit_map(map).map(|s| vec![s])
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(vec![crate::MessageSegment::text(s.to_owned())])
    }
}

fn deserialize_message<'de, D>(deserializer: D) -> Result<crate::Message, D::Error>
where
    D: Deserializer<'de>,
{
    deserializer.deserialize_any(MessageVisitor)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetLatestEvents {
    #[serde(default)]
    pub limit: i64,
    #[serde(default)]
    pub timeout: i64,
    #[serde(flatten)]
    pub extra: ExtendedMap,
}
// onebot_action!(GetLatestEvents, limit: i64, timeout: i64);
extra_struct!(DeleteMessage, message_id: String);
extra_struct!(GetMessage, message_id: String);
extra_struct!(GetUserInfo, user_id: String);
extra_struct!(GetGroupInfo, group_id: String);
extra_struct!(GetGroupMemberList, group_id: String);
extra_struct!(LeaveGroup, group_id: String);
extra_struct!(GetGroupMemberInfo, group_id: String, user_id: String);
extra_struct!(KickGroupMember, group_id: String, user_id: String);
extra_struct!(BanGroupMember, group_id: String, user_id: String);
extra_struct!(UnbanGroupMember, group_id: String, user_id: String);
extra_struct!(SetGroupAdmin, group_id: String, user_id: String);
extra_struct!(UnsetGroupAdmin, group_id: String, user_id: String);
extra_struct!(SetGroupName, group_id: String, group_name: String);
extra_struct!(GetChannelInfo, guild_id: String, channel_id: String);
extra_struct!(GetChannelList, guild_id: String);
extra_struct!(GetGuildMemberInfo, guild_id: String, user_id: String);
extra_struct!(GetGuildMemberList, guild_id: String);
extra_struct!(SetGuildName, guild_id: String, guild_name: String);
extra_struct!(
    SetChannelName,
    guild_id: String,
    channel_id: String,
    channel_name: String
);
extra_struct!(LeaveGuild, guild_id: String);
extra_struct!(
    UploadFile,
    r#type: String,
    name: String,
    url: Option<String>,
    headers: Option<std::collections::HashMap<String, String>>,
    path: Option<String>,
    data: Option<Vec<u8>>,
    sha256: Option<String>
);
extra_struct!(GetFile, file_id: String, r#type: String);
extra_struct!(GetGuildInfo, guild_id: String);

/// Action content for UploadFileFragmented
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "stage", rename_all = "snake_case")]
pub enum UploadFileFragmented {
    Prepare {
        name: String,
        total_size: i64,
    },
    Transfer {
        file_id: String,
        offset: i64,
        size: i64,
        data: Vec<u8>,
    },
    Finish {
        file_id: String,
        sha256: String,
    },
}

/// Action content for GetFileFragmented
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "stage", rename_all = "snake_case")]
pub enum GetFileFragmented {
    Prepare {
        file_id: String,
    },
    Transfer {
        file_id: String,
        offset: i64,
        size: i64,
    },
}

impl_from!(
    StandardAction =>
    SendMessage,
    GetLatestEvents,
    DeleteMessage,
    GetUserInfo,
    GetGroupInfo,
    GetGroupMemberList,
    LeaveGroup,
    GetGroupMemberInfo,
    KickGroupMember,
    BanGroupMember,
    UnbanGroupMember,
    SetGroupAdmin,
    UnsetGroupAdmin,
    SetGroupName,
    UploadFile,
    UploadFileFragmented,
    GetFile,
    GetFileFragmented
);

#[macro_export]
macro_rules! onebot_action_ext {
    ($ext_name: ident => $($action: ident),*) => {
        #[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
        #[serde(tag = "action", content = "params", rename_all = "snake_case")]
        pub enum $ext_name {
            GetLatestEvents(walle_core::action::GetLatestEvents),
            GetSupportedActions(walle_core::ExtendedMap),
            GetStatus(walle_core::ExtendedMap),
            GetVersion(walle_core::ExtendedMap),
            SendMessage(walle_core::action::SendMessage),
            DeleteMessage(walle_core::action::DeleteMessage),
            GetMessage(walle_core::action::GetMessage),
            GetSelfInfo(walle_core::ExtendedMap),
            GetUserInfo(walle_core::action::GetUserInfo),
            GetFriendList(walle_core::ExtendedMap),
            GetGroupInfo(walle_core::action::GetGroupInfo),
            GetGroupList(walle_core::ExtendedMap),
            GetGroupMemberInfo(walle_core::action::GetGroupMemberInfo),
            GetGroupMemberList(walle_core::action::GetGroupMemberList),
            SetGroupName(walle_core::action::SetGroupName),
            LeaveGroup(walle_core::action::LeaveGroup),
            KickGroupMember(walle_core::action::KickGroupMember),
            BanGroupMember(walle_core::action::BanGroupMember),
            UnbanGroupMember(walle_core::action::UnbanGroupMember),
            SetGroupAdmin(walle_core::action::SetGroupAdmin),
            UnsetGroupAdmin(walle_core::action::UnsetGroupAdmin),
            GetGuildInfo(walle_core::action::GetGuildInfo),
            GetGuildList(walle_core::ExtendedMap),
            GetGuildMemberInfo(walle_core::action::GetGuildMemberInfo),
            GetGuildMemberList(walle_core::action::GetGuildMemberList),
            SetGuildName(walle_core::action::SetGuildName),
            LeaveGuild(walle_core::action::LeaveGuild),
            GetChannelInfo(walle_core::action::GetChannelInfo),
            GetChannelList(walle_core::action::GetChannelList),
            SetChannelName(walle_core::action::SetChannelName),
            UploadFile(walle_core::action::UploadFile),
            UploadFileFragmented(walle_core::action::UploadFileFragmented),
            GetFile(walle_core::action::GetFile),
            GetFileFragmented(walle_core::action::GetFileFragmented),
            $($action($action)),*
        }

        impl From<walle_core::StandardAction> for $ext_name {
            fn from(from: walle_core::StandardAction) -> $ext_name {
                match from {
                    walle_core::StandardAction::GetLatestEvents(action) => $ext_name::GetLatestEvents(action),
                    walle_core::StandardAction::GetSupportedActions(action) => $ext_name::GetSupportedActions(action),
                    walle_core::StandardAction::GetStatus(action) => $ext_name::GetStatus(action),
                    walle_core::StandardAction::GetVersion(action) => $ext_name::GetVersion(action),
                    walle_core::StandardAction::SendMessage(action) => $ext_name::SendMessage(action),
                    walle_core::StandardAction::DeleteMessage(action) => $ext_name::DeleteMessage(action),
                    walle_core::StandardAction::GetMessage(action) => $ext_name::GetMessage(action),
                    walle_core::StandardAction::GetSelfInfo(action) => $ext_name::GetSelfInfo(action),
                    walle_core::StandardAction::GetUserInfo(action) => $ext_name::GetUserInfo(action),
                    walle_core::StandardAction::GetFriendList(action) => $ext_name::GetFriendList(action),
                    walle_core::StandardAction::GetGroupInfo(action) => $ext_name::GetGroupInfo(action),
                    walle_core::StandardAction::GetGroupList(action) => $ext_name::GetGroupList(action),
                    walle_core::StandardAction::GetGroupMemberInfo(action) => $ext_name::GetGroupMemberInfo(action),
                    walle_core::StandardAction::GetGroupMemberList(action) => $ext_name::GetGroupMemberList(action),
                    walle_core::StandardAction::SetGroupName(action) => $ext_name::SetGroupName(action),
                    walle_core::StandardAction::LeaveGroup(action) => $ext_name::LeaveGroup(action),
                    walle_core::StandardAction::KickGroupMember(action) => $ext_name::KickGroupMember(action),
                    walle_core::StandardAction::BanGroupMember(action) => $ext_name::BanGroupMember(action),
                    walle_core::StandardAction::UnbanGroupMember(action) => $ext_name::UnbanGroupMember(action),
                    walle_core::StandardAction::SetGroupAdmin(action) => $ext_name::SetGroupAdmin(action),
                    walle_core::StandardAction::UnsetGroupAdmin(action) => $ext_name::UnsetGroupAdmin(action),
                    walle_core::StandardAction::GetGuildInfo(action) => $ext_name::GetGuildInfo(action),
                    walle_core::StandardAction::GetGuildList(action) => $ext_name::GetGuildList(action),
                    walle_core::StandardAction::GetGuildMemberInfo(action) => $ext_name::GetGuildMemberInfo(action),
                    walle_core::StandardAction::GetGuildMemberList(action) => $ext_name::GetGuildMemberList(action),
                    walle_core::StandardAction::SetGuildName(action) => $ext_name::SetGuildName(action),
                    walle_core::StandardAction::LeaveGuild(action) => $ext_name::LeaveGuild(action),
                    walle_core::StandardAction::GetChannelInfo(action) => $ext_name::GetChannelInfo(action),
                    walle_core::StandardAction::GetChannelList(action) => $ext_name::GetChannelList(action),
                    walle_core::StandardAction::SetChannelName(action) => $ext_name::SetChannelName(action),
                    walle_core::StandardAction::UploadFile(action) => $ext_name::UploadFile(action),
                    walle_core::StandardAction::UploadFileFragmented(action) => $ext_name::UploadFileFragmented(action),
                    walle_core::StandardAction::GetFile(action) => $ext_name::GetFile(action),
                    walle_core::StandardAction::GetFileFragmented(action) => $ext_name::GetFileFragmented(action),
                }
            }
        }

        $(impl From<$action> for $ext_name {
            fn from(from: $action) -> $ext_name {
                $ext_name::$action(from)
            }
        })*
    };
}

pub trait ActionType {
    fn content_type(&self) -> crate::utils::ContentType;
}

impl ActionType for StandardAction {
    fn content_type(&self) -> crate::utils::ContentType {
        match self {
            Self::UploadFile(_)
            | Self::UploadFileFragmented(_)
            | Self::GetFile(_)
            | Self::GetFileFragmented(_) => crate::utils::ContentType::MsgPack,
            _ => crate::utils::ContentType::Json,
        }
    }
}