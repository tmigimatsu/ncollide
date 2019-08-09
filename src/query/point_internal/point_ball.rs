use alga::general::Real;
use na;

use crate::math::{Isometry, Point, Vector};
use crate::query::{PointProjection, PointQuery};
use crate::shape::{Ball, FeatureId};
use crate::utils::IsometryOps;

impl<N: Real> PointQuery<N> for Ball<N> {
    #[inline]
    fn project_point(&self, m: &Isometry<N>, pt: &Point<N>, solid: bool) -> PointProjection<N> {
        let ls_pt = m.inverse_transform_point(pt);
        let distance_squared = ls_pt.coords.norm_squared();

        let inside = distance_squared <= self.radius() * self.radius();

        if inside && solid {
            PointProjection::new(true, *pt)
        } else if distance_squared == na::zero() {
            let ls_proj = Point::origin() + Vector::x() * self.radius();
            PointProjection::new(inside, m * ls_proj)
        } else {
            let ls_proj = Point::origin() + ls_pt.coords / distance_squared.sqrt() * self.radius();
            PointProjection::new(inside, m * ls_proj)
        }
    }

    #[inline]
    fn project_point_with_feature(
        &self,
        m: &Isometry<N>,
        pt: &Point<N>,
    ) -> (PointProjection<N>, FeatureId)
    {
        (self.project_point(m, pt, false), FeatureId::Face(0))
    }

    #[inline]
    fn distance_to_point(&self, m: &Isometry<N>, pt: &Point<N>, solid: bool) -> N {
        let dist = m.inverse_transform_point(pt).coords.norm() - self.radius();

        if solid && dist < na::zero() {
            na::zero()
        } else {
            dist
        }
    }

    #[inline]
    fn contains_point(&self, m: &Isometry<N>, pt: &Point<N>) -> bool {
        m.inverse_transform_point(pt).coords.norm_squared() <= self.radius() * self.radius()
    }
}
