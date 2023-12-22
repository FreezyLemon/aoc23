use std::{env, error::Error, fs::{self, File}, io::Write, path::{Path, PathBuf}};

const INPUT_DIR: &str = "inputs/";

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = env::var("OUT_DIR")?;
    let dst_path = Path::new(&out_dir).join("inputs.rs");
    let mut inputs_file = File::create(dst_path)?;

    writeln!(&mut inputs_file, r##"["##,)?;

    let mut d = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    d.push(INPUT_DIR);
    for f in fs::read_dir(d)? {
        let f = f?;

        if !f.file_type()?.is_file() {
            continue;
        }

        writeln!(
            &mut inputs_file,
            r##"("{short_name}", include_str!(r#"{name}"#)),"##,
            short_name = &f.file_name().into_string().unwrap(),
            name = f.path().display(),
        )?;
    }

    writeln!(&mut inputs_file, r##"]"##,)?;

    Ok(())
}

