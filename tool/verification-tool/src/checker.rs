use std::{convert::TryInto, fs::{File}, io::{Read, Write}, path::Path};

use crate::parser::{Implementation, Specification, FunctionImplementation, FunctionSpecification,VariableDeclaration,FunctionSignature,FunctionImplementationContractName};


pub fn check_synctatic_conformance(spec: &Specification, imp: &Implementation, evolution: bool) -> Result<(), String> {
   
    for i in 0..spec.variables.len() {
        let compatible_variable = search_variables_declaration(&imp, &spec.variables[i]).unwrap();
        if !compatible_variable {
            return Err("Incompatible variables between the specification and implementation contracts".to_owned());
        }
    }   

    for i in 0..spec.functions.len() {
        
        if evolution && spec.functions[i].signature.kind == "constructor" { continue; }
        let compatible_function = search_function_implementation(&imp, &spec.functions[i].signature).unwrap();
            
        if !compatible_function {
            return Err("Incompatible functions between the specification and implementation contracts".to_owned());
        }
    }

    let mut impl_func :Vec<FunctionImplementation> = Vec::new();
    get_function_implementation(&imp, &mut impl_func);

    for i in 0..impl_func.len() {
  
        if evolution && impl_func[i].signature.kind == "constructor" {
            return Err("The contract's implementation cannot have constructor".to_owned());
        }
        let func_spec = get_func_spec(&impl_func[i], &spec.functions)?;

        if func_spec.signature != impl_func[i].signature {
            return Err("Incompatible functions between the specification and implementation contracts".to_owned());
        }
    }
    Ok(())
}

pub fn get_function_implementation(imp: &Implementation, func: &mut Vec<FunctionImplementation>) {
    func.append( &mut imp.functions.clone());
    for i in 0..imp.contracts_inherited.len() {
        get_function_implementation(&imp.contracts_inherited[i], func);
    }
}

pub fn get_variables_implementation(imp: &Implementation, variables: &mut Vec<VariableDeclaration>) {
    variables.append(&mut imp.variables.clone());
    for i in 0..imp.contracts_inherited.len() {
        get_variables_implementation(&imp.contracts_inherited[i], variables);
    }
}


fn search_function_implementation(imp: &Implementation, funcion_sig: &FunctionSignature) -> Result<bool,String> {
    for i in 0..imp.functions.len() { 
        if &imp.functions[i].signature == funcion_sig {
           return Ok(true);
        }
    }
    for i in 0..imp.contracts_inherited.len() {
        return search_function_implementation(&imp.contracts_inherited[i], funcion_sig);
    }
    Ok(false)
}

fn search_variables_declaration(imp: &Implementation, variable_decla: &VariableDeclaration) -> Result<bool,String> {
    for i in 0..imp.variables.len() { 
        if &imp.variables[i] == variable_decla {
           return Ok(true);
        }
    }
    for i in 0..imp.contracts_inherited.len() {
        return search_variables_declaration(&imp.contracts_inherited[i], variable_decla);
    }
    Ok(false)
}

fn get_function_implementation_contract(imp: &Implementation, func: &mut Vec<FunctionImplementationContractName>) {
    for i in 0..imp.functions.len() {
        let imp_contr = FunctionImplementationContractName {
            contract_name: imp.contract_name.clone(),
            function: imp.functions[i].clone()
        };
        func.push(imp_contr);
    }
    for i in 0..imp.contracts_inherited.len() {
        get_function_implementation_contract(&imp.contracts_inherited[i], func);
    }
}

