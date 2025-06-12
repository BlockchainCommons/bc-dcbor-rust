#![allow(unused_imports)]

#[cfg(feature = "std")]
#[doc(hidden)]
pub(crate) mod with_std {
    pub(crate) use std::{
        array::TryFromSliceError,
        borrow::ToOwned,
        boxed::Box,
        cmp,
        collections::{
            BTreeMap, BTreeSet, HashMap, HashSet, VecDeque,
            btree_map::Values as BTreeMapValues,
        },
        fmt, format, hash,
        ops::Deref,
        result::Result as StdResult,
        str,
        string::{String, ToString},
        sync::{Arc, Mutex, MutexGuard, Once},
        time::Duration,
        vec::Vec,
    };

    pub(crate) use thiserror::Error as ThisError;
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
#[doc(hidden)]
pub(crate) mod without_std {
    extern crate alloc;

    // pub(crate) use alloc::rc; // Unused import
    pub(crate) use alloc::str;
    pub(crate) use alloc::{
        borrow::ToOwned,
        boxed::Box,
        collections::{
            BTreeMap, BTreeSet, VecDeque, btree_map::Values as BTreeMapValues,
        },
        fmt, format,
        string::{String, ToString},
        sync::Arc,
        vec,
        vec::Vec,
    };
    pub(crate) use core::{
        array::TryFromSliceError, cmp, hash, ops::Deref,
        result::Result as StdResult, time::Duration,
    };

    pub(crate) use hashbrown::{HashMap, HashSet};
    pub(crate) use spin::{Mutex, MutexGuard, Once};
    pub(crate) use thiserror_no_std::Error as ThisError;
}

// Re-export items that are needed in the public API
#[cfg(feature = "std")]
pub mod public_exports {
    pub use std::{
        boxed::Box,
        collections::{HashMap, HashSet},
        string::String,
        sync::Arc,
        vec::Vec,
    };
}

#[cfg(not(feature = "std"))]
#[cfg(feature = "no_std")]
pub mod public_exports {
    extern crate alloc;
    pub use alloc::{boxed::Box, string::String, sync::Arc, vec::Vec};

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
