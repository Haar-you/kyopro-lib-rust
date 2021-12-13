use crate::geom::*;

pub fn tangent_circle<T: Eps>(c: Circle<T>, p: Vector<T>) -> Vec<Vector<T>> {
    let d = (p - c.center).abs();

    if d < c.radius {
        return vec![];
    }
    if d - c.radius == T::from(0.0) {
        return vec![p];
    }

    let a = (c.radius / d).acos();
    let t = (p.1 - c.center.1).atan2(p.0 - c.center.0);

    vec![
        c.center + Vector::polar(c.radius, t + a),
        c.center + Vector::polar(c.radius, t - a),
    ]
}
