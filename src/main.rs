use clap::{App, Arg};
use std::fs;
use anyhow::{ensure, Result, Context};

fn main() -> Result<()> {

    // set absolute path for template file
    let path_to_template_file = "/path/to/template.cpp";
    let path_to_template_file_cf = "/path/to/template_cf.cpp";
    let path_to_template_file_mini = "/path/to/template_mini.cpp";
    let generated_file_name = "main.cpp";

    // read command line options
    let command_line_options = App::new("gentemp")
        .version("0.1")
        .author("tabae")
        .about("Generating template directory and files for competitive programming contests")
        .arg(
            Arg::with_name("directory_name")
                .help("Directory name of generated directory")
                .default_value("dir_generated_by_gentemp")
                .index(1)
                .required(false)
        )
        .arg(
            Arg::with_name("destination_path")
                .short("d")
                .long("destination")
                .help("Set path to generate directory.")
                .default_value(".")
                .required(false)
        )
        .arg(
            Arg::with_name("source_path")
                .short("s")
                .long("source")
                .help("Set path to source file.")
                .required(false)
        )
        .arg(
            Arg::with_name("number_of_problems")
                .short("n")
                .help("Set the number of problems")
                .default_value("8")
                .required(false)
        )
        .arg(
            Arg::with_name("codeforces")
                .long("cf")
                .help("Generate template files for Codeforces")
                .required(false)
        )
        .arg(
            Arg::with_name("minimum_template")
                .long("mini")
                .help("Generate template files which has minimum amount of lines")
                .required(false)
        )
        .get_matches();
    
    // determine directory name, number of problems, source path and destionation path,
    // Note: priority of source path is [--source] > [--cf] > [--mini].
    let directory_name = command_line_options.value_of("directory_name").unwrap();
    let number_of_problems = command_line_options.value_of("number_of_problems").unwrap().parse::<u8>().with_context(|| format!("-n option must be followed by an integer which is smaller than or equal to 26"))?;
    ensure!(number_of_problems <= 26, "Number of problems must smaller than or equal to 26");
    let mut source_path = path_to_template_file;
    if command_line_options.is_present("source_path") {
        source_path = command_line_options.value_of("source_path").unwrap();
    } else if command_line_options.is_present("codeforces") {
        source_path = path_to_template_file_cf;
    } else if command_line_options.is_present("minimum_template") {
        source_path = path_to_template_file_mini;
    }
    let destination_path = command_line_options.value_of("destination_path").unwrap();

    // generating directory and template files
    let base_directory = format!("{}/{}", destination_path, directory_name);
    mkdir(&base_directory)?;
    for i in 0..number_of_problems {
        let sub_directory_name : char = ('a' as u8 + i) as char;
        let sub_directory = format!("{}/{}", base_directory, sub_directory_name);
        mkdir(&sub_directory)?;
        let destination_file_path = format!("{}/{}", sub_directory, generated_file_name);
        cp(&source_path, &destination_file_path)?;
    }
    Ok(())

}

fn mkdir(path: &str) -> Result<()> {
    fs::create_dir(path).with_context(|| format!("faild in `mkdir {}", path))?;
    Ok(())
}

fn cp(src: &str, dst: &str) -> Result<()> {
    fs::copy(src, dst).with_context(|| format!("failed in `cp {} {}`", src, dst))?;
    Ok(())
} 