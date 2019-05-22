//! # MS-OFORMS Implementation
//!
//! This project aims to be a rust-native implementation of [MS-OFORMS]
//! as covered by the Microsoft open specification promise. As this format
//! is pretty deeply integrated in the COM/OLE/ActiveX architecture, expect
//! some very generic types to be declared here at first and moved out into
//! other crates eventually, before reaching 1.0.
//!
//! This crate is not made to be binary compatible with the COM-ABI on windows
//! systems, but to get access to the contained data on any platform.
//!
//! [MS-OFORMS]: https://docs.microsoft.com/openspecs/office_file_formats/ms-oforms/

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate nom;
#[macro_use]
extern crate num_derive;

pub mod common;
pub mod controls;
pub mod properties;
