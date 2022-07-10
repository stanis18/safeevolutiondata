
use crate::parser::{Implementation,VariableDeclaration};
use std::str::FromStr;
use std::env;
use std::fs::File;
use std::str;
use std::{path::Path, process::{Command, Stdio}};
use std::io::{self, prelude::*, Read};
use std::fs;
use ethabi::{Token};
use rocket::serde::{Serialize, Deserialize};

#[derive(Debug,Clone, Serialize,Deserialize)]
pub struct AssignedVariable {
    pub variable_declaration: VariableDeclaration, 
    pub variable_value: String, 
 }


pub fn get_value(assigned_variable: &AssignedVariable) -> Result<Token, String> {
    
    match assigned_variable.variable_declaration.typ.as_str() {
        "address" =>  { 
            let result = web3::types::H160::from_str(&assigned_variable.variable_value);
            if let Err(_) = &result {
                return Err( format!("Cannot convert value {} for type {}", assigned_variable.variable_value, assigned_variable.variable_declaration.typ));
            }
            return Ok(Token::Address(ethereum_types::H160::from(string_to_static_str(assigned_variable.variable_value.clone()))));
        },
        "bool" =>  {
            let result = assigned_variable.variable_value.parse::<bool>();
            if let Err(_) = &result {
                return Err( format!("Cannot convert value {} for type {}", assigned_variable.variable_value, assigned_variable.variable_declaration.typ));
            }
            return Ok(Token::Bool(result.unwrap()))
        },
        "string" => return Ok(Token::String(assigned_variable.variable_value.to_owned())),
        "bytes8" | "bytes32" | "bytes64" | "bytes1024" => return Ok(Token::FixedBytes(assigned_variable.variable_value.as_bytes().to_vec())),
        "uint256" => {
            let result = assigned_variable.variable_value.parse::<u64>();
            if let Err(_) = &result {
                return Err( format!("Cannot convert value {} for type {}", assigned_variable.variable_value, assigned_variable.variable_declaration.typ));
            }
            return Ok(Token::Uint(ethereum_types::U256::from(result.unwrap())));
        },
        _ => Err("There is no match for this value".to_owned()),
    }
}

// println!("file_name -> {:?}", file_name);
// println!("text -> {:?}", text);

// let file = File::create(Path::new("contracts/input").join(&file_name).as_path());

// if let Err(error) = &file {
//     println!("{:?}", "error write file");
//     println!("{:?}", error);
// }

// let result = file.unwrap().write_all(&text.as_bytes()).unwrap();
// println!("{:?}", result);

pub fn write_file(text: &String, file_name: &String) {

    let mut file = match File::create(Path::new("contracts/input").join(&file_name).as_path()) {
        Err(why) => panic!("Couldn't create the file {}", why),
        Ok(file) => file,
    };

    match file.write_all(text.as_bytes()) {
        Err(why) => panic!("Couldn't create the file {} ", why),
        Ok(_) => println!("Successfully created the file"),
    }
}

pub fn get_number_argument_constructor(imp: &Implementation) ->  Result <Vec<VariableDeclaration>, ()>{
    let mut contructor_variables: Vec<VariableDeclaration> = Vec::new();
    for i in 0..imp.functions.len() {
        if &imp.functions[i].signature.kind == "constructor" {
            contructor_variables = imp.functions[i].signature.ins.clone();
        }
    }
    Ok(contructor_variables)
}


pub fn delete_dir_contents() {
    let contents = fs::read_dir("contracts/input");
    
    if let Ok(dir) = contents {
        for entry in dir {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_dir() {
                    fs::remove_dir_all(path).expect("Failed to remove a dir");
                } else {
                    fs::remove_file(path).expect("Failed to remove a file");
                }
            };
        }
    };
}

