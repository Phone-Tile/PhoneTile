use rand::Rng;
use rand::SeedableRng;

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct Point(pub f64, pub f64);
impl Point {
    pub fn into_tuple(self) -> (f64, f64) {
        (self.0, self.1)
    }
    pub fn symmetry(&self, rhs: Point) -> Point {
        2 * *self - rhs
    }
}
impl<T> From<T> for Point
where
    (f64, f64): From<T>,
    T: Copy,
{
    fn from(tpl: T) -> Point {
        Point(<(f64, f64)>::from(tpl).0, <(f64, f64)>::from(tpl).0)
    }
}

/// Even though Vector and Points are the same in practice, we tend to see them as two separate things.
/// We will use the following function with the type annotation of a Vector only.
pub type Vector = Point;
impl Vector {
    #[inline]
    pub fn l2_norm(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1).sqrt()
    }
    fn normalize(&mut self) {
        let norm = self.l2_norm();
        *self /= norm;
    }
    pub fn scalar(&self, rhs: Vector) -> f64 {
        self.0 * rhs.0 + self.1 * rhs.1
    }
    pub fn normal(&self) -> Vector {
        let mut normal = Point(-self.1, self.0);
        normal.normalize();
        normal
    }
}

impl<T> std::ops::Mul<T> for Point
where
    f64: From<T>,
    T: Copy,
{
    type Output = Point;
    #[inline]
    fn mul(self, other: T) -> Self {
        Point(self.0 * f64::from(other), self.1 * f64::from(other))
    }
}
impl std::ops::Add<Point> for Point {
    type Output = Point;
    #[inline]
    fn add(self, other: Point) -> Self {
        Point(self.0 + other.0, self.1 + other.1)
    }
}
impl std::ops::Sub<Point> for Point {
    type Output = Point;
    #[inline]
    fn sub(self, other: Point) -> Self {
        Point(self.0 - other.0, self.1 - other.1)
    }
}
impl std::ops::AddAssign for Point {
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl std::ops::DivAssign<f64> for Point {
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}

macro_rules! left_scalar_mul_impl(
    ($($T: ty),* $(,)*) => {$(
        impl std::ops::Mul<Point> for $T {
            type Output = Point;

            #[inline]
            fn mul(self, rhs: Point) -> Self::Output {
                rhs * self
            }
        }
    )*}
);

left_scalar_mul_impl!(u8, u16, u32, i8, i16, i32, f32, f64);

/// The points of the gradient of a Bezier curve. `Gradient(A,B,C)` represents the function At^2 + Bt + C that can be computed via the method `gradient.at_time(t)`
#[derive(Clone, Debug, Default)]
struct Gradient(Point, Point, Point);
impl Gradient {
    fn at_time(&self, t: f64) -> Point {
        self.0 * t * t + self.1 * t + self.2
    }
}

/// The Bezier curve is {(1-t)^3P1 + 3t(1-t)^2P2 + 3t^2(2-t)P3 + t^3P4 | t in [0,1]}. This value can be computed using the following method: `bezier.compute_curve()`.
/// It can also represent lower rank Bezier curves by choosing parameters points P2 and P3 so that they points to where the parameter point would be in a rank 2 and to the other reference point for a rank 1.
///
#[derive(Debug, Clone, Default)]
pub struct Bezier {
    p1: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    grad: Gradient,
    length: f64,
}
impl Bezier {
    pub fn new(p1: Point, p2: Point, p3: Point, p4: Point) -> Self {
        assert!(p1 != p2, "p1!=p2 for differentiability on 0!");
        let grad = Gradient(
            3 * (p4 - 3 * p3 + 3 * p2 - p1),
            6 * p3 - 12 * p2 + 6 * p1,
            3 * (p2 - p1),
        );
        let mut bez = Bezier {
            p1,
            p2,
            p3,
            p4,
            grad,
            length: 0.,
        };
        bez.init_length(100);
        bez
    }
    pub fn new_tuple(p1: (f64, f64), p2: (f64, f64), p3: (f64, f64), p4: (f64, f64)) -> Self {
        Bezier::new(
            Point::from(p1),
            Point::from(p2),
            Point::from(p3),
            Point::from(p4),
        )
    }
    #[allow(unused)]
    pub fn length(&self) -> f64 {
        self.length
    }

    #[allow(unused)]
    /// io_points[i] represents the two points to link together with a Bezier curve.
    /// In the following example, we expect io_points to be equal to [(i1,o1), (o1,i2), (i2,o2), (i4, o4), (o4, i1), (o2,i3), (i3, o3), (o3,i4)].
    ///
    ///    |¯¯¯¯¯¯¯|         |¯¯¯¯¯¯¯|
    ///    |       ||¯¯¯¯¯¯¯||       |
    ///    |    i1#||#o4 i4#||#o3    |
    ///    |    o1#||#i2 o2#||#i3    |
    ///    |       ||___O___||       |
    ///    |___O___|         |___O___|
    ///
    /// We want to link these points smoothly by using the temp curves (o_k,i_k+1) as a symmetry of the other control points.
    ///
    pub fn random_map(
        dimensions: Vec<(f64, f64)>,
        io_points: Vec<((f64, f64), (f64, f64))>,
    ) -> Vec<Self> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let mut bezier_curves = Vec::new();

