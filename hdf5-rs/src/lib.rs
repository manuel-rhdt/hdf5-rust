#![cfg_attr(feature = "cargo-clippy", allow(clippy::block_in_if_condition_stmt))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::needless_return))]
#![cfg_attr(all(feature = "cargo-clippy", test), allow(clippy::cyclomatic_complexity))]
#![cfg_attr(not(test), allow(dead_code))]

mod export {
    pub use crate::{
        container::Container,
        dataset::Dataset,
        datatype::Datatype,
        error::{Result, Error},
        file::File,
        filters::Filters,
        group::Group,
        location::Location,
        object::Object,
        space::{Dimension, Ix, Dataspace},
    };
}

pub use crate::export::*;

pub use hdf5_types::{self as types, H5Type};
pub use hdf5_derive::H5Type;

#[macro_use]
mod macros;

mod container;
mod dataset;
mod datatype;
mod error;
mod file;
mod filters;
mod group;
mod handle;
mod location;
mod object;
mod plist;
mod space;
mod sync;
mod util;

#[allow(dead_code)]
mod globals;

pub mod prelude {
    //! The HDF5 prelude module.
    //!
    //! The purpose of this module is to provide reexports of many core `hdf5` traits so that
    //! they can be then glob-imported all at once:
    //!
    //! ```ignore
    //! use h5::prelude::*;
    //! ```
    //! This module provides reexports of such traits as `Object`, `Location` and `Container`
    //! and does not expose any structures or functions.

    pub use super::{Object, Location, Container, Dimension, H5Type};
}

mod internal_prelude {
    pub use libc::{c_int, c_uint, c_void, c_char, size_t};

    pub use ffi::h5::{hsize_t, hbool_t, haddr_t, herr_t};
    pub use ffi::h5i::{H5I_INVALID_HID, hid_t};
    pub use ffi::h5p::H5P_DEFAULT;
    pub use ffi::h5i::H5I_type_t::*;

    pub use crate::export::*;
    pub use crate::types::H5Type;

    pub use crate::dataset::DatasetBuilder;
    pub use crate::error::silence_errors;
    pub use crate::file::FileBuilder;
    pub use crate::handle::{Handle, ID, FromID, get_id_type};
    pub use crate::plist::PropertyList;
    pub use crate::util::{to_cstring, string_from_cstr, get_h5_str};

    #[cfg(test)]
    pub use crate::test::{with_tmp_file, with_tmp_dir, with_tmp_path};
}

#[cfg(test)]
pub mod test;

/// Returns the version of the HDF5 library that the crate was compiled against.
pub fn hdf5_version() -> (u8, u8, u8) {
    h5lock!(libhdf5_lib::hdf5_version()).unwrap_or((0, 0, 0))
}

#[cfg(test)]
pub mod tests {
    use super::hdf5_version;

    #[test]
    pub fn test_hdf5_version() {
        assert!(hdf5_version() >= (1, 8, 0));
    }
}
