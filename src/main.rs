#[macro_use]
extern crate log;
extern crate log4rs;

use crate::logging::logging::get_logging_config;
use clap::{ArgMatches, Arg, App, SubCommand};
use std::process::exit;
use crate::commands::commands::reorganize_files;
use crate::domain::domain::NoExifConfig;

mod commands;
mod commands_tests;

mod path_parser;
mod path_parser_tests;

mod logging;
mod domain;

const REORGANIZE_COMMAND: &str = "reorganize";
const SRC_PATH_ARG: &str = "src-dir";
const DEST_PATH_ARG: &str = "dest-dir";

const EXTRACT_DATE_FROM_PATH_OPTION: &str = "extract-date-from-path";

/// Force year for files without EXIF or without 'Date created' exif-property
const FORCE_YEAR_OPTION: &str = "force-year";
const YEAR_VALUE: &str = "year";

const LOG_LEVEL_ARGUMENT: &str = "log-level";
const LOG_LEVEL_DEFAULT_VALUE: &str = "info";

const ERROR_EXIT_CODE: i32 = 1;

fn main() {
    let matches = App::new("imgtag")
        .version("0.1.0 BETA")
        .about("Image tagging tool")
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
            .arg(
                Arg::with_name(EXTRACT_DATE_FROM_PATH_OPTION)
                    .help("try to extract date from file path for files without EXIF. \
                         Supported date formats: yyyyMMdd, yyyy-MM-dd, yyyy.MM.dd. \
                         Takes the nearest date to filename. Has lower priority than 'year' option.")
                    .long(EXTRACT_DATE_FROM_PATH_OPTION)
                    .takes_value(false)
                    .required(false)
            )
            .arg(
                Arg::with_name(FORCE_YEAR_OPTION)
                    .help("force year for files without EXIF or without 'Date created' exif-property")
                    .value_name(YEAR_VALUE)
                    .takes_value(true).required(false)
            )
        )
        .get_matches();

    let extract_dates_from_path = matches.is_present(EXTRACT_DATE_FROM_PATH_OPTION);

    let logging_level: &str = get_logging_level(&matches);
    let logging_config = get_logging_config(logging_level);
    log4rs::init_config(logging_config).unwrap();

    let mut command_matches = false;

    match matches.subcommand_matches(REORGANIZE_COMMAND) {
        Some(args) => {
            command_matches = true;

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

            match reorganize_files(src_path, dest_path,
                                   &no_exif_config, show_progress) {
                Ok(_) => {
                    println!("\nAll files have been reorganized");
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

    if !command_matches {
        println!("{}", matches.usage());
    }
}

fn show_progress(total_elements: usize, current_element_index: usize) {
    print!("\r");
    print!("Progress: {}/{}", current_element_index, total_elements);
}

fn get_logging_level<'a>(arg_matches: &'a ArgMatches) -> &'a str {
    if arg_matches.is_present(LOG_LEVEL_ARGUMENT) {
        arg_matches.value_of(LOG_LEVEL_ARGUMENT).unwrap()
    } else { LOG_LEVEL_DEFAULT_VALUE }
}
