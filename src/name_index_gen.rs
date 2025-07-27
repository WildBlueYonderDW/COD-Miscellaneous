use crate::error::Error;
use porter_utils::NameDatabase;
use std::path::Path;

// TODO: NameDatabase uses the HashMap type internally, so the order of saving results is different each time
/// Loads a csv file and turns it into a NameDatabase
pub fn generate_name_index<P: AsRef<Path>>(filename: P) -> Result<(), Error> {
    let mut name_index = NameDatabase::new();

    let path = filename.as_ref();

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_path(path)?;

    for result in rdr.records() {
        let record = result?;

        if let (Some(key_str), Some(value)) = (record.get(0), record.get(1)) {
            if let Ok(key) = u64::from_str_radix(key_str, 16) {
                name_index.insert(key, String::from(value));
            } else {
                eprintln!("Failed to parse key: {key_str}");
            }
        } else {
            eprintln!("Malformed CSV record");
        }
    }

    let output_path = path.with_extension("cdb");
    name_index.save(output_path)?;

    Ok(())
}

/// Converts a NameDatabase (.cdb) file back into a CSV file.
/// It reads the binary database, then writes out each entry as a
/// hexadecimal key and a string value, creating a new .csv file
/// with the same base name as the input file.
pub fn generate_name_csv<P: AsRef<Path>>(filename: P) -> Result<(), Error> {
    // Load the binary name database from the .cdb file.
    let mut name_index = NameDatabase::new();
    let path = filename.as_ref();
    name_index.load(path)?;

    // Determine the output path by replacing the extension with .csv.
    let output_path = path.with_extension("csv");

    // Create a CSV writer for the new file.
    let mut wtr = csv::WriterBuilder::new()
        .has_headers(false)
        .from_path(output_path)?;

    // Iterate over all the key-value pairs in the loaded database.
    // Assuming `name_index` can be iterated over directly.
    for (key, value) in name_index.iter() {
        // Format the u64 key as a lowercase hexadecimal string.
        let key_hex = format!("{key:x}");
        // Write the record to the CSV file.
        wtr.write_record(&[key_hex, value.clone()])?;
    }

    // Ensure all buffered data is written to the file.
    wtr.flush()?;

    Ok(())
}
