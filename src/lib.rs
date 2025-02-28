// SPDX-License-Identifier: GPL-3.0-only

//! This crate provides bindings to the COSMIC wayland protocol extensions.
//!
//! These bindings are built on top of the crates wayland-client and wayland-server.
//!
//! Each protocol module contains a `client` and a `server` submodules, for each side of the
//! protocol. The creation of these modules (and the dependency on the associated crate) is
//! controlled by the two cargo features `client` and `server`.

#![warn(missing_docs)]
#![forbid(improper_ctypes, unsafe_op_in_unsafe_fn)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![cfg_attr(rustfmt, rustfmt_skip)]

#[macro_use]
mod protocol_macro;

pub mod export_dmabuf {
    //! Low overhead capture interface.

    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!(
            "./unstable/cosmic-export-dmabuf-unstable-v1.xml",
            [crate::workspace::v1, crate::toplevel_info::v1]
        );
    }
}

pub mod toplevel_info {
    //! Receive information about toplevel surfaces.

    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!(
            "./unstable/cosmic-toplevel-info-unstable-v1.xml",
            [crate::workspace::v1]
        );
    }
}

pub mod toplevel_management {
    //! Modify state toplevel surfaces.

    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!(
            "./unstable/cosmic-toplevel-management-unstable-v1.xml",
            [crate::toplevel_info::v1]
        );
    }
}

pub mod workspace {
    //! Receive information about and control workspaces.

    #[allow(missing_docs)]
    pub mod v1 {
        wayland_protocol!(
            "./unstable/cosmic-workspace-unstable-v1.xml",
            []
        );
    }
}
