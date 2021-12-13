use crate::geom::*;

pub fn common_tangent_circles<T: Eps + std::fmt::Debug>(
    a: Circle<T>,
    b: Circle<T>,
) -> Vec<Vector<T>> {
    let cc = b.center - a.center;
    let d = cc.abs();
    let n = cc.unit();
    let r_diff = a.radius - b.radius;
    let r_sum = a.radius + b.radius;

    if r_sum == d {
        vec![
            a.center
                + (cc * r_diff + cc.normal() * (cc.abs_sq() - r_diff * r_diff).sqrt()) * a.radius
                    / cc.abs_sq(),
            a.center
                + (cc * r_diff - cc.normal() * (cc.abs_sq() - r_diff * r_diff).sqrt()) * a.radius
                    / cc.abs_sq(),
            a.center + n * a.radius,
        ]
    } else if r_sum < d {
        vec![
            a.center
                + (cc * r_diff + cc.normal() * (cc.abs_sq() - r_diff * r_diff).sqrt()) * a.radius
                    / cc.abs_sq(),
            a.center
                + (cc * r_diff - cc.normal() * (cc.abs_sq() - r_diff * r_diff).sqrt()) * a.radius
                    / cc.abs_sq(),
            a.center
                + (cc * r_sum + cc.normal() * (cc.abs_sq() - r_sum * r_sum).sqrt()) * a.radius
                    / cc.abs_sq(),
            a.center
                + (cc * r_sum - cc.normal() * (cc.abs_sq() - r_sum * r_sum).sqrt()) * a.radius
                    / cc.abs_sq(),
        ]
    } else if r_sum > d && d > (a.radius - b.radius).abs() {
        vec![
            a.center
                + (cc * r_diff + cc.normal() * (cc.abs_sq() - r_diff * r_diff).sqrt()) * a.radius
                    / cc.abs_sq(),
            a.center
                + (cc * r_diff - cc.normal() * (cc.abs_sq() - r_diff * r_diff).sqrt()) * a.radius
                    / cc.abs_sq(),
        ]
    } else if (a.radius - b.radius).abs() == d {
        vec![if a.radius > b.radius {
            a.center + n * a.radius
        } else {
            b.center - n * b.radius
        }]
    } else {
        vec![]
    }
}
