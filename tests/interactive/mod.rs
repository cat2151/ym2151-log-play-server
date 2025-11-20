//! Interactive mode tests
//!
//! This module contains tests for the interactive mode functionality,
//! which are currently experiencing bugs and need debugging.
//!
//! These tests are separated from the main test suite to allow
//! selective execution and prevent interference with stable tests.

// Shared mutex for interactive tests
mod shared_mutex;

// Import test modules
mod mode_test;
mod step_by_step_test;
mod play_json_test;
mod row_by_row_test;

// Re-export any public test utilities if needed
// (Currently none, but structure is ready for future additions)
