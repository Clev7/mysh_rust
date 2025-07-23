use std::fs::read_to_string;
use std::fs::canonicalize;
use std::io::Result;
use std::path::PathBuf;

fn main() -> Result<()> {
    let f1 = PathBuf::from("../diffs/file1.txt");
    let f2 = PathBuf::from("../diffs/file2.txt");

    println!("{:?}", canonicalize(&f1));
    println!("{:?}", canonicalize(&f2));

    let f1_contents = read_to_string(f1)?;
    let f2_contents = read_to_string(f2)?;

    println!("f1_contents: {f1_contents}");
    println!("f2_contents: {f2_contents}");

    Ok(())
}
