// SPDX-License-Identifier: Apache-2.0
use crate::Cid;
#[cfg(feature = "dag_cbor")]
use multi_codec::Codec;
use multi_trait::EncodeIntoBuffer;
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
                let mut v = Vec::new();
                // start with the Identity codec (0x00)
                Codec::Identity.encode_into_buffer(&mut v);
                // add in the binary serialized CID
                self.encode_into_buffer(&mut v);
                // annotate the bytes
                let bytes = multi_cbor::value::Value::Bytes(v);
                // wrap it as a tagged object with tag 42
                let tagged = multi_cbor::tags::Tagged::new(Some(42_u64), bytes);
                // serialize the tagged data
                tagged.serialize(serializer)
            }

            #[cfg(not(feature = "dag_cbor"))]
            {
                let mut v = Vec::new();
                self.encode_into_buffer(&mut v);
                serializer.serialize_bytes(v.as_slice())
            }
        }
    }
}
