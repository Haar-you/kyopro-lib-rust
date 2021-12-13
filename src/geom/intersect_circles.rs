use crate::geom::*;

#[derive(Clone, Copy, Debug, PartialEq)]
#[allow(non_camel_case_types)]
pub enum IntersectCircles {
    SAME,
    INSIDE,
    INSCRIBED,
    INTERSECTED,
    CIRCUMSCRIBED,
    OUTSIDE,
}

impl IntersectCircles {
    pub fn num_common_tangent(self) -> Option<u32> {
        use self::IntersectCircles::*;
        match self {
            SAME => None,
            INSIDE => Some(0),
            INSCRIBED => Some(1),
            INTERSECTED => Some(2),
            CIRCUMSCRIBED => Some(3),
            OUTSIDE => Some(4),
        }
    }
}

pub fn intersect_circles<T: Eps>(a: Circle<T>, b: Circle<T>) -> (IntersectCircles, Vec<Vector<T>>) {
    use self::IntersectCircles::*;

    let d = (a.center - b.center).abs();
    let x = ((a.radius.sq() + d.sq() - b.radius.sq()) / (T::from(2.0) * d * a.radius)).acos();
    let t = (b.center.1 - a.center.1).atan2(b.center.0 - a.center.0);

    if a.radius + b.radius == d {
        (CIRCUMSCRIBED, vec![a.center + Vector::polar(a.radius, t)])
    } else if (a.radius - b.radius).abs() == d {
        (INSCRIBED, vec![a.center + Vector::polar(a.radius, t)])
    } else if a.radius + b.radius > d && d > (a.radius - b.radius).abs() {
        (
            INTERSECTED,
            vec![
                a.center + Vector::polar(a.radius, t + x),
                a.center + Vector::polar(a.radius, t - x),
            ],
        )
    } else if a.radius + b.radius < d {
        (OUTSIDE, vec![])
    } else if (a.radius - b.radius).abs() > d {
        (INSIDE, vec![])
    } else {
        (SAME, vec![])
    }
}
