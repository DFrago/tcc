use std::process::Command;

fn main(){
    //TODO FAILURE BEHAVIOR IS NOT CONSISTENT WITH THE TESTS!
    let args:Vec<String>=std::env::args().collect(); 

    let input_file:&str=match args.len(){
        2=>args.get(1).unwrap().strip_suffix(".c").expect("Input file must end in .c"), 
        _=>panic!("Usage: <file>")
    };

    let preprocessed_file=&format!("{input_file}.i");
    let assembly_file=&format!("{input_file}.s");

    Command::new("gcc")
        .args(["-E","-P",input_file,&preprocessed_file])
        .status()
        .expect("Failed to preprocess with gcc");

    Command::new("./tcc")
        .args([preprocessed_file,"-o",assembly_file])
        .status()
        .expect("Failed to compile with tcc");

    std::fs::remove_file(preprocessed_file)
        .unwrap_or_else(|e|eprintln!("Failed to delete {preprocessed_file}: {e}"));

    Command::new("./gcc")
        .args([assembly_file,"-o",input_file])
        .status()
        .expect("Failed to assemble and link with gcc");

    std::fs::remove_file(assembly_file)
        .unwrap_or_else(|e|eprintln!("Failed to delete {assembly_file}: {e}"));

}
