#[cfg(feature = "std")]
pub mod with_std {
    pub use std::{fmt, str::FromStr};

    pub use std::string::String;
    pub use std::vec::Vec;
    pub use std::string::ToString;
    pub use std::borrow::ToOwned;
    pub use std::boxed::Box;
    pub use std::collections::{BTreeMap, btree_map::Values as BTreeMapValues, VecDeque, HashSet, HashMap};
    pub use std::hash::{self};
    pub use std::rc::{self};
    pub use std::sync::{self};
    pub use std::ops::{self};
    pub use std::cmp::{self};
    pub use std::str::{self};
    pub use std::time::Duration;
    pub use std::format;
    pub use thiserror::Error as ThisError;
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
pub mod without_std {
    extern crate alloc;

    pub use alloc::fmt::{self};
    pub use alloc::string::String;
    pub use alloc::vec;
    pub use alloc::vec::Vec;
    pub use alloc::boxed::Box;
    pub use alloc::collections::{BTreeMap, btree_map::Values as BTreeMapValues, VecDeque};
    pub use hashbrown::{HashSet, HashMap};
    pub use core::hash::{self};
    pub use core::ops::{self};
    pub use core::cmp::{self};
    pub use core::time::Duration;
    pub use alloc::rc::{self};
    pub use alloc::sync::{self};
    pub use alloc::string::ToString;
    pub use alloc::str::{self};
    pub use alloc::borrow::ToOwned;
    pub use alloc::format;
    pub use thiserror_no_std::Error as ThisError;

    pub trait StdError: fmt::Debug + fmt::Display { }
}

macro_rules! import_stdlib {
    () => {
        #[allow(unused_imports)]
        #[cfg(feature = "std")]
        pub use $crate::stdlib::with_std::*;
        #[allow(unused_imports)]
        #[cfg(not(feature = "std"))]
        pub use $crate::stdlib::without_std::*;
    };
}
