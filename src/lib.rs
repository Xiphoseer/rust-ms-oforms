#![doc(html_root_url = "https://xiphoseer.de/rust-ms-oforms")]
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
    io::{self, Read, Seek},
    ops::{Deref, DerefMut, Range},
};

use cfb::{CompoundFile, Stream};
use common::{parse_comp_obj, CompObj};
use controls::{
    form::{parse_form_control, ClassTable, FormControl, Site},
    ole_site_concrete::{Clsid, OleSiteConcrete},
};
use nom::{error::VerboseError, Err};
use uuid::Uuid;

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

fn map_verbose_err(input: &[u8]) -> impl Fn(Err<VerboseError<&[u8]>>) -> io::Error + 'static {
    let len = input.len();
    move |e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            e.map(move |e: VerboseError<&[u8]>| VerboseError {
                errors: e
                    .errors
                    .into_iter()
                    .map(|(rest, kind)| (len - rest.len(), kind))
                    .collect(),
            }),
        )
    }
}

fn read_to_end<T: Read + Seek>(f_stream: &mut Stream<T>) -> io::Result<Vec<u8>> {
    let f_stream_len = usize::try_from(f_stream.len())
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    let mut bytes: Vec<u8> = Vec::with_capacity(f_stream_len);
    f_stream.read_to_end(&mut bytes)?;
    Ok(bytes)
}

pub struct Form<F> {
    form_control: FormControl,
    obj_stream: Stream<F>,
}

impl<F> Form<F> {
    pub fn site_iter(&mut self) -> SiteIter<'_, F> {
        SiteIter {
            stream: &mut self.obj_stream,
            range: 0..0,
            sites: self.form_control.sites.iter(),
            classes: &self.form_control.site_classes,
        }
    }
}

pub struct SiteIter<'a, F> {
    stream: &'a mut Stream<F>,
    range: Range<usize>,
    sites: std::slice::Iter<'a, Site>,
    classes: &'a [ClassTable],
}

impl<'a, F: Read + Seek> SiteIter<'a, F> {
    pub fn site_stream(&mut self) -> io::Result<std::io::Take<&mut cfb::Stream<F>>> {
        self.stream
            .seek(io::SeekFrom::Start(self.range.start as u64))?;
        Ok(self.stream.take(self.range.len() as u64))
    }
}

impl<'a, F> Iterator for SiteIter<'a, F> {
    type Item = (Uuid, &'a OleSiteConcrete);

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(s) = self.sites.next() {
            let Site::Ole(ole_site) = s;
            self.range = self.range.end..self.range.end + (ole_site.object_stream_size as usize);
            let clsid = match ole_site.clsid_cache_index {
                Clsid::ClassTable(c) => self.classes.get(c as usize).unwrap().cls_id,
                Clsid::Invalid => todo!(),
                Clsid::Global(_) => todo!(),
            };
            Some((clsid, ole_site))
        } else {
            None
        }
    }
}

impl<T: Read + Seek> OFormsFile<T> {
    /// Create a new instance by opening the underlying [`cfb::CompoundFile`]
    pub fn open(buf: T) -> io::Result<Self> {
        Ok(Self {
            inner: CompoundFile::open(buf)?,
        })
    }

    /// Get the form stream (`f`)
    ///
    /// See <https://learn.microsoft.com/en-us/openspecs/office_file_formats/ms-oforms/cb5df5d6-e090-4bf3-a328-c4edaff0c66b>
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

    pub fn root_comp_obj(&mut self) -> io::Result<CompObj> {
        let mut f_stream = self.root_comp_obj_stream()?;
        let bytes = read_to_end(&mut f_stream)?;
        let (_rest, form_control) = parse_comp_obj(&bytes).map_err(map_verbose_err(&bytes))?;
        Ok(form_control)
    }

    pub fn root_form_control(&mut self) -> io::Result<FormControl> {
        let mut f_stream = self.root_form_stream()?;
        let bytes = read_to_end(&mut f_stream)?;
        let (_rest, form_control) = parse_form_control(&bytes).map_err(map_verbose_err(&bytes))?;
        Ok(form_control)
    }

    pub fn root_form(&mut self) -> io::Result<Form<T>> {
        let form_control = self.root_form_control()?;
        let obj_stream = self.root_object_stream()?;
        Ok(Form {
            form_control,
            obj_stream,
        })
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
