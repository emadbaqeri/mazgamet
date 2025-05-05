use chrono::Local;
use log::{LevelFilter, SetLoggerError};
use simplelog::{Config, WriteLogger};
use std::fs::File;

pub fn init_logger(log_level: LevelFilter) -> Result<(), SetLoggerError> {
    // Create a timestamp for the log file name
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let log_file_path = format!("lexer_{}.log", timestamp);

    // Print the log file path to make it easier to find
    println!("Logging to file: {}", log_file_path);

    // Create the log file
    let file = File::create(&log_file_path).expect("Failed to create log file");

    // Initialize the logger
    WriteLogger::init(log_level, Config::default(), file)
}

pub fn init_test_logger() -> Result<(), SetLoggerError> {
    // For tests, we'll use a specific file
    let log_file_path = "lexer_test.log";

    // Create the log file, truncating it if it already exists
    let file = File::create(log_file_path).expect("Failed to create test log file");

    // Initialize the logger with Debug level for tests
    WriteLogger::init(LevelFilter::Debug, Config::default(), file)
}
