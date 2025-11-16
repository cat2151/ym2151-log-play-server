use ym2151_log_play_server::logging;

fn main() {
    // Initialize with non-verbose mode
    logging::init(false);
    
    println!("Testing non-verbose mode:");
    logging::log_always("This should only log to file, not print (non-verbose mode)");
    logging::log_verbose("This should not print or log (non-verbose mode)");
    
    println!("Done!");
}
