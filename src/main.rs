// compiler driver file
use std::env;
use std::process::Command;
use std::process::ExitCode;
use std::fs;

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    let options: Vec<String> = args.clone()
        .into_iter()
        .filter(|arg| arg.starts_with("--"))
        .collect();
    // dbg!(&options);

    let input_file_path = &args[1];
    // dbg!(input_file_path);
    let parts: Vec<&str> = input_file_path.split(".").collect();
    // dbg!(&parts);

    let file_name = &parts[0];

    preprocess(input_file_path, file_name);
    compile(file_name, options);
    assemble_and_link(file_name);

    ExitCode::SUCCESS

}

fn preprocess(input_file_path: &String, file_name: &&str) {
    let output_file = format!("{}.i", file_name);
    // dbg!(&output_file);

    Command::new("gcc")
        .args(["-E", "-P", &input_file_path, "-o", &output_file])
        .output()
        .expect("preprocessing source file failed");
}

fn compile(file_name: &&str, options: Vec<String>) {
    let preprocessed_file_name = format!("{}.i", file_name);
    // print!("{:?}", options);

    let tokens = lexer(preprocessed_file_name.clone());
    
    // Command::new("gcc")
    //     .args(["-S", "-O", "-fno-asynchronous-unwind-tables", "-fcf-protection=none", &preprocessed_file_name])
    //     .output()
    //     .expect("compile file failed");
    
    Command::new("rm")
        .arg(preprocessed_file_name)
        .output()
        .expect("rm file");
}

fn lexer(file_name: String) -> Vec<String> {
    let contents = fs::read_to_string(file_name)
        .expect("Should have been able to read the file");
    dbg!(contents);
    return vec!["text".to_string()];
    
}

fn assemble_and_link(file_name: &&str) {
    let compiled_file_name = format!("{}.s", file_name);
    let output_file_name = format!("{}", file_name);
    Command::new("gcc")
        .args([&compiled_file_name, "-o", &output_file_name])
        .output()
        .expect("assembled and linked file");

    Command::new("rm")
        .arg(compiled_file_name)
        .output()
        .expect("rm compiled file");
}