use std::io;

use apache_avro::types::Value as AvroValue;
use serde_json::Value as JsonValue;

pub fn a2j(a: AvroValue) -> Result<JsonValue, io::Error> {
    a.try_into().map_err(io::Error::other)
}