        let (_total_width, total_height) =
            dimensions
                .iter()
                .fold((0., 0.), |(sum_width, max_height), &new_size| {
                    (sum_width + new_size.0, f64::max(max_height, new_size.1))
                });

        let mut i = 0;
        let mut phone_num = 0;
        println!("{:?}", io_points);
        while i < io_points.len() {
            debug_assert!(phone_num < dimensions.len());
            let (input, output) = io_points[i];
            if input.0 == output.0 {
                println!("extrema");
                // one of the extremal phone
                if input.0 < dimensions[0].0 {
                    // Left most phone
                    let mut control_1 = rng.gen::<(f64, f64)>();
                    let mut control_2 = rng.gen::<(f64, f64)>();
                    println!(
                        "{:?}",
                        (control_1.0 * dimensions[0].0, control_1.1 * dimensions[0].1)
                    );
                    bezier_curves.push(Bezier::new_tuple(
                        input,
                        (control_1.0 * dimensions[0].0, control_1.1 * dimensions[0].1),
                        (control_2.0 * dimensions[0].0, control_2.1 * dimensions[0].1),
                        output,
                    ));
                } else {
                    // Right most phone
                    let mut control_1 = rng.gen::<(f64, f64)>();
                    let mut control_2 = rng.gen::<(f64, f64)>();
                    let len = dimensions.len() - 1;
                    let offset = (total_height - dimensions[len].0) / 2.;
                    bezier_curves.push(Bezier::new_tuple(
                        input,
                        (
                            control_1.0 * dimensions[len].0 + input.0,
                            control_1.1 * dimensions[len].1 + offset,
                        ),
                        (
                            control_2.0 * dimensions[len].0 + input.0,
                            control_2.1 * dimensions[len].1 + offset,
                        ),
                        output,
                    ));
                }
            } else {
                println!("middle");
                // middle phone, expect to have [left link bottom, path bottom, path top, left link top]
                i += 1;
                let (li1, li2) = io_points[i];
                i += 1;
                let (li3, li4) = io_points[i];
                i += 1;
                let (i2, o2) = io_points[i];

                let mut control_1_top = rng.gen::<(f64, f64)>();
                let mut control_2_top = rng.gen::<(f64, f64)>();

                let mut control_1_bot = rng.gen::<(f64, f64)>();
                let mut control_2_bot = rng.gen::<(f64, f64)>();

                let offset = (total_height - dimensions[phone_num].0) / 2.;
                bezier_curves.push(Bezier::new_tuple(
                    li1,
                    (
                        control_1_bot.0 * dimensions[phone_num].0 + li1.0,
                        control_1_bot.1 * dimensions[phone_num].1 + offset,
                    ),
                    (
                        control_2_bot.0 * dimensions[phone_num].0 + li1.0,
                        control_2_bot.1 * dimensions[phone_num].1 + offset,
                    ),
                    li2,
                ));
                bezier_curves.push(Bezier::new_tuple(
                    li3,
                    (
                        control_1_top.0 * dimensions[phone_num].0 + li1.0,
                        control_1_top.1 * dimensions[phone_num].1 + offset,
                    ),
                    (
                        control_2_top.0 * dimensions[phone_num].0 + li1.0,
                        control_2_top.1 * dimensions[phone_num].1 + offset,
                    ),
                    li4,
                ));
            }
            i += 1;
            phone_num += 1;
        }
        i = 0;
        phone_num = 0;
        let mut link_right = Vec::new();
        let mut link_left = Vec::new();
        while i < io_points.len() {
            let (input, output) = io_points[i];
            if input.1 == output.1 {
                // temp line
                let curve_idx = (phone_num - 1) * 2 + 1;
                // weird but ok
                let in_control_point_bot = bezier_curves[curve_idx - 1].get_points().2;
                let out_control_point_bot = bezier_curves[curve_idx].get_points().1;
                let in_control_point = bezier_curves[curve_idx + 1].get_points().2;
                let out_control_point = if curve_idx == 1 {
                    // second phone, there is only one curve before
                    bezier_curves[curve_idx - 1].get_points().1
                } else {
                    bezier_curves[curve_idx - 2].get_points().1
                };
                link_right.push(Bezier::new(
                    Point::from(input),
                    (Point::from(input).symmetry(in_control_point_bot)),
                    (Point::from(output).symmetry(out_control_point_bot)),
                    Point::from(output),
                ));
                i += 3;
                // second link curve for top left
                let (in2, out2) = io_points[i];
                link_left.push(Bezier::new(
                    Point::from(in2),
                    (Point::from(in2).symmetry(in_control_point)),
                    (Point::from(out2).symmetry(out_control_point)),
                    Point::from(out2),
                ));
            }
            i += 1;
            phone_num += 1;
        }
        i = 1;
        let mut bez_right = Vec::new();
        let mut bez_left = Vec::new();
        let mut last_bez = bezier_curves[0].clone();
        println!("whut {:?}", bezier_curves.len());
        while i < bezier_curves.len() {
            let points = bezier_curves[i].get_points();
            if points.0 .0 != points.3 .0 {
                // not first or last phone
                bez_right.push(bezier_curves[i].clone());
                i += 1;
                bez_left.push(bezier_curves[i].clone());
            } else {
                last_bez = bezier_curves[i].clone();
            }
            i += 1;
        }

