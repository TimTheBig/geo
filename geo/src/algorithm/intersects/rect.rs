use super::Intersects;
use crate::*;

impl<T> Intersects<Coord<T>> for Rect<T>
where
    T: CoordNum,
{
    fn intersects(&self, rhs: &Coord<T>) -> bool {
        rhs.x >= self.min().x
            && rhs.y >= self.min().y
            && rhs.z >= self.min().z
            && rhs.x <= self.max().x
            && rhs.y <= self.max().y
            && rhs.z <= self.max().z
    }
}
symmetric_intersects_impl!(Coord<T>, Rect<T>);
symmetric_intersects_impl!(Rect<T>, Point<T>);
symmetric_intersects_impl!(Rect<T>, MultiPoint<T>);

impl<T> Intersects<Rect<T>> for Rect<T>
where
    T: CoordNum,
{
    fn intersects(&self, other: &Rect<T>) -> bool {
        if self.max().x < other.min().x || self.max().y < other.min().y || self.max().z < other.min().z {
            return false;
        }

        if self.min().x > other.max().x || self.min().y > other.max().y || self.min().z > other.max().z {
            return false;
        }

        true
    }
}

// Same logic as Polygon<T>: Intersects<Line<T>>, but avoid
// an allocation.
impl<T> Intersects<Line<T>> for Rect<T>
where
    T: GeoNum,
{
    // todo check 3D, maybe more lines are needed
    fn intersects(&self, rhs: &Line<T>) -> bool {
        // left top
        let lt = self.min();
        // right bottom
        let rb = self.max();
        let lb = Coord::from((lt.x, rb.y, rb.z));
        let rt = Coord::from((rb.x, lt.y, lt.z));
        // If either rhs.{start,end} lies inside Rect, then true
        self.intersects(&rhs.start)
            || self.intersects(&rhs.end)
            || Line::new(lt, rt).intersects(rhs)
            || Line::new(rt, rb).intersects(rhs)
            || Line::new(lb, rb).intersects(rhs)
            || Line::new(lt, lb).intersects(rhs)
    }
}
symmetric_intersects_impl!(Line<T>, Rect<T>);

impl<T> Intersects<Triangle<T>> for Rect<T>
where
    T: GeoNum,
{
    fn intersects(&self, rhs: &Triangle<T>) -> bool {
        self.intersects(&rhs.to_polygon())
    }
}
symmetric_intersects_impl!(Triangle<T>, Rect<T>);
