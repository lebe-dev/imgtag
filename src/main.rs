#[macro_use]
extern crate log;
extern crate log4rs;

use crate::logging::logging::get_logging_config;
use clap::{ArgMatches, Arg, App, SubCommand};
use std::process::exit;
use crate::commands::commands::reorganize_files;
use crate::domain::domain::NoExifConfig;
use chrono::Local;
use crate::diag::diag::diag_path;

mod commands;
mod commands_tests;

mod path_parser;
mod path_parser_tests;

mod logging;
mod domain;
mod diag;
mod files;
mod exif;
mod exif_tests;

const REORGANIZE_COMMAND: &str = "reorganize";
const SRC_PATH_ARG: &str = "src-dir";
const DEST_PATH_ARG: &str = "dest-dir";

const DIAG_COMMAND: &str = "diag";

const DONT_EXTRACT_DATE_FROM_PATH_FLAG: &str = "dont-extract-date-from-path";

/// Force year for files without EXIF or without 'Date created' exif-property
const FORCE_YEAR_OPTION: &str = "force-year";
const YEAR_VALUE: &str = "year";

const LOG_LEVEL_ARGUMENT: &str = "log-level";
const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let dont_extract_date_from_path_arg = Arg::with_name(DONT_EXTRACT_DATE_FROM_PATH_FLAG)
        .help("don't extract date from file path for files without EXIF.")
        .long(DONT_EXTRACT_DATE_FROM_PATH_FLAG)
        .takes_value(false)
        .required(false);

    let matches = App::new("imgtag")
        .version("0.3.0")
        .about("Image files hierarchy tool")
        .arg(
            Arg::with_name(LOG_LEVEL_ARGUMENT)
                .help("set logging level. possible values: debug, info, error, warn, trace")
                .long(LOG_LEVEL_ARGUMENT)
                .case_insensitive(true)
                .takes_value(true).required(false)
                .default_value(LOG_LEVEL_DEFAULT_VALUE)
        )
        .subcommand(SubCommand::with_name(REORGANIZE_COMMAND)
            .about("reorganize JPG\\TIFF files in hierarchy YYYY/Month/YYYY-MM-DD__filename.jpg or \
                YYYY/Month/YYYY-MM-DD__HH-MM-SS__filename.jpg \
            Extract picture taken date from EXIF meta-data.")
            .arg(
                Arg::with_name(SRC_PATH_ARG)
                    .help("source path")
                    .value_name(SRC_PATH_ARG)
                    .takes_value(true).required(true)
            )
            .arg(
                Arg::with_name(DEST_PATH_ARG)
                    .help("destination path")
                    .value_name(DEST_PATH_ARG)
                    .takes_value(true).required(true)
            )
            .arg(&dont_extract_date_from_path_arg)
            .arg(
                Arg::with_name(FORCE_YEAR_OPTION)
                    .help("force year for files without EXIF or without 'Date created' exif-property")
                    .long(FORCE_YEAR_OPTION)
                    .value_name(YEAR_VALUE)
                    .takes_value(true).required(false)
            )
        )
        .subcommand(SubCommand::with_name(DIAG_COMMAND)
                        .about("do diagnostics without modifications in filesystem.")
            .arg(
                Arg::with_name(SRC_PATH_ARG)
                    .help("source path")
                    .value_name(SRC_PATH_ARG)
                    .takes_value(true).required(true)
            )
            .arg(dont_extract_date_from_path_arg)
        )
        .get_matches();

    let logging_level: &str = get_logging_level(&matches);
    let logging_config = get_logging_config(logging_level);
    log4rs::init_config(logging_config).unwrap();

    let mut command_matches = false;

    match matches.subcommand_matches(REORGANIZE_COMMAND) {
        Some(args) => {
            command_matches = true;

            let extract_dates_from_path = !args.is_present(DONT_EXTRACT_DATE_FROM_PATH_FLAG);
            info!("extract dates from path: {}", extract_dates_from_path);
            println!("extract dates from path: {}", extract_dates_from_path);

            let src_path: &str = args.value_of(SRC_PATH_ARG)
                                     .expect("invalid value for src-path argument");

            let dest_path: &str = args.value_of(DEST_PATH_ARG)
                                      .expect("invalid value for dest-path argument");

            let (force_year_for_unknown, year) = if matches.is_present(FORCE_YEAR_OPTION) {
                let value_str = matches.value_of(FORCE_YEAR_OPTION).unwrap();
                let year: i32 = value_str.parse::<i32>().unwrap() as i32;
                (true, year)

            } else {
                (false, 0)
            };

            let no_exif_config: NoExifConfig = NoExifConfig {
                extract_dates_from_path,
                force_year: force_year_for_unknown,
                year
            };

            print_operation_start();

            let ext_filters: Vec<String> = get_extension_filters();

            match reorganize_files(src_path, dest_path, &ext_filters,
                                   &no_exif_config, show_reorganize_progress) {
                Ok(_) => {
                    print_operation_finish();
                    println!("\n---\nAll files have been reorganized");
                    exit(0);
                }
                Err(e) => {
                    eprintln!("unable to reorganize image files: {}", e);
                    exit(ERROR_EXIT_CODE)
                }
            }
        }
        None => {}
    }

    match matches.subcommand_matches(DIAG_COMMAND) {
        Some(args) => {
            command_matches = true;

            let extract_dates_from_path = !args.is_present(DONT_EXTRACT_DATE_FROM_PATH_FLAG);
            info!("extract dates from path: {}", extract_dates_from_path);
            println!("extract dates from path: {}", extract_dates_from_path);

            let src_path: &str = args.value_of(SRC_PATH_ARG)
                                     .expect("invalid value for src-path argument");

            print_operation_start();

            let ext_filters: Vec<String> = get_extension_filters();

            println!("Getting files list..");

            match diag_path(src_path, &ext_filters,
                            extract_dates_from_path, show_diag_progress) {
                Ok(diag_report) => {
                    println!("\rFiles total: {}", diag_report.files_total);

                    if diag_report.files_with_issues.is_empty() {
                        println!("---\nAll files are fine. Nothing to do.");

                    } else {
                        println!("---\nUnable to determine date for file(s):");
                        diag_report.files_with_issues.iter().for_each(|file_path| println!("{}", file_path));
                    }

                    print_operation_finish();

                    exit(0);
                }
                Err(e) => {
                    println!("unable to diagnostic path '{}': {}", src_path, e);
                    exit(ERROR_EXIT_CODE)
                }
            }
        }
        None => {}
    }

    if !command_matches {
        println!("{}", matches.usage());
    }
}

fn get_extension_filters() -> Vec<String> {
    vec![String::from("jpg"), String::from("jpeg"), String::from("tiff")]
}

fn show_reorganize_progress(total_elements: usize, current_element_index: usize) {
    print!("\r");
    print!("Progress: {}/{}", current_element_index, total_elements);
}

fn show_diag_progress(total_elements: usize, current_element_index: usize, with_issues: usize) {
    print!("\r");
    print!("Progress: {}/{} (with issues: {})", current_element_index, total_elements, with_issues);
}

fn print_operation_start() {
    print_operation_datetime("Started")
}

fn print_operation_finish() {
    print_operation_datetime("Finished")
}

fn print_operation_datetime(operation_name: &str) {
    let now = Local::now();
    println!("{}: {}", operation_name, now.to_rfc2822());
}

fn get_logging_level<'a>(arg_matches: &'a ArgMatches) -> &'a str {
    if arg_matches.is_present(LOG_LEVEL_ARGUMENT) {
        arg_matches.value_of(LOG_LEVEL_ARGUMENT).unwrap()
    } else { LOG_LEVEL_DEFAULT_VALUE }
}