pub fn copy_dir_contents(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_contents(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}


pub fn config_contructor_parameters(assigned_variables: &Vec<AssignedVariable>)  -> Result <Vec<Token>, String> {

    let mut list_token: Vec<Token> = Vec::new();
    for value in assigned_variables {
        list_token.push(get_value(&value)?);
    }
    Ok(list_token)
}

pub fn get_compiled_files(path_contract:&Path)  -> Result <String, String> {
    let mut file_contract = File::open(&path_contract).unwrap();
    let mut data_contract = String::new();
    file_contract.read_to_string(&mut data_contract).unwrap();
    Ok(data_contract)
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}


pub fn generate_ast_contract(file_name: &str) -> Result <(), String> { 
    
    // if cfg!(target_os = "windows") {
    //     let path = env::current_dir().unwrap();
    //     let command = format!("docker run --rm -v {}/contracts/input:/sources ethereum/solc:0.5.17 -o sources --ast-compact-json  /sources/{} --overwrite",
    //     path.to_str().unwrap(), &file_name);
    
    //     let com = Command::new("cmd").args(&["/C", &command]).stdin(Stdio::piped())
    //     .stdout(Stdio::piped()).spawn().expect("echo command failed to start");
    
    //     let mut answer = String::new();
    //     match com.stdout.unwrap().read_to_string(&mut answer) {
    //         Err(why) => panic!("Couldn't generate ast tree: {}", why),
    //         Ok(_) => print!("Tree generated with sucess:\n{}", answer),
    //     }
    
    // } else {

        let command = format!("/home/solc-static-linux-0.5.17 -o /home/back/contracts/input --ast-compact-json  /home/back/contracts/input/{} --overwrite", &file_name);
        println!("Command is {}", command);
        let com = Command::new("sh").args(["-c", &command]).stdin(Stdio::piped())
        .stdout(Stdio::piped()).spawn().expect("echo command failed to start");

        let mut answer = String::new();
        match com.stdout.unwrap().read_to_string(&mut answer) {
            Err(why) => panic!("Couldn't generate ast tree: {}", why),
            Ok(_) => print!("Tree generated with sucess:\n{}", answer),
        }
    // }
   
    Ok(())
}

pub fn verify_contract(merged_contract_file: String) -> Result <String, String> {
  
    // if cfg!(target_os = "windows") {
    //     let path = env::current_dir().unwrap();
    //     let command = format!("docker run --rm -v {}/contracts/input:/contracts solc-verify:0.7 /contracts/{}",
    //     path.to_str().unwrap(), merged_contract_file);

    //     let com = Command::new("cmd").args(&["/C", &command]).stdin(Stdio::piped())
    //     .stdout(Stdio::piped()).spawn().expect("echo command failed to start");

    //     let mut answer = String::new();
    //     match com.stdout.unwrap().read_to_string(&mut answer) {
    //         Err(why) => return Err(format!("Couldn't verify the contract {} :", why)),
    //         Ok(_) => Ok(answer)
    //     }

    // } else {
        let path = env::current_dir().unwrap();
        let command = format!("solc-verify.py --parallel 1 --solver z3 /home/back/contracts/input/{}", merged_contract_file);
        println!("Command is {}", command);
        let com = Command::new("sh").args(["-c", &command]).stdin(Stdio::piped())
        .stdout(Stdio::piped()).spawn().expect("echo command failed to start");

        let mut answer = String::new();
        match com.stdout.unwrap().read_to_string(&mut answer) {
            Err(why) => return Err(format!("Couldn't verify the contract {} :", why)),
            Ok(_) => Ok(answer)
        }
    // }
}


pub fn generate_compiled_contract(file_name: &str) -> Result <(), String> {

    // if cfg!(target_os = "windows") {
    //     let path = env::current_dir().unwrap();
    //     let command = format!("docker run --rm -v {}/contracts/input:/sources ethereum/solc:0.5.17 -o sources --bin --abi  /sources/{} --overwrite",
    //     path.to_str().unwrap(), &file_name);
        
    //     let com = Command::new("cmd").args(&["/C", &command]).stdin(Stdio::piped())
    //     .stdout(Stdio::piped()).spawn().expect("echo command failed to start");

    //     let mut answer = String::new();
    //     match com.stdout.unwrap().read_to_string(&mut answer) {
    //         Err(why) => panic!("Couldn't compile the contract {} : {}", why, file_name),
    //         Ok(_) => print!("Contract compiled with sucess:\n{}", answer),
    //     }

    // } else {
       
        let command = format!("/home/solc-static-linux-0.5.17 -o /home/back/contracts/input --bin --abi  /home/back/contracts/input/{} --overwrite", &file_name);
        println!("Command is {}", command);
        let com = Command::new("sh").args(["-c", &command]).stdin(Stdio::piped())
        .stdout(Stdio::piped()).spawn().expect("echo command failed to start");
    
        let mut answer = String::new();
        match com.stdout.unwrap().read_to_string(&mut answer) {
            Err(why) => panic!("Couldn't compile the contract {} : {}", why, file_name),
            Ok(_) => print!("Contract compiled with sucess:\n{}", answer),
        } 
    // }
   Ok(())
}


pub fn parse_merge_files(list_verification:Vec<String>) -> Result<(), String> {
    for file_result in list_verification {
        if file_result.contains("Errors were found by the verifier.") {
            return Err("The contract could not be deployed. Errors were found by the verifier.".to_owned());
        }
    }
    Ok(())
}

pub fn search_merge_files_dir(path_contract:&Path)  ->Result<Vec<String>, String>{
    let mut list_verification = Vec::new();
    for entry in fs::read_dir(path_contract).unwrap() {
        let entry = entry.unwrap();
        let file_name = entry.file_name().into_string().unwrap();
        if file_name.starts_with("merged_contract_") {
           let result = verify_contract(file_name);
           list_verification.push(result.unwrap()); 
        }
    }
    Ok(list_verification)
}