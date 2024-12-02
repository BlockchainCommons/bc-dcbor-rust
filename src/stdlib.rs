#[cfg(feature = "std")]
pub mod with_std {
    pub use std::{fmt, str::FromStr};

    pub use std::array::TryFromSliceError;
    pub use std::borrow::ToOwned;
    pub use std::boxed::Box;
    pub use std::cmp::{self};
    pub use std::collections::{BTreeMap, btree_map::Values as BTreeMapValues, VecDeque, HashSet, HashMap};
    pub use std::format;
    pub use std::hash::{self};
    pub use std::ops::{self, Deref};
    pub use std::rc::{self};
    pub use std::str::{self};
    pub use std::string::{String, ToString};
    pub use std::sync::{self, Arc, Once, Mutex, MutexGuard};
    pub use std::time::Duration;
    pub use std::vec::Vec;
    pub use thiserror::Error as ThisError;
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
pub mod without_std {
    extern crate alloc;

    pub use alloc::borrow::ToOwned;
    pub use alloc::boxed::Box;
    pub use alloc::collections::{BTreeMap, btree_map::Values as BTreeMapValues, VecDeque};
    pub use alloc::fmt::{self};
    pub use alloc::format;
    pub use alloc::rc::{self};
    pub use alloc::str::{self};
    pub use alloc::string::{String, ToString};
    pub use alloc::sync::{self, Arc};
    pub use alloc::vec;
    pub use alloc::vec::Vec;
    pub use core::array::TryFromSliceError;
    pub use core::cmp::{self};
    pub use core::hash::{self};
    pub use core::ops::{self, Deref};
    pub use core::time::Duration;
    pub use hashbrown::{HashSet, HashMap};
    pub use spin::{Once, Mutex, MutexGuard};
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
