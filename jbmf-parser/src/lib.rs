pub use java_rs_base;
pub use java_rs_derive;
pub use java_rs_pacific;

use java_rs_pacific::JavaClass;
use std::fs::File;
use std::io::BufReader;

pub fn parse_class_file(path: &str) -> anyhow::Result<JavaClass> {
    let mut reader = BufReader::new(File::open(path).unwrap());
    Ok(JavaClass::read(&mut reader)?)
}
