//! Point inclusion and projection.

#[doc(inline)]
pub use self::point_query::{PointProjection, PointQuery, PointQueryWithLocation};

mod point_aabb;
mod point_ball;
mod point_bounding_sphere;
mod point_capsule;
mod point_compound;
mod point_cuboid;
mod point_plane;
mod point_polyline;
#[doc(hidden)]
pub mod point_query;
mod point_segment;
mod point_shape;
mod point_support_map;
#[cfg(feature = "dim3")]
mod point_tetrahedron;
mod point_triangle;
#[cfg(feature = "dim3")]
mod point_trimesh;
mod point_heightfield;
