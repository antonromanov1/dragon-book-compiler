use crate::lexer::*;
use crate::parser::*;
use std::collections::HashMap;
use std::io::prelude::*;
use std::fs::File;

pub fn generate(bytes_used: usize, variables: HashMap::<String, TypeBase>,
                _ast: Option<Box<Node>>, file_name: &str) -> std::io::Result<()> {
    let v: Vec<&str> = file_name.split('.').collect();
    let mut output_name = String::from(v[0]);
    output_name.push_str(".s");
    let mut file = File::create(&output_name)?;

    file.write_all(b".global main\n")?;
    file.write_all(b".text\n")?;

    file.write_all(b"main:\n")?;
    file.write_all(b"    mov %rsp, %rbp\n")?;

    // Subtract total size of local variables
    file.write_all(b"    sub $")?;
    let s = format!("{}", bytes_used);
    file.write_all(&(s.clone()).into_bytes())?;
    file.write_all(b", %rsp\n")?;

    // Implicit initializing
    let mut distance: usize = 0;
    for (_key, val) in variables.iter() {
        distance = distance + val.get_width();
        if val.get_width() == 4 {
            file.write_all(&format!("    movl $0, -{}(%rbp)\n", distance).into_bytes())?;
        }
        else if val.get_width() == 8 {
            file.write_all(&format!("    movq $0, -{}(%rbp)\n", distance).into_bytes())?;
        }
    }

    // Add total size of local variables
    file.write_all(b"    add $")?;
    file.write_all(&s.into_bytes())?;
    file.write_all(b", %rsp\n")?;

    file.write_all(b"    ret\n")?;

    Ok(())
}
