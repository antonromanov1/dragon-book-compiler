use crate::lexer::*;
use crate::parser::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

pub fn generate(bytes_used: u32, variables: HashMap::<String, TypeBase>,
                ast: *mut Node, file_name: &str) -> std::io::Result<()> {
    let v: Vec<&str> = file_name.split('.').collect();
    let mut output_name = String::from(v[0]);
    output_name.push_str(".s");
    let mut file = File::create(&output_name)?;

    file.write_all(b".global main\n")?;
    file.write_all(b".text\n")?;

    file.write_all(b"main:\n")?;

    file.write_all(b"    sub $")?;
    let s = format!("{}", bytes_used);
    file.write_all(&(s.clone()).into_bytes());
    file.write_all(b", %rsp\n");

    file.write_all(b"    add $");
    file.write_all(&s.into_bytes());
    file.write_all(b", %rsp\n");

    file.write_all(b"    ret\n")?;

    Ok(())
}
