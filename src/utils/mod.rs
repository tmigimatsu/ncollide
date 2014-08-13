//! Miscelaneous, unsorted generic geometric utilities.

pub use utils::center::center;
pub use utils::triangulate::triangulate;
pub use utils::project_homogeneous::{project_homogeneous, project_homogeneous_to};
pub use utils::triangle_utils::{circumcircle, is_affinely_dependent_triangle, is_point_in_triangle,
                                triangle_area};
pub use utils::cleanup::remove_unused_points;
pub use utils::derivatives::{dcos, dsin, binom};
pub use utils::optimization::{maximize_with_newton, newton, minimize_with_bfgs, bfgs,
                              LineSearch, BacktrackingLineSearch};
pub use utils::hashable_partial_eq::HashablePartialEq;
#[doc(inline)]
pub use utils::any_private::AnyPrivate;
#[doc(inline)]
pub use utils::as_bytes::AsBytes;


pub mod symbolic;
mod center;
mod triangulate;
mod project_homogeneous;
mod triangle_utils;
mod cleanup;
mod derivatives;
mod optimization;
mod hashable_partial_eq;
#[doc(hidden)]
pub mod any_private;
#[doc(hidden)]
pub mod as_bytes;
