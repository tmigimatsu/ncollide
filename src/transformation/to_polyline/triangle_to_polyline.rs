use super::ToPolyline;
use alga::general::Real;
use crate::procedural::Polyline;
use crate::shape::Triangle;

impl<N: Real> ToPolyline<N> for Triangle<N> {
    type DiscretizationParameter = ();

    fn to_polyline(&self, _: ()) -> Polyline<N> {
        Polyline::new(vec![*self.a(), *self.b(), *self.c()], None)
    }
}
