use space::{point2, Mat33, Point2};

pub fn arc(l: Mat33, a: f64, b: f64) -> Arc {
    Arc::new(l, a, b)
}

#[derive(Debug, Clone)]
pub struct Arc {
    pub l: Mat33,
    pub a: f64,
    pub b: f64,
}
impl Arc {
    pub fn new(l: Mat33, a: f64, b: f64) -> Self {
        Self { l, a, b }
    }

    pub fn eval(&self, u: f64) -> Point2 {
        let u = u * 2.0 * std::f64::consts::PI;
        //self.l.o + (self.a * u.cos() * self.l.x) + (self.b * u.sin() * self.l.y)
        let pt = point2(self.a * u.cos(), self.b * u.sin());

        pt.transform(&self.l)
    }
}

#[cfg(test)]
mod tests {
    use space::{deg, vec2};

    use super::*;

    #[test]
    fn arc_test() {
        let arc = arc(
            Mat33::rotation(deg(90.0)) * Mat33::translation(vec2(3.0, 3.0)),
            2.0,
            1.0,
        );

        println!("arc {:#?}", arc);

        let samples = 100;
        for i in 0..=samples {
            let u = i as f64 / samples as f64;

            println!("{}", arc.eval(u));
        }
    }
}
