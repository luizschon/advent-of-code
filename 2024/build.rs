use std::{env, fs, path::Path};

use indoc::indoc;

fn main() {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("exported_days.rs");

    let mut mod_code = indoc! { "
        // Auto-generated module exports
        // DO NOT MODIFY
    " }
    .to_owned();

    let mut day_numbers = vec![];

    for entry in fs::read_dir("src/days").unwrap() {
        let entry = entry.unwrap();
        let name = entry.file_name().into_string().unwrap();

        if name.len() != 9 {
            continue;
        }

        if let Some(day_num) = name
            .strip_prefix("day_")
            .and_then(|s| s.strip_suffix(".rs"))
        {
            day_numbers.push(day_num.to_owned());
            mod_code.push_str(&format!(
                indoc! {r#"
                    pub mod day_{0} {{
                        include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/days/day_{0}.rs"));
                    }}

                    "#
                },
                day_num
            ));
        }
    }

    //
    mod_code.push_str("// Generate static day solution map\n");
    mod_code.push_str(&format!("register_days![{}];\n", day_numbers.join(", ")));

    fs::write(&dest_path, mod_code).unwrap();
    println!("cargo:rerun-if-changed=src/days");
}
