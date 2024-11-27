use std::io;
use std::io::BufReader;
use std::io::Read;

use apache_avro::types::Value;
use apache_avro::Reader;

pub fn reader2values<R>(
    reader: R,
) -> Result<impl Iterator<Item = Result<Value, io::Error>>, io::Error>
where
    R: Read,
{
    let ar: Reader<_> = Reader::new(reader).map_err(io::Error::other)?;
    Ok(ar.map(|rslt| rslt.map_err(io::Error::other)))
}

pub fn stdin2values() -> Result<impl Iterator<Item = Result<Value, io::Error>>, io::Error> {
    let i = io::stdin();
    let il = i.lock();
    let br = BufReader::new(il);
    reader2values(br)
}
