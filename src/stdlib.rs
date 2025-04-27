#![allow(unused_imports)]

#[cfg(feature = "std")]
#[doc(hidden)]
pub(crate) mod with_std {
    pub(crate) use std::fmt;
    pub(crate) use std::str;

    pub(crate) use std::array::TryFromSliceError;
    pub(crate) use std::borrow::ToOwned;
    pub(crate) use std::boxed::Box;
    pub(crate) use std::cmp;
    pub(crate) use std::collections::{BTreeMap, BTreeSet, btree_map::Values as BTreeMapValues, VecDeque, HashSet, HashMap};
    pub(crate) use std::format;
    pub(crate) use std::hash;
    pub(crate) use std::ops::Deref;
    pub(crate) use std::string::{String, ToString};
    pub(crate) use std::sync::{Arc, Once, Mutex, MutexGuard};
    pub(crate) use std::time::Duration;
    pub(crate) use std::vec::Vec;
    pub(crate) use std::result::Result as StdResult;
    pub(crate) use thiserror::Error as ThisError;
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
#[doc(hidden)]
pub(crate) mod without_std {
    extern crate alloc;

    pub(crate) use alloc::borrow::ToOwned;
    pub(crate) use alloc::boxed::Box;
    pub(crate) use alloc::collections::{BTreeMap, BTreeSet, btree_map::Values as BTreeMapValues, VecDeque};
    pub(crate) use alloc::fmt;
    pub(crate) use alloc::format;
    // pub(crate) use alloc::rc; // Unused import
    pub(crate) use alloc::str;
    pub(crate) use alloc::string::{String, ToString};
    pub(crate) use alloc::sync::Arc;
    pub(crate) use alloc::vec;
    pub(crate) use alloc::vec::Vec;
    pub(crate) use core::result::Result as StdResult;
    pub(crate) use core::array::TryFromSliceError;
    pub(crate) use core::cmp;
    pub(crate) use core::hash;
    pub(crate) use core::ops::Deref;
    pub(crate) use core::time::Duration;
    pub(crate) use hashbrown::{HashSet, HashMap};
    pub(crate) use spin::{Once, Mutex, MutexGuard};
    pub(crate) use thiserror_no_std::Error as ThisError;
}

// Re-export items that are needed in the public API
#[cfg(feature = "std")]
pub mod public_exports {
    pub use std::sync::Arc;
    pub use std::vec::Vec;
    pub use std::string::String;
    pub use std::collections::{HashMap, HashSet};
    pub use std::boxed::Box;
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
pub mod public_exports {
    extern crate alloc;
    pub use alloc::sync::Arc;
    pub use alloc::vec::Vec;
    pub use alloc::string::String;
    pub use alloc::boxed::Box;
    pub use hashbrown::{HashMap, HashSet};
}

macro_rules! import_stdlib {
    () => {
        #[allow(unused_imports)]
        #[cfg(feature = "std")]
        use $crate::stdlib::with_std::*;
        #[allow(unused_imports)]
        #[cfg(not(feature = "std"))]
        use $crate::stdlib::without_std::*;
    };
}
