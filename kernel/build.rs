use std::fs::{File, read_dir};
use std::io::Write;

static TARGET_PATH: &str = "./target/riscv64gc-unknown-none-elf/release/";

fn main() {
    println!("cargo:rerun-if-changed=../user/src");
    println!("cargo:rerun-if-changed={}", TARGET_PATH);
    insert_app_data();
}

fn insert_app_data() {
    let mut file = File::create("src/link_app.S").unwrap();
    let mut apps: Vec<_> = read_dir("../user/src/bin")
        .unwrap()
        .into_iter()
        .map(|entry| {
            let mut name_with_ext = entry.unwrap().file_name().into_string().unwrap();
            name_with_ext.drain(name_with_ext.find('.').unwrap()..name_with_ext.len());
            name_with_ext
        })
        .collect();
    apps.sort();

    writeln!(
        file,
        r#"    .align 3
    .section .data
    .global app_instruction_section_ptr
app_instruction_section_ptr:
    .quad {}"#,
        apps.len()
    )
    .unwrap();

    for i in 0..apps.len() {
        writeln!(file, r#"    .quad app_{}_start"#, i).unwrap();
    }
    writeln!(
        file,
        r#"    .quad app_{}_end
        "#,
        apps.len() - 1
    )
    .unwrap();

    for (idx, app) in apps.iter().enumerate() {
        println!("app_{}: {}", idx, app);
        writeln!(
            file,
            r#"    .section .data
    .global app_{0}_start
    .global app_{0}_end
app_{0}_start:
    .incbin "{2}{1}.bin"
app_{0}_end:
"#,
            idx, app, TARGET_PATH
        )
        .unwrap();
    }
}
