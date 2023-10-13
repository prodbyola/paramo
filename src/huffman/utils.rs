use std::{io::{Error as ioError, Result as ioResult, ErrorKind}, fs::{remove_file, self, create_dir_all}, path::{Path, PathBuf}};

use clap::{Parser, command};
use serde::{Deserialize, Serialize};

use super::frequency::Frequencies;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct AppOptions {
    #[arg(short, long)]
    pub input: String,

    #[arg(short, long)]
    pub output_folder: Option<String>,
    
    #[arg(short, long)]
    pub filename: Option<String>,
    
    #[arg(short, long, default_value_t = false)]
    pub decode: bool,
}

pub trait BitString {
    fn len(&self) -> usize;

    /// Converts the input string to to `Vec`.
    fn to_vec(&self) -> Result<Vec<u8>, ioError>;

    /// Checks if the string can converted to `Vec`. 
    fn is_complete(&self) -> bool {
        self.len() % 8 == 0
    }

    /// Length of bytes to be derived from the string. 
    fn byte_len(&self) -> usize {
        self.len()/8
    }
}

impl BitString for String {
    fn len(&self) -> usize {
        self.len()
    }

    fn to_vec(&self) -> Result<Vec<u8>, ioError> {
        if !self.is_complete() {
            let err = ioError::new(std::io::ErrorKind::InvalidInput, "String is not a complete bit string.");
            return Err(err);
        }

        let mut current_byte = 0u8; // 8-bits zeros
        let mut remaining_bytes: u8 = 8;
        let mut output = Vec::with_capacity(self.byte_len());

        for bit in self.chars() {
            if bit != '1' && bit != '0' {
                let err = ioError::new(std::io::ErrorKind::InvalidData, "Not a valid bit string");
                return Err(err);
            }

            let bit_value: u8 = if bit == '1' { 1 } else { 0 };
            current_byte = (current_byte << 1) | bit_value;

            remaining_bytes -= 1;

            // If current_byte is full, push it to `output` and reinitialize our monitors
            if remaining_bytes == 0 {
                output.push(current_byte);
                current_byte = 0;
                remaining_bytes = 8;
            }
        }

        Ok(output)
    }
}

#[derive(Serialize, Deserialize)]
pub struct Header {
    pub frequencies: Frequencies,
    pub extension: Option<String>,
    pub padding: Option<u8>
}

/// Generates a repeated character string
pub fn str_generator(c: char, len: usize) -> String {
    std::iter::repeat(c).take(len).collect()
}

pub fn rm_file<P: AsRef<Path>>(path: &P) -> ioResult<()> {
    if let Ok(_) = fs::metadata(path) {
        remove_file(path)?;
    }

    Ok(())
}

#[derive(Default)]
pub struct ParamoIOFiles {
    pub input: PathBuf,
    pub output: PathBuf,
    pub input_ext: Option<String>,
    pub output_ext: Option<String>,
}
impl ParamoIOFiles {
    pub fn parse(opts: &AppOptions) -> ioResult<ParamoIOFiles> {
        let mut io_files = ParamoIOFiles::default();
        let input_arg = &opts.input;

        // validate the input path argument
        let input = PathBuf::from(input_arg);
        if !input.exists() {
            let err = ioError::new(ErrorKind::NotFound, "Specified input file does not exist!");
            return Err(err);
        }

        if !input.is_file() {
            let err = ioError::new(ErrorKind::InvalidInput, "Input must be a valid file path");
            return Err(err);
        }
        
        // Extract input file extension. 
        // This will be stored in encoded file header
        // and can be retrieved during decoding
        if !opts.decode {
            io_files.input_ext = input
                .extension()
                .map(|s| s.to_string_lossy().to_string());
        }

        // Build output path
        let out_ext = ".paramo";
        let mut output = input.clone();
        output.pop();

        let default_output_name = if opts.decode { "decoded" } else { "encoded" }.to_string();

        let mut output_basename = input
            .file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or(default_output_name.clone());

        if let Some(f) = &opts.filename {
            output_basename = f.to_string();

            if opts.decode && output_basename.contains(".") {
                let sob = output_basename.split(".");
                let ext = sob.last();
                io_files.output_ext = ext.map(|s| s.to_string());
            }
        }

        if !&opts.decode {
            output_basename.push_str(out_ext);
        }
        
        if let Some(out_arg) = &opts.output_folder {
            output = PathBuf::from(out_arg);

            // If output folder does not exist, create it.
            if !output.exists() {
                create_dir_all(&output)?
            }
        }

        output.push(output_basename);

        // update io_files with built path
        io_files.output = output;
        io_files.input = input;

        Ok(io_files)
    }

    
}