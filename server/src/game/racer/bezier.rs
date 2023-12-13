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
impl<T1, T2> From<(T1, T2)> for Point
where
    f64: From<T1>,
    f64: From<T2>,
    T1: Copy,
    T2: Copy,
    T2: std::fmt::Debug,
    T1: std::fmt::Debug,
{
    fn from(tpl: (T1, T2)) -> Point {
        Point(f64::from(tpl.0), f64::from(tpl.1))
    }
}

/// Even though Vector and Points are the same in practice, we tend to see them as two separate things.
/// We will usef the following function with the type annotation of a Vector only.
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
    fn pseudo_normalised(&self) -> Vector {
        let norm = self.l2_norm();
        *self / (0.1 + norm)
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
impl std::ops::Div<f64> for Point {
    type Output = Point;
    #[inline]
    fn div(self, other: f64) -> Self {
        Point(self.0 / other, self.1 / other)
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
    #[inline]
    fn add_assign(&mut self, rhs: Point) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}
impl std::ops::DivAssign<f64> for Point {
    #[inline]
    fn div_assign(&mut self, rhs: f64) {
        self.0 /= rhs;
        self.1 /= rhs;
    }
}
impl std::ops::MulAssign<f64> for Point {
    #[inline]
    fn mul_assign(&mut self, rhs: f64) {
        self.0 *= rhs;
        self.1 *= rhs;
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

pub type Data = Vec<((f64, f64), (f64, f64), usize)>;

/// The points of the gradient of a Bezier curve. `Gradient(A,B,C)` represents the function At^2 + Bt + C that can be computed via the method `gradient.at_time(t)`
#[derive(Clone, Debug, Default)]
struct Gradient(Point, Point, Point);
impl Gradient {
    #[inline]
    fn at_time(&self, t: f64) -> Point {
        self.0 * t * t + self.1 * t + self.2
    }
}

/// The Bezier curve is {(1-t)^3P1 + 3t(1-t)^2P2 + 3t^2(2-t)P3 + t^3P4 | t in [0,1]}. This value can be computed using the following method: `bezier.compute_curve()`.
/// It can also represent lower rank Bezier curves by choosing parameters points P2 and P3 so that they points to where the parameter point would be in a rank 2 and to the other reference point for a rank 1.
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
    /// In the following example, we expect io_points to be equal to [(i1,o1), (o1,i2), (i2,o2), (o2,i3), (i3, o3), (o3,i4), (i4, o4), (o4, i1)].
    ///
    ///    |¯¯¯¯¯¯¯|         |¯¯¯¯¯¯¯|
    ///    |       ||¯¯¯¯¯¯¯||       |
    ///    |    i1#||#o4 i4#||#o3    |
    ///    |    o1#||#i2 o2#||#i3    |
    ///    |       ||___O___||       |
    ///    |___O___|         |___O___|
    ///
    /// Transform this list of pairs into a list of Bezier curves with starting and ending points corresponding to each pair.
    ///
    pub fn random_map_pas_si_random(dimensions: &Vec<(f64, f64)>, io_points: Data) -> Vec<Self> {
        let mut map = vec![];
        let mut width = 0.;
        map.push(Bezier::new(
            Point::from((dimensions[0].0, dimensions[0].1 / 3. * 2.)),
            Point::from((0, dimensions[0].1 / 3. * 2.)),
            Point::from((0, dimensions[0].1 / 3.)),
            Point::from((dimensions[0].0, dimensions[0].1 / 3.)),
        ));

        for i in 0..dimensions.len() - 2 {
            width += dimensions[i].0;

            map.push(Bezier::new(
                Point::from((width, dimensions[i + 1].1 / 3.)),
                Point::from(((width + dimensions[i + 1].0) / 2., dimensions[i + 1].1 / 3.)),
                Point::from(((width + dimensions[i + 1].0) / 2., dimensions[i + 1].1 / 3.)),
                Point::from((width + dimensions[i + 1].0, dimensions[i + 1].1 / 3.)),
            ));
        }
        width += dimensions[dimensions.len() - 2].0;
        map.push(Bezier::new(
            Point::from((width, dimensions[dimensions.len() - 1].1 / 3.)),
            Point::from((
                width + dimensions[dimensions.len() - 1].0,
                dimensions[dimensions.len() - 1].1 / 3.,
            )),
            Point::from((
                width + dimensions[dimensions.len() - 1].0,
                dimensions[dimensions.len() - 1].1 / 3. * 2.,
            )),
            Point::from((width, dimensions[dimensions.len() - 1].1 / 3. * 2.)),
        ));

        for i in dimensions.len() - 1..1 {
            width -= dimensions[i].0;

            map.push(Bezier::new(
                Point::from((width + dimensions[i].0, dimensions[i].1 / 3. * 2.)),
                Point::from(((width + dimensions[i].0) / 2., dimensions[i].1 / 3. * 2.)),
                Point::from(((width + dimensions[i].0) / 2., dimensions[i].1 / 3. * 2.)),
                Point::from((width, dimensions[i].1 / 3. * 2.)),
            ));
        }
        map
    }

    pub fn random_map2(dimensions: &Vec<(f64, f64)>, io_points: Data) -> Vec<Self> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(41);
        let mut map = vec![];
        let mut width = 0.;
        let mut control_screen_start_bot = rng.gen::<(f64, f64)>();
        let mut control_screen_start_top = rng.gen::<(f64, f64)>();
        // spend more time on the first screen
        control_screen_start_bot.0 *= 0.5;
        control_screen_start_top.0 *= 0.5;
        // Transform into real size
        control_screen_start_bot.0 *= dimensions[0].0;
        control_screen_start_top.0 *= dimensions[0].0;
        control_screen_start_bot.1 *= dimensions[0].1;
        control_screen_start_top.1 *= dimensions[0].1;

        map.push(Bezier::new(
            Point::from((dimensions[0].0, dimensions[0].1 / 3. * 2.)),
            Point::from(control_screen_start_top),
            Point::from(control_screen_start_bot),
            Point::from((dimensions[0].0, dimensions[0].1 / 3.)),
        ));

        // Do the same for every intermediate phones remembering the previous random control point and taking the symmetry
        // as the new control point in order to keep a infinitely derivable curves
        let mut previous_point = control_screen_start_bot;
        for i in 0..dimensions.len() - 2 {
            width += dimensions[i].0;
            let mut next_control_point = rng.gen::<(f64, f64)>();
            next_control_point.0 *= dimensions[i + 1].0;
            next_control_point.1 *= dimensions[i + 1].1;

            // Avoid collisions by choosing each control point in a separated area (at the bottom for the moment)
            next_control_point.1 *= 0.5;

            map.push(Bezier::new(
                Point::from((width, dimensions[i + 1].1 / 3.)),
                Point::from(previous_point)
                    .symmetry(Point::from((width, dimensions[i + 1].1 / 3.))),
                Point::from(next_control_point),
                Point::from((width + dimensions[i + 1].0, dimensions[i + 1].1 / 3.)),
            ));
            previous_point = next_control_point;
        }
        width += dimensions[dimensions.len() - 2].0;

        let mut next_control_point = rng.gen::<(f64, f64)>();
        next_control_point.0 *= dimensions[dimensions.len() - 1].0;
        next_control_point.1 *= dimensions[dimensions.len() - 1].1;

        // change it to make the circuit spend more time on the last phone
        previous_point.0 = 0.5 * (1. + previous_point.0);

        map.push(Bezier::new(
            Point::from((width, dimensions[dimensions.len() - 1].1 / 3.)),
            Point::from(previous_point).symmetry(Point::from((
                width,
                dimensions[dimensions.len() - 1].1 / 3.,
            ))),
            Point::from(next_control_point),
            Point::from((width, dimensions[dimensions.len() - 1].1 / 3. * 2.)),
        ));
        previous_point = next_control_point;

        for i in dimensions.len() - 1..1 {
            width -= dimensions[i].0;
            width += dimensions[i].0;
            let mut next_control_point = rng.gen::<(f64, f64)>();
            next_control_point.0 *= dimensions[i + 1].0;
            next_control_point.1 *= dimensions[i + 1].1;

            // Avoid collisions by choosing each control point in a separated area (at the top now)
            next_control_point.1 = 0.5 * (next_control_point.1 + 1.);

            map.push(Bezier::new(
                Point::from((width + dimensions[i].0, dimensions[i].1 / 3. * 2.)),
                Point::from(previous_point).symmetry(Point::from((
                    width + dimensions[i].0,
                    dimensions[i].1 / 3. * 2.,
                ))),
                Point::from(next_control_point),
                Point::from((width, dimensions[i].1 / 3. * 2.)),
            ));
            previous_point = next_control_point;
        }
        // Join the first and last curves together with smoothness
        map.push(Bezier::new(
            Point::from((width + dimensions[1].0, dimensions[1].1 / 3. * 2.)),
            Point::from(previous_point).symmetry(Point::from((
                width + dimensions[1].0,
                dimensions[1].1 / 3. * 2.,
            ))),
            Point::from(control_screen_start_top).symmetry(Point::from((
                width + dimensions[0].0,
                dimensions[0].1 / 3. * 2.,
            ))),
            Point::from((width, dimensions[0].1 / 3. * 2.)),
        ));
        map
    }

    pub fn random_map(dimensions: &Vec<(f64, f64)>, io_points: Data) -> Vec<Self> {
        let mut rng = rand::rngs::StdRng::seed_from_u64(41);

        let (_total_width, total_height) =
            dimensions
                .iter()
                .fold((0., 0.), |(sum_width, max_height), &new_size| {
                    (sum_width + new_size.0, f64::max(max_height, new_size.1))
                });

        let len = dimensions.len() - 1;
        let eps = 10.;

        let mut opt_bez_curves = vec![None; io_points.len()];
        let mut widths = Vec::new();
        dimensions.iter().fold(0., |old_width, new_size| {
            widths.push(old_width);
            old_width + new_size.0
        });

        let mut control_last_right = rng.gen::<(f64, f64)>();
        for (i, (input, output, phone_idx)) in io_points.iter().enumerate() {
            let mut control_1 = rng.gen::<(f64, f64)>();
            let mut control_2 = rng.gen::<(f64, f64)>();
            if *phone_idx == 0 {
                // Make the car spend more time on first phone
                control_1 = control_last_right;
                control_2.0 *= 0.5;
            } else if *phone_idx == len {
                // Make the car spend more time on last phone x = (b-a) + x*a/b
                control_1.0 = 0.5 * (control_1.0 + 1.);
                control_2.0 = 0.5 * (control_2.0 + 1.);
            } else if i == io_points.len() - 1 {
                // no random for the last control point in order to loop
                control_2 = control_last_right;
            }
            // Add offset to control points
            let offset = (total_height - dimensions[*phone_idx].1) / 2.;
            let mut control_1_1;
            let mut control_1_2;
            let mut control_2_1;
            let mut control_2_2;
            if *phone_idx == 0 {
                let offset_1 = (total_height - dimensions[1].1) / 2.;
                control_1_1 = control_1.0 * dimensions[1].0 + widths[1] + eps;
                control_1_2 = control_1.1 * dimensions[1].1 / 2. + offset_1 + dimensions[1].1 / 2.;
                (control_1_1, control_1_2) = Point::from((control_1_1, control_1_2))
                    .symmetry(Point::from(*input))
                    .into_tuple()
            } else if input.1 > dimensions[*phone_idx].1 / 2. + offset {
                control_1_1 = control_1.0 * dimensions[*phone_idx].0 + widths[*phone_idx] + eps;
                control_1_2 = control_1.1 * dimensions[*phone_idx].1 / 2.
                    + offset
                    + dimensions[*phone_idx].1 / 2.;
            } else {
                control_1_1 = control_1.0 * (dimensions[*phone_idx].0) + widths[*phone_idx] + eps;
                control_1_2 = control_1.1 * dimensions[*phone_idx].1 / 2. + offset;
            }
            if output.1 > dimensions[*phone_idx].1 / 2. + offset {
                control_2_1 = control_2.0 * (dimensions[*phone_idx].0) + widths[*phone_idx] + eps;
                control_2_2 = control_2.1 * dimensions[*phone_idx].1 / 2.
                    + offset
                    + dimensions[*phone_idx].1 / 2.;
            } else if *phone_idx == 0 {
                control_2_1 = control_2.0 * (dimensions[0].0);
                control_2_2 = control_2.1 * dimensions[*phone_idx].1 / 2. + offset;
            } else {
                control_2_1 = control_2.0 * (dimensions[*phone_idx].0) + widths[*phone_idx] + eps;
                control_2_2 = control_2.1 * dimensions[*phone_idx].1 / 2. + offset;
            }

            opt_bez_curves[i] = Some(Bezier::new(
                Point::from(*input),
                Point::from((control_1_1, control_1_2)),
                Point::from((control_2_1, control_2_2)),
                Point::from(*output),
            ));
        }
        let mut bezier_curves = Vec::new();
        opt_bez_curves.iter().for_each(|opt_curve| {
            bezier_curves.push(opt_curve.as_ref().unwrap().clone());
        });
        bezier_curves
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

    /// Returns the reference points of the curve [Start, Control1, Control2, End]
    pub fn get_points(&self) -> (Point, Point, Point, Point) {
        (self.p1, self.p2, self.p3, self.p4)
    }

    pub fn compute_grad(&self, t: f64) -> Vector {
        self.grad.at_time(t)
    }
    #[allow(unused)]
    pub fn get_fun(&self) -> impl Fn(f64) -> Point + '_ {
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

    pub fn approx_points(&self, num_points: usize) -> impl Iterator<Item = (f64, f64)> + '_ {
        (0..=num_points)
            .map(|x| x as f64 / 100.0)
            .map(|x| self.compute_curve(x).into_tuple())
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
        assert!(bez.approx_points(100).all(|(x, y)| x == y));
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
