use name_index_gen::{generate_name_csv, generate_name_index};
mod error;
mod name_index_gen;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for arg in args.iter().skip(1) {
        let path = std::path::Path::new(arg);

        let result = match path.extension() {
            Some(ext) => {
                if ext == "cdb" {
                    generate_name_csv(path)
                } else if ext == "csv" {
                    generate_name_index(path)
                } else {
                    Err(std::io::Error::other(format!(
                        "Unsupported file extension: {}",
                        ext.to_string_lossy()
                    ))
                    .into())
                }
            }
            _ => Err(std::io::Error::other("File has no extension").into()),
        };
        if let Err(e) = result {
            eprintln!("Error: {e}");
        }
    }
}
