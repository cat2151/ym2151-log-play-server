use ym2151_log_play_server::logging;

fn main() {
    // Initialize with verbose mode
    logging::init(true);
    
    println!("Testing verbose mode:");
    logging::log_always("This should print and log (verbose mode)");
    logging::log_verbose("This should also print (verbose mode)");
    
    println!("Done!");
}
