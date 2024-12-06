use serde::{de::Visitor, Deserialize, Serialize};

/// 用于序列化和反序列化 OneBot 消息的 bytes 类型
///
/// json 序列化时，使用 base64 编码
///
/// msgpack 序列化时，使用原始 bytes
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OneBotBytes(pub Vec<u8>);

impl Serialize for OneBotBytes {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if std::any::type_name::<S>().starts_with("&mut serde_json") {
            use base64::Engine;
            return serializer
                .serialize_str(&base64::engine::general_purpose::STANDARD.encode(&self.0));
        }
        serializer.serialize_bytes(&self.0)
    }
}

impl From<Vec<u8>> for OneBotBytes {
    fn from(v: Vec<u8>) -> Self {
        Self(v)
    }
}

impl From<&[u8]> for OneBotBytes {
    fn from(v: &[u8]) -> Self {
        Self(v.to_vec())
    }
}

struct OBBVistor;

impl<'de> Visitor<'de> for OBBVistor {
    type Value = OneBotBytes;
    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expect json base64 or msgpack bytes")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        use base64::Engine;
        Ok(OneBotBytes(
            base64::engine::general_purpose::STANDARD
                .decode(v)
                .map_err(|_| serde::de::Error::custom("Not a valid base64 String"))?,
        ))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OneBotBytes(v.to_vec()))
    }

    fn visit_byte_buf<E>(self, v: Vec<u8>) -> Result<Self::Value, E>
    where
        E: serde::de::Error,
    {
        Ok(OneBotBytes(v))
    }
}

impl<'de> Deserialize<'de> for OneBotBytes {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(OBBVistor)
    }
}

#[test]
fn sertest() {
    let bytes = OneBotBytes(vec![0, 1, 2, 3]);
    assert_eq!("\"AAECAw==\"", &serde_json::to_string(&bytes).unwrap());
    assert_eq!(vec![196, 4, 0, 1, 2, 3], rmp_serde::to_vec(&bytes).unwrap());
}

#[test]
fn detest() {
    let bytes = OneBotBytes(vec![0, 1, 2, 3]);
    let json = "\"AAECAw==\"";
    assert_eq!(bytes, serde_json::from_str::<OneBotBytes>(&json).unwrap());
    let msgpack = vec![196, 4, 0, 1, 2, 3];
    assert_eq!(
        bytes,
        rmp_serde::from_slice::<OneBotBytes>(&msgpack).unwrap()
    );
}
