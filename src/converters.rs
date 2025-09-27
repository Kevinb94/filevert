use anyhow::{Result, Context};          // bring Context for .with_context()
use std::fs::{File, create_dir_all};                      // File
use std::io::{BufWriter, Write};        // BufWriter, Write
use std::path::Path;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, clap::ValueEnum)]
pub enum Format {
    Json,
    Csv,
    Xml,
    Png,
    Jpeg,
}

pub fn convert(
    input_fmt: Format,
    output_fmt: Format,
    input_path: &Path,
    output_path: &Path,
) -> Result<()> {
    use Format::*;
    match (input_fmt, output_fmt) {
        (Csv, Json) => csv_to_json(input_path, output_path),
        (Json, Csv) => json_to_csv(input_path, output_path),
        (Csv, Xml)  => csv_to_xml(input_path, output_path),
        (Xml, Csv)  => xml_to_csv(input_path, output_path),
        (Json, Xml) => json_to_xml(input_path, output_path),
        (Xml, Json) => xml_to_json(input_path, output_path),
        (Png, Jpeg) => png_to_jpeg(input_path, output_path),
        (Jpeg, Png) => jpeg_to_png(input_path, output_path),
        (a, b) if a == b => anyhow::bail!("Input and output formats are the same: {a:?}"),
        (a, b) => anyhow::bail!("Unsupported conversion: {a:?} -> {b:?}"),
    }
}

fn ensure_parent_dirs(path: &Path) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.as_os_str().is_empty() {
            create_dir_all(parent)
                .with_context(|| format!("Failed to create output directory: {}", parent.display()))?;
        }
    }
    Ok(())
}

fn csv_to_json(input: &Path, output: &Path) -> Result<()> {
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .from_path(input)
        .with_context(|| format!("Failed to open CSV: {}", input.display()))?;

    // NEW: make sure folders exist
    ensure_parent_dirs(output)?;

    let out_file = File::create(output)
        .with_context(|| format!("Failed to create output: {}", output.display()))?;
    let mut w = BufWriter::new(out_file);

    w.write_all(b"[")?;
    let headers = rdr.headers().with_context(|| "Failed to read CSV headers")?.clone();

    let mut first = true;
    for rec in rdr.records() {
        let rec = rec.with_context(|| "Failed to read CSV record")?;
        if !first { w.write_all(b",")?; }
        first = false;

        let mut obj = serde_json::Map::with_capacity(headers.len());
        for (h, v) in headers.iter().zip(rec.iter()) {
            obj.insert(h.to_string(), serde_json::Value::String(v.to_string()));
        }
        serde_json::to_writer(&mut w, &serde_json::Value::Object(obj))
            .with_context(|| "Failed to serialize JSON object")?;
    }

    w.write_all(b"]")?;
    w.flush()?;
    Ok(())
}

fn json_to_csv(_in: &Path, _out: &Path) -> anyhow::Result<()> {
    println!("json_to_csv()");
    Ok(())
}
fn csv_to_xml(_in: &Path, _out: &Path) -> anyhow::Result<()> {
    println!("csv_to_xml()");
    Ok(())
}
fn xml_to_csv(_in: &Path, _out: &Path) -> anyhow::Result<()> {
    println!("xml_to_csv()");
    Ok(())
}
fn json_to_xml(_in: &Path, _out: &Path) -> anyhow::Result<()> {
    println!("json_to_xml()");
    Ok(())
}
fn xml_to_json(_in: &Path, _out: &Path) -> anyhow::Result<()> {
    println!("xml_to_json()");
    Ok(())
}
fn png_to_jpeg(_in: &Path, _out: &Path) -> anyhow::Result<()> {
    println!("png_to_jpeg()");
    Ok(())
}
fn jpeg_to_png(_in: &Path, _out: &Path) -> anyhow::Result<()> {
    println!("jpeg_to_png()");
    Ok(())
}
