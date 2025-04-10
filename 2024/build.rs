use std::{fs, path::Path};

use indoc::indoc;

fn main() {
    let dest_path = Path::new("src/days/").join("_generated_.rs");

    let mut mod_code = indoc! { "
        // DO NOT MODIFY!
        //
        // Auto-generated module exports
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
                    #[path = "day_{0}.rs"]
                    pub mod day_{0};
                "# },
                day_num
            ));
        }
    }

    mod_code.push_str("\n// Generate static day solution map\n");
    mod_code.push_str(&format!("register_days![{}];\n", day_numbers.join(", ")));

    fs::write(&dest_path, mod_code).unwrap();
    println!("cargo:rerun-if-changed=src/days");
}
