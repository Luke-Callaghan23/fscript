use std::fs;


pub struct PathAndContents {
    pub path: String,
    pub contents: String
}

/// Function to process a list of string paths to files into a vector of
///         compilation target structs to be compiles
/// # Arguments
/// * `files` - vector of all files to process
/// 
/// # Returns 
/// * Vector of Compilation targets contained in the directory
pub fn process_files <'a> (files: Vec<String>) -> Vec<PathAndContents> {
    // let mut targets: Vec<PathAndContents> = Vec::new();
    // for file in files { 
        
    // }

    files.into_iter().map(| file | {
        let path = file;
        // Getting the metadata of the target path, to check if the
        match fs::metadata(&path) {
            Ok (metadata) => {
                if metadata.is_file() {
                    if path.ends_with(".fjs") {
                        // If the entry is a file that ends with .fjs, process the file
                        //      into a Some CompilationTarget, and add it to the targets
                        //      vector
                        if let Some ( target ) = process_file(path) {
                            Some( vec![target] )
                        }
                        else { None }                              
                    }
                    else { None }
                }
                else if metadata.is_dir() {
                    // If the entry is a directory call process_directory, and return
                    //      the retrieved vector
                    Some( process_dir(path) )
                }
                else { None }
            },
            // Otherwise, report the error
            Err(err) => {
                println!("Error opening file {} !", path);
                println!("Error: {}", err);
                None
            }
        }
    })
    .flatten()
    .flatten()
    .collect()
}

/// Function to process all compilable files in a directory into Compilation target 
///         structs
/// # Arguments
/// * `path` - a string slice that holds the path to the targeted directoy
/// 
/// # Returns 
/// * Vector of Compilation targets contained in the directory
fn process_dir <'a>(path: String) -> Vec<PathAndContents> {
    println!("{}", path);
    process_files (
        // Call process file with a mapping from directory items, to 
        //      a String path of that entry
        fs::read_dir(&path).unwrap().map(| entry | {
            match entry {
                Ok(entry) => {
                    // If the entry was read successfully, return Some,
                    //      String interpretation of its path
                    Some (String::from(entry.path().to_str().unwrap()))
                }
                // Otherwise, repor the error
                Err(err) => {
                    println!("Error opening file {} !", path);
                    println!("Error: {}", err);
                    None                
                }
            }
        })   // Option <String> iterable of String paths
        .flatten()  // <String> iterable of paths
        .collect() // Vector of String paths
    )
}


/// Function to read a single .fjs file into a string, and create a CompilationTarget
///      struct for that file
/// # Arguments
/// * `path` - a string slice that holds the path to the targeted file
/// 
/// # Returns 
/// * Optional enum of the CompilationTarget based of the path file, return None if
///         there was an error opening the file
#[allow(unused_parens)]
fn process_file <'a> (path: String) -> Option<PathAndContents> {   
    match fs::read_to_string(&path) {
        // If the file has the .fjs extension, create a CompilationTarget
        //      struct for that file and return it
        Ok( contents) => {
            println!("Got file {}!", path);
            Some (
                PathAndContents {
                    path: path,
                    contents: contents
                }
            )
        },
        // Otherwise, report the error
        Err( err ) => {
            println!("Error opening file {} !", path);
            println!("Error: {}", err);
            None
        }
    }
}
