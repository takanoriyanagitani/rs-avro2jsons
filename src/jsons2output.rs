use std::io;
use std::io::BufWriter;
use std::io::Write;

use serde_json::Value;

pub fn jsons2writer<I, W>(jsons: I, mut writer: W) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Value, io::Error>>,
    W: Write,
{
    for rj in jsons {
        let j: Value = rj?;
        serde_json::to_writer(&mut writer, &j)?;
        writeln!(writer)?;
    }
    Ok(())
}

pub fn jsons2stdout<I>(jsons: I) -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Value, io::Error>>,
{
    let o = io::stdout();
    let mut ol = o.lock();
    {
        let mut bw = BufWriter::new(&mut ol);
        jsons2writer(jsons, &mut bw)?;
        bw.flush()?;
    }
    ol.flush()?;
    Ok(())
}

pub fn jsons2stdout_new<I>(jsons: I) -> impl FnOnce() -> Result<(), io::Error>
where
    I: Iterator<Item = Result<Value, io::Error>>,
{
    move || jsons2stdout(jsons)
}
