use std::env;
use std::fs::File;
use std::io::Read;
use zip;

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
    println!("{}", buffer);

    Ok(())
}
