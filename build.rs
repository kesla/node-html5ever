extern crate napi_build;

use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
  // first build it
  napi_build::setup();

  merge_dts();
}

fn merge_dts() {
  let path = Path::new("index.d.ts");
  let mut output = File::create(&path).unwrap();

  ["types/additional.d.ts", "types/generated.d.ts"]
    .iter()
    .for_each(|path| {
      let path = &Path::new(path);
      let mut input = File::open(path).unwrap();
      let mut content = String::new();
      content.push_str(format!("// {}\n", path.display()).as_str());
      input.read_to_string(&mut content).unwrap();
      content = content.trim().to_string();
      content.push('\n');

      output.write_all(content.as_bytes()).unwrap();
    });
}
