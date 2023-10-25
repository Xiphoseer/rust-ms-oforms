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

use std::{
    convert::TryFrom,
    io::{self, Read},
    ops::{Deref, DerefMut},
};

use cfb::CompoundFile;
use controls::form::{parse_form_control, FormControl};
use nom::error::VerboseError;

#[macro_use]
extern crate bitflags;
#[macro_use]
extern crate num_derive;

pub mod common;
pub mod controls;
pub mod properties;

/// An OForms file is a [`cfb::CompoundFile`].
pub struct OFormsFile<F> {
    inner: CompoundFile<F>,
}

impl<T: Read + io::Seek> OFormsFile<T> {
    /// Create a new instance by opening the underlying [`cfb::CompoundFile`]
    pub fn open(buf: T) -> io::Result<Self> {
        Ok(Self {
            inner: CompoundFile::open(buf)?,
        })
    }

    /// Get the form stream (`f`)
    ///
    /// https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-oforms/cb5df5d6-e090-4bf3-a328-c4edaff0c66b
    pub fn root_form_stream(&mut self) -> io::Result<cfb::Stream<T>> {
        self.inner.open_stream("/f")
    }

    /// Get the object stream (`o`)
    ///
    /// See <https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-oforms/15778df8-8a8e-45dc-933b-f914f4e011cf>
    pub fn root_object_stream(&mut self) -> io::Result<cfb::Stream<T>> {
        self.inner.open_stream("/o")
    }

    /// Get the CompObj stream (`\001CompObj`)
    ///
    /// See: <https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-oforms/a0043a76-919e-4d2a-9a90-83daacbfaba2>
    pub fn root_comp_obj_stream(&mut self) -> io::Result<cfb::Stream<T>> {
        self.inner.open_stream("/\x01CompObj")
    }

    pub fn root_form_control(&mut self) -> io::Result<FormControl> {
        let mut f_stream = self.root_form_stream()?;
        let f_stream_len = usize::try_from(f_stream.len())
            .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        let mut bytes: Vec<u8> = Vec::with_capacity(f_stream_len);
        f_stream.read_to_end(&mut bytes)?;
        let (_rest, form_control) =
            parse_form_control::<VerboseError<&[u8]>>(&bytes[..]).map_err(|e| {
                let e = e.map(|e| VerboseError {
                    errors: e
                        .errors
                        .into_iter()
                        .map(|(rest, kind)| (bytes.len() - rest.len(), kind))
                        .collect(),
                });
                io::Error::new(io::ErrorKind::InvalidData, e)
            })?;
        Ok(form_control)
    }
}

impl<T> Deref for OFormsFile<T> {
    type Target = CompoundFile<T>;

    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for OFormsFile<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}
