use std::{path::Path, process::{Command, Stdio}};

use crate::{checker::{check_synctatic_conformance, generate_merge_contract}, parser::{parse_implementation, parse_specification}};

// use assert_json_diff::assert_json_include;
mod checker;
mod parser;

#[warn(unused_imports)]
fn main() {
    let spec_path = std::env::args().nth(1).expect("Expected specification path");
    let impl_path = std::env::args().nth(2).expect("Expected implementation path");
    let out_path = std::env::args().nth(3).unwrap_or(".".to_owned());
    let solc_path = std::env::args().nth(4).unwrap_or("solc".to_owned());
    let solc_verify_path = std::env::args().nth(5).unwrap_or("solc-verify.py".to_owned());
    
    Command::new(&solc_path).arg(&spec_path).arg("--ast-compact-json").arg("--overwrite").arg("-o").arg(&out_path).output().expect("Couldn't generate spec ast");
    Command::new(&solc_path).arg(&impl_path).arg("--ast-compact-json").arg("--overwrite").arg("-o").arg(&out_path).output().expect("Couldn't generate impl ast");

    let spec_json_path = format!("{}_json.ast", Path::new(&spec_path).file_name().unwrap().to_str().unwrap());
    let impl_json_path = format!("{}_json.ast", Path::new(&impl_path).file_name().unwrap().to_str().unwrap());

    let spec= parse_specification(Path::new(&out_path).join(spec_json_path).as_path()).expect("Expected specification");
    let imp = parse_implementation(Path::new(&out_path).join(impl_json_path).as_path()).expect("Expected implementation");

    if let Err(s) = check_synctatic_conformance(&spec, &imp) {
        println!("Found an error: {}", s);
        return;
    }

    let res_merge = generate_merge_contract(&spec, &imp, Path::new(&impl_path), Path::new(&out_path));

    if let Err(s) = &res_merge {
        println!("Found an error: {}", s);
        return;
    }

    let merge_path = res_merge.unwrap();

    let comm = Command::new(&solc_verify_path).arg(&merge_path).arg("--solver").arg("z3").stdout(Stdio::piped()).output().expect("Couldn't generate spec ast");
    if comm.status.success() {
        println!("Semantic conformance checked")
    } else {
        println!("Unable to show semantic conformance")
    }

    println!("{}", String::from_utf8(comm.stdout).unwrap())

}