        let mut out = vec![bezier_curves[0].clone()];
        for (i, b) in bez_left.iter().enumerate() {
            out.push(b.clone());
            out.push(link_right[i].clone());
        }
        out.push(last_bez.clone());
        for (i, b) in bez_right.iter().enumerate() {
            out.push(b.clone());
            out.push(link_left[i].clone());
        }
        out
    }

    fn init_length(&mut self, num_ref_points: usize) {
        // Take `num_ref_points` on the curve and sum the distances between them.
        self.length = (0..num_ref_points)
            .map(|i| i as f64 / (num_ref_points - 1) as f64)
            .fold((0.0, self.p1), |(length, prev_point), t| {
                let current_point = self.compute_curve(t);
                let step_length = (current_point - prev_point).l2_norm();
                (length + step_length, current_point)
            })
            .0;
    }

    #[allow(unused)]
    /// Returns the reference points of the curve [Start, Control1, Control2, End]
    pub fn get_points(&self) -> (Point, Point, Point, Point) {
        (self.p1, self.p2, self.p3, self.p4)
    }

    pub fn compute_grad(&self, t: f64) -> Vector {
        self.grad.at_time(t)
    }
    #[allow(unused)]
    pub fn get_fun<'a>(&'a self) -> impl Fn(f64) -> Point + 'a {
        |t: f64| {
            let u = 1. - t;
            let t2 = t * t;
            let u2 = u * u;
            u2 * u * self.p1 + 3. * t * u2 * self.p2 + 3. * t2 * u * self.p3 + t2 * t * self.p4
        }
    }

    pub fn compute_curve(&self, t: f64) -> Point {
        let u = 1. - t;
        let t2 = t * t;
        let u2 = u * u;
        u2 * u * self.p1 + 3. * t * u2 * self.p2 + 3. * t2 * u * self.p3 + t2 * t * self.p4
    }

    pub fn approx_points(&self, num_points: usize) -> Vec<(f64, f64)> {
        (0..=num_points)
            .map(|x| x as f64 / 100.0)
            .map(|x| self.compute_curve(x).into_tuple())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn linear_bezier() {
        let bez = Bezier::new(
            Point(0., 0.),
            Point(0.5, 0.5),
            Point(0.5, 0.5),
            Point(1., 1.),
        );
        // All points well placed
        assert!(bez.approx_points(100).iter().all(|(x, y)| x == y));
    }
    #[test]
    fn test_points() {
        let mut point1 = Point(1., 1.);
        // Symetry
        assert_eq!(point1.symmetry(Point(2., 2.)), Point(0., 0.));
        assert_eq!(point1.symmetry(Point(0., 0.)), Point(2., 2.));
        // operations on points
        let point2 = Point(0.1, 0.5);
        assert_eq!(point1 + point2, Point(1.1, 1.5));
        assert!(point1 * 2 == 2 * point1 && point1 * 2 == Point(2., 2.));
        point1 /= 2.;
        assert_eq!(point1, Point(0.5, 0.5));
        // Type transformation
        assert_eq!(point1.into_tuple(), (0.5, 0.5));
    }
    #[test]
    fn test_vector() {
        let v1: Vector = Point(0., 1.);
        // Normal computation is good
        assert_eq!(v1.normal(), Point(-1., 0.));
        let mut v2: Vector = Point(f64::sqrt(2.), f64::sqrt(2.));
        // The norm 2 works as expected
        assert_eq!(v2.l2_norm(), 2.);
        v2.normalize();
        // The normalization works well up to float precision
        assert!((v2 - Point(1. / f64::sqrt(2.), 1. / f64::sqrt(2.))).0.abs() < f64::EPSILON);
        assert!((v2 - Point(1. / f64::sqrt(2.), 1. / f64::sqrt(2.))).1.abs() < f64::EPSILON);
    }
}
