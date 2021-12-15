//! 最小包含円
use crate::geom::{circumcircle::*, *};
use rand::seq::SliceRandom;

pub fn min_covering_circle<T: Eps>(mut ps: Vec<Vector<T>>) -> Option<Circle<T>> {
    let n = ps.len();
    match n {
        0 => None,
        1 => Some(Circle::new(ps[0], T::from(0.0))),
        _ => {
            let mut rng = rand::thread_rng();
            ps.shuffle(&mut rng);

            let make_2 = |p: Vector<T>, q: Vector<T>| {
                let c = (p + q) / T::from(2.0);
                Circle::new(c, (p - c).abs())
            };

            let check = |p: Vector<T>, c: Circle<T>| (c.center - p).abs() <= c.radius;

            let mut ret = make_2(ps[0], ps[1]);

            for i in 2..n {
                if check(ps[i], ret) {
                    continue;
                }

                ret = make_2(ps[0], ps[i]);

                for j in 1..i {
                    if check(ps[j], ret) {
                        continue;
                    }

                    ret = make_2(ps[i], ps[j]);

                    for k in 0..j {
                        if check(ps[k], ret) {
                            continue;
                        }

                        if i == j || j == k || k == i {
                            continue;
                        }

                        ret = circumcircle(ps[i], ps[j], ps[k]);
                    }
                }
            }

            Some(ret)
        }
    }
}
