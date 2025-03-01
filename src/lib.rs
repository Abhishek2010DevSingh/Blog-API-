//! # Project Overview
//! This Rust project enforces strict linting rules to maintain high code quality.
//! It includes modules for handling the database, errors, server logic, and application state.

#![deny(unused_variables)] // Prevents unused variables.
#![deny(unused_imports)] // Prevents unused imports.
#![deny(clippy::unwrap_used)] // Disallows usage of `unwrap` to prevent panics.
#![deny(clippy::expect_used)] // Disallows `expect` calls to ensure proper error handling.
#![deny(clippy::panic)] // Forbids explicit panics in the code.
#![deny(clippy::todo)] // Prevents `todo!()` macros from being left in the code.
#![deny(clippy::unimplemented)] // Prevents `unimplemented!()` macros.
#![deny(clippy::dbg_macro)] // Disallows `dbg!` macro usage.
#![deny(clippy::print_stdout)] // Prevents usage of `println!` to enforce structured logging.
#![deny(clippy::print_stderr)] // Prevents `eprintln!` usage.
#![deny(clippy::missing_docs_in_private_items)] // Ensures all private items have documentation.
#![deny(clippy::missing_errors_doc)] // Requires documentation for errors in function signatures.
#![deny(clippy::missing_panics_doc)] // Requires documentation for panics in functions.
#![deny(clippy::large_enum_variant)] // Discourages large enum variants to optimize memory usage.
#![deny(clippy::too_many_arguments)] // Limits the number of arguments in functions to improve readability.
#![deny(clippy::type_complexity)] // Discourages overly complex types.
#![deny(clippy::cast_lossless)] // Ensures all numeric casts are lossless.
#![deny(clippy::cast_possible_truncation)] // Prevents possible truncation when casting.
#![deny(clippy::cast_possible_wrap)] // Avoids wrapping behavior during casting.
#![deny(clippy::cast_precision_loss)] // Prevents loss of precision during casts.
#![deny(clippy::cast_sign_loss)] // Disallows sign loss during casting.

/// Module for handling database operations.
pub mod database;

/// Module for handling errors within the application.
pub mod error;
/// Module for defining application models.
pub mod model;
/// Module for handling server logic.
pub mod server;
/// Module for maintaining application state.
pub mod state;