pub fn generate_merge_contract(spec: &Specification, imp: &Implementation, invariant: bool) -> Result <(), String>{

    let impl_path = Path::new("contracts/input").join(format!("{}.sol", imp.contract_name));
    let out_path = Path::new("contracts/input");

    let merge_file_name = format!("{}_{}.sol", "merged_contract", imp.contract_name);
    let merge_file_path = out_path.join(&merge_file_name).to_str().unwrap().to_string();
    let mut merge_file = File::create(merge_file_path.clone()).map_err( |_| {"Error creating merge".to_owned()}).unwrap();
    
    let mut impl_file = File::open(impl_path).map_err( |_| {"Error opening impl".to_owned()}).unwrap();
    let mut last_offset = 0;
    let mut buf = vec![0;imp.contract_definition.offset.try_into().unwrap()];

    if invariant {
        impl_file.read_exact(buf.as_mut_slice()).unwrap();
        merge_file.write_all(buf.as_slice()).unwrap();
        let x = format!("/** \n * {} \n */ \n", &spec.invariant);
        merge_file.write_all(x.as_bytes()).unwrap();
        last_offset = imp.contract_definition.offset;
    }


    for i in 0..imp.functions.len() {
        let func_impl = &imp.functions[i]; // get implementation
        let func_spec = get_func_spec(&func_impl, &spec.functions).unwrap();
        let buf_len = (func_impl.src.offset - last_offset).try_into().unwrap(); // buff size
        let mut buf = vec![0;buf_len]; // creating buff array
        last_offset = func_impl.src.offset;
        impl_file.read_exact(buf.as_mut_slice()).unwrap(); // reading from implementation file
        merge_file.write_all(buf.as_slice()).unwrap(); // writing in merge file
        let spec = format!("/** \n * {} \n */ \n", func_spec.spec); // formating specification
        merge_file.write_all(spec.as_bytes()).unwrap(); // writing spec
    } 
    let mut rest = Vec::new();
    impl_file.read_to_end(&mut rest).unwrap();
    merge_file.write_all(rest.as_slice()).unwrap();


    for i in 0..imp.contracts_inherited.len() {
        generate_merge_contract(&spec, &imp.contracts_inherited[i], false);
    }
    Ok(())
}





// pub fn generate_merge_contract(spec: &Specification, imp: &Implementation, impl_path: &Path, out_path: &Path, evolution: bool) -> Result<String, String>{
//     let merge_file_name = format!("{}", "merged_contract.sol");
//     let merge_file_path = out_path.join(&merge_file_name).to_str().unwrap().to_string();
//     let mut merge_file = File::create(merge_file_path.clone()).map_err( |_| {"Error creating merge".to_owned()})?;
//     let mut impl_file = File::open(impl_path).map_err( |_| {"Error opening impl".to_owned()})?;
//     let mut last_offset = 0;
    
//     let mut buf = vec![0;imp.contract_definition.offset.try_into().unwrap()];
//     impl_file.read_exact(buf.as_mut_slice()).unwrap();
//     merge_file.write_all(buf.as_slice()).unwrap();
//     let x = format!("/** \n * {} \n */ \n", &spec.invariant);
//     merge_file.write_all(x.as_bytes()).unwrap();
//     last_offset = imp.contract_definition.offset;

//     for i in 0..imp.functions.len() {
//         let func_impl = &imp.functions[i]; // get implementation
//         let func_spec = get_func_spec(&func_impl, &spec.functions)?;
//         let buf_len = (func_impl.src.offset - last_offset).try_into().unwrap(); // buff size
//         let mut buf = vec![0;buf_len]; // creating buff array
//         last_offset = func_impl.src.offset;
//         impl_file.read_exact(buf.as_mut_slice()).unwrap(); // reading from implementation file
//         merge_file.write_all(buf.as_slice()).unwrap(); // writing in merge file
//         let spec = format!("/** \n * {} \n */ \n", func_spec.spec); // formating specification
//         merge_file.write_all(spec.as_bytes()).unwrap(); // writing spec
//     } 
//     let mut rest = Vec::new();
//     impl_file.read_to_end(&mut rest).unwrap();
//     merge_file.write_all(rest.as_slice()).unwrap();

//     Ok(merge_file_path)
// }

pub fn get_func_spec(func_imp: &FunctionImplementation, spec_functions : &Vec<FunctionSpecification> ) ->  Result<FunctionSpecification, String >{
    for func in spec_functions {
       if func.signature.name == func_imp.signature.name {
        return Ok(func.clone());
       }
    }
    return Err("Missing Function".to_owned());
}
