// SPDX-License-Identifier: Apache-2.0
use crate::Cid;
use multi_trait::EncodeInto;
use serde::ser::{self, SerializeStruct};

/// Serialize instance of [`crate::Cid`]
impl ser::Serialize for Cid {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ser::Serializer,
    {
        if serializer.is_human_readable() {
            let mut ss = serializer.serialize_struct("cid", 3)?;
            ss.serialize_field("version", &self.codec.code())?;
            ss.serialize_field("encoding", &self.target_codec)?;
            ss.serialize_field("hash", &self.hash)?;
            ss.end()
        } else {
            #[cfg(feature = "dag_cbor")]
            {
                // build the byte string for DAG-CBOR according to the spec
                // https://github.com/ipld/specs/blob/master/block-layer/codecs/dag-cbor.md#links
                // start with the Identity codec (0x00)
                let identity: Vec<u8> = multi_codec::Codec::Identity.into();
                // add in the binary serialized CID
                let mut v = identity;
                v.extend_from_slice(&self.encode_into());
                // annotate the bytes
                let bytes = multi_cbor::value::Value::Bytes(v);
                // wrap it as a tagged object with tag 42
                let tagged = multi_cbor::tags::Tagged::new(Some(42_u64), bytes);
                // serialize the tagged data
                tagged.serialize(serializer)
            }

            #[cfg(not(feature = "dag_cbor"))]
            {
                let v: Vec<u8> = self.encode_into();
                serializer.serialize_bytes(v.as_slice())
            }
        }
    }
}
