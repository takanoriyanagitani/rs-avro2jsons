use std::io;

use apache_avro::types::Value as AvroValue;
use serde_json::Value as JsonValue;

use crate::bind;

use crate::avro2json::a2j;

use crate::avro2values::stdin2values;
use crate::jsons2output::jsons2stdout_new;

pub fn conv<I>(a: I) -> impl Iterator<Item = Result<JsonValue, io::Error>>
where
    I: Iterator<Item = Result<AvroValue, io::Error>>,
{
    a.map(|r| r.and_then(a2j))
}

pub fn stdin2values2jsons() -> Result<impl Iterator<Item = Result<JsonValue, io::Error>>, io::Error>
{
    let a = stdin2values()?;
    Ok(conv(a))
}

pub fn stdin2stdout() -> Result<(), io::Error> {
    bind!(stdin2values2jsons, jsons2stdout_new)()
}
