use roxmltree;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::File;
use std::io::Read;
use zip;

#[derive(Debug, Serialize, Deserialize)]
struct Record<'a> {
    value: &'a str,
    end_date: &'a str,
    start_date: &'a str,
    creation_date: &'a str,
    unit: &'a str,
    device: &'a str,
    source_version: &'a str,
    source_name: &'a str,
    r#type: &'a str,
}

impl<'a> Record<'a> {
    fn from_xml_node(node: &roxmltree::Node<'a, '_>) -> Record<'a> {
        Record {
            value: node.attribute("value").unwrap_or_default(),
            end_date: node.attribute("endDate").unwrap_or_default(),
            start_date: node.attribute("startDate").unwrap_or_default(),
            creation_date: node.attribute("endDate").unwrap_or_default(),
            unit: node.attribute("unit").unwrap_or_default(),
            device: node.attribute("device").unwrap_or_default(),
            source_version: node.attribute("sourceVersion").unwrap_or_default(),
            source_name: node.attribute("sourceName").unwrap_or_default(),
            // escape the type keyword so we can use it
            r#type: node.attribute("type").unwrap_or_default(),
        }
    }
}

fn main() -> std::io::Result<()> {
    // Grab the command line options from argv
    let args: Vec<String> = env::args().collect();

    // open the file given as the first positional argument to the script
    // the file should be a zip archive
    let f = File::open(&args[1])?;
    let mut archive = zip::ZipArchive::new(f)?;

    // get the file that contains the xml data we want to parse
    let mut xml_file = archive.by_name("apple_health_export/export.xml")?;
    let mut buffer = String::new();
    xml_file.read_to_string(&mut buffer)?;

    // parse the string buffer into an XML tree
    let doc = match roxmltree::Document::parse(&buffer) {
        Ok(value) => value,
        Err(_) => panic!("Couldn't parse XML"),
    };

    for node in doc.descendants().filter(|node| node.has_tag_name("Record")) {
        let record = Record::from_xml_node(&node);

        println!("{:?}", record.value);
    }

    Ok(())
}
