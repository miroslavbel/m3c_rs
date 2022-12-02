//! Module for serialization and deserialization into/from different formats.
//!
//! It's only available to serialize from [Internal format](crate::formats::internal)
//! and deserialize into [Internal format](crate::formats::internal).
//!
//! Available submodules:
//! * [native] - serializers and deserializers for native formats.
//! * [custom] - serializers and deserializers for custom formats.

pub mod custom;
pub mod native;
