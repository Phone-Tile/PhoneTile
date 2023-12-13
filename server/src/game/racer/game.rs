use super::bezier::*;
use super::vehicle::*;

#[cfg(debug_assertions)]
use plotters::prelude::*;
#[cfg(debug_assertions)]
use tqdm::tqdm;

const ACC_RATE: f64 = 25.;
const DECC_RATE: f64 = -1.2;
const SPEED_EXCESS: f64 = 0.3;
const FRICTION: f64 = 2.;
const DT: f64 = 1. / 60. * 1.;

/// The game structure. `Game.map` is the continuous sequence of bezier curves forming the curcuit and `Game.cars` is the list of cars present on the circuit.
pub struct Game {
    map: Vec<Bezier>,
    cars: Vec<Vehicle>,
}

impl Game {
    /// Create a new game structure. If the map parameter is an empty list, the map is randomly created.
    pub fn new(
        mut map: Vec<Bezier>,
        n_cars: usize,
        dimensions: &Vec<(f64, f64)>,
    ) -> Result<Self, std::io::Error> {
        let mut cars = Vec::new();
        for i in 0..n_cars {
            cars.push(Vehicle::new(0, i));
        }
        if map.is_empty() {
            let io_map = Game::get_io_map(dimensions)?;
            map = Bezier::random_map(dimensions, io_map);
        }
        // assert!(
        //     map[0].get_points().0 == map[map.len() - 1].get_points().3,
        //     "The given circuit is not closed. Please make sure the first and last point coincide."
        // );
        Ok(Game { map, cars })
    }

    /// Generate the points at a third of the minimal height of two consecutive phones to build the Bezier curves. The circuit is build anticlockwise.
    ///
    ///    |¯¯¯¯¯¯¯|         |¯¯¯¯¯¯¯|
    ///    |       ||¯¯¯¯¯¯¯||       |
    ///    |     i#||#o   i#||#o     |
    ///    |     o#||#i   o#||#i     |
    ///    |       ||___O___||       |
    ///    |___O___|         |___O___|
    ///
    /// Choose the '#' points. The bottom left corner of the frame is (0.,0.), the top right is (max(height), sum(width)).
    /// #i goes to inputs and #o to outputs. It represents the input and output points for the future Bezier curve.
    /// Recall that I chose to travel anticlockwise, elsewise invert #i with #o points.
    /// A #o point should be followed by a #i point, followed then by a #o point.
    /// Separate each of these points with a width of 2*eps to smooth the Bezier curves.
    /// `curr_weight` is expected to be equal to the sum of the weights up to the current phone (not included).
    /// At each phone, we add the connextion between the points #i' and #o' AND the connexion with the previous (#i,#o) pair (i.e. connect #o to #i').
    ///
    pub fn get_io_map(dimensions: &Vec<(f64, f64)>) -> Result<Data, std::io::Error> {
        if dimensions.len() < 2 {
            panic!("There must be at least two phones.")
        }
        let (_total_width, total_height) =
            dimensions
                .iter()
                .fold((0., 0.), |(sum_width, max_height), &new_size| {
                    (sum_width + new_size.0, f64::max(max_height, new_size.1))
                });
        let mut min_heights = vec![0.; dimensions.len()];
        let mut old_height = 0.;
        dimensions.iter().enumerate().for_each(|(i, &new_size)| {
            min_heights[i] = f64::min(old_height, new_size.1);
            old_height = new_size.1;
        });
        let mut return_side = Vec::new();
        let eps = 5.;
        let mut curr_width = 0.;
        // io_points[i] = (input_point, output_point)
        let mut io_points = Vec::new();
        for (i, &curr_height) in min_heights.iter().enumerate() {
            if i == 0 {
                let min_height = min_heights[1];
                let offset = (total_height - min_heights[1]) / 2.;
                io_points.push((
                    (dimensions[0].0, 2. * min_height / 3. + offset),
                    (dimensions[0].0, min_height / 3. + offset),
                    i,
                ));
            } else if i == min_heights.len() - 1 {
                let offset = (total_height - curr_height) / 2.;
                // last phone with [bottom, middle, top] segments.
                io_points.push((
                    (curr_width, curr_height / 3. + offset),
                    (curr_width, 2. * curr_height / 3. + offset),
                    i,
                ));
            } else {
                let offset_left = (total_height - curr_height) / 2.;
                let offset_right = (total_height - min_heights[i + 1]) / 2.;
                // bottom segment
                io_points.push((
                    (curr_width, curr_height / 3. + offset_left),
                    (
                        curr_width + dimensions[i].0,
                        min_heights[i + 1] / 3. + offset_right,
                    ),
                    i,
                ));
                // top segment
                return_side.push((
                    (
                        curr_width + dimensions[i].0,
                        2. * min_heights[i + 1] / 3. + offset_right,
                    ),
                    (curr_width, 2. * curr_height / 3. + offset_left),
                    i,
                ));
            }
            curr_width += dimensions[i].0;
        }
        return_side.reverse();
        io_points.extend(return_side);
        Ok(io_points)
    }

    pub fn get_map(&self) -> &Vec<Bezier> {
        &self.map
    }

    pub fn get_cars(&self) -> &Vec<Vehicle> {
        &self.cars
    }

    pub fn get_car_dir(&self, car : &Vehicle) -> (f64, f64){
        self.map[car.curve_index].compute_grad(car.t).into_tuple()
    }

    pub fn get_cars_pos(&self) -> Vec<(f64, f64)> {
        self.cars
            .iter()
            .map(|c| self.map[c.curve_index].compute_curve(c.t).into_tuple())
            .collect()
    }

    fn leave_road(&mut self, car_idx: usize, direction: Point) {
        // Set the fact that the car will leave the road
        let pos = self.get_pos(car_idx);
        let normalized_point = direction / direction.l2_norm() * self.cars[car_idx].speed;
        self.cars[car_idx].is_leaving = Some(pos + normalized_point);
    }

    pub fn update_position(&mut self, car_idx: usize, accelerate: bool) {
        // Random bullshit, GO! v(t+dt) = (a-f*v(t))*dt
        self.cars[car_idx].speed += ((if accelerate { ACC_RATE } else { 0. })
            - FRICTION * self.cars[car_idx].speed /* self.cars[car_idx].speed*/)
            * DT;

        let mut new_t = self.cars[car_idx].t;
        let mut new_curve = self.cars[car_idx].curve_index;

        for _ in 0..100 {
            let grad = self.map[new_curve].compute_grad(new_t);
            // The distance traveled will be total_distance/||grad|| in order to keep a constant speed accross the curve
            new_t += self.cars[car_idx].speed / 100. / grad.l2_norm();
            new_curve = (new_curve + ((new_t > 1.) as usize)) % self.map.len();

            if new_t > 1. {
                new_t = 0.;
            }
        }
        self.cars[car_idx].curve_index = new_curve;
        self.cars[car_idx].t = new_t;
    }

    fn get_pos(&mut self, car_idx: usize) -> Point {
        if let Some(dir) = self.cars[car_idx].is_leaving {
            self.cars[car_idx].is_leaving = None;
            self.cars[car_idx].speed = 0.;
            dir
        } else {
            self.map[self.cars[car_idx].curve_index].compute_curve(self.cars[car_idx].t)
        }
    }
}

#[cfg(debug_assertions)]
impl Game {
    #[allow(unused)]
    pub fn animate(&mut self, name: &str, iter: usize) -> Result<(), Box<dyn std::error::Error>> {
        let root = BitMapBackend::gif(name, (1024, 768), 1_000)?.into_drawing_area();

        for i in tqdm(0..iter) {
            root.fill(&WHITE)?;

            let mut chart = ChartBuilder::on(&root)
                .caption(format!("Game (n_iter = {i})"), ("sans-serif", 50))
                .build_cartesian_2d(-0.01f64..1.1f64, -0.1f64..1.1f64)?;

            for curve in self.map.iter() {
                chart.draw_series(LineSeries::new(curve.approx_points(100), RED))?;
            }

            let cars = self.cars.clone();
            for (idx, car) in cars.iter().enumerate() {
                chart
                    .draw_series([Circle::new(self.get_pos(car.car_idx).into_tuple(), 5, BLUE)])?
                    .label("Car");
                self.update_position(idx, true);
            }
            root.present()?;
        }

        println!("Result has been saved to {name}");

        Ok(())
    }

    #[allow(unused)]
    pub fn plot_game(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut car_pos = Vec::new();
        //println!("Accelerating");
        for _ in 0..70 {
            self.update_position(0, true);
            car_pos.push(self.get_pos(0));
            //println!("{:?}", self.get_pos(0));
        }
        //println!("Breaking");
        for _ in 0..10 {
            self.update_position(0, false);
            car_pos.push(self.get_pos(0));
            //println!("{:?}", self.get_pos(0));
        }
        //println!("Accelerating");
        for _ in 0..50 {
            self.update_position(0, true);
            car_pos.push(self.get_pos(0));
            //println!("{:?}", self.get_pos(0));
        }
        //println!("Breaking");
        for _ in 0..20 {
            self.update_position(0, false);
            car_pos.push(self.get_pos(0));
            //println!("{:?}", self.get_pos(0));
        }

        let root = BitMapBackend::new(name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption("Game", ("sans-serif", 50))
            .build_cartesian_2d(-0.01f64..1.3f64, -0.1f64..1.1f64)?;

        chart
            .draw_series(
                car_pos
                    .iter()
                    .map(|&point| Circle::new(point.into_tuple(), 4, BLUE)),
            )?
            .label("Car");

        chart
            .draw_series([
                Circle::new((car_pos[69].0, car_pos[69].1 + 0.05), 4, BLACK),
                Circle::new((car_pos[79].0, car_pos[79].1 + 0.05), 4, BLACK),
                Circle::new((car_pos[129].0, car_pos[129].1 + 0.05), 4, BLACK),
            ])?
            .label("Car");

        root.present()?;
        println!("Result has been saved to {name}");
        Ok(())
    }

    #[allow(unused)]
    pub fn plot_map(
        &self,
        name: &str,
        dimensions: &[(f64, f64)],
    ) -> Result<(), Box<dyn std::error::Error>> {
        let (total_width, total_height) =
            dimensions
                .iter()
                .fold((0., 0.), |(sum_width, max_height), &new_size| {
                    (sum_width + new_size.0, f64::max(max_height, new_size.1))
                });

        let root = BitMapBackend::new(name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption("Game map", ("sans-serif", 50))
            .set_label_area_size(LabelAreaPosition::Left, 40)
            .set_label_area_size(LabelAreaPosition::Bottom, 40)
            .build_cartesian_2d(-0.01f64..total_width + 0.1, -0.1f64..total_height + 0.1)?;

        chart.configure_mesh().draw()?;

        for curve in self.map.iter() {
            chart.draw_series(LineSeries::new(curve.approx_points(100), RED))?;
            let (p1, p2, p3, p4) = curve.get_points();
            chart
                .draw_series([
                    Circle::new(p1.into_tuple(), 4, BLUE),
                    Circle::new(p2.into_tuple(), 4, GREEN),
                    Circle::new(p3.into_tuple(), 4, GREEN),
                    Circle::new(p4.into_tuple(), 4, BLUE),
                ])?
                .label("Car");
            let dep1 = vec![p1.into_tuple(), p2.into_tuple()];
            let dep2 = vec![p3.into_tuple(), p4.into_tuple()];
            chart.draw_series(LineSeries::new(dep1.into_iter(), YELLOW))?;
            chart.draw_series(LineSeries::new(dep2.into_iter(), YELLOW))?;
        }

        let mut curr_width = 0.;
        let offset: Vec<f64> = dimensions
            .iter()
            .map(|(width, height)| (total_height - height) / 2.)
            .collect();
        for (i, (width, height)) in dimensions.iter().enumerate() {
            let value = vec![
                (curr_width, offset[i]),
                (curr_width + width, offset[i]),
                (curr_width + width, offset[i] + height),
                (curr_width, offset[i] + height),
                (curr_width, offset[i]),
            ];
            chart.draw_series(LineSeries::new(value.into_iter(), BLACK))?;
            curr_width += width;
        }

        root.present()?;
        println!("Map has been saved to {name}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const EPSILON: f64 = f64::EPSILON * 10.;
    #[test]
    fn test_io_map() {
        // Can generate the points as required
        let phone_size = vec![(1000., 1000.), (300., 300.), (1000., 1000.)];
        let io_map = Game::get_io_map(&phone_size).unwrap();
        assert_eq!(io_map.len(), 2 * (phone_size.len() - 1));
        assert!(io_map
            .iter()
            .zip(vec![
                ((1000., 550.), (1000., 450.)),
                ((1000., 450.), (1300., 450.)),
                ((1300., 450.), (1300., 550.)),
                ((1300., 550.), (1000., 550.)),
            ])
            .all(|(&(f1, f2, _), (f3, f4))| (f1.0 - f3.0) < EPSILON
                && (f1.1 - f3.1) < EPSILON
                && (f2.1 - f4.1) < EPSILON
                && (f2.0 - f4.0) < EPSILON));
    }

    #[test]
    fn test_random_map_bezier() {
        let phone_size = vec![(1000., 1000.), (300., 300.), (1000., 1000.)];
        let game = Game::new(Vec::new(), 1, &phone_size).unwrap();
        let map = game.get_map();
        assert!(map
            .iter()
            .enumerate()
            .all(|(i, curve)| { curve.get_points().3 == map[(i + 1) % map.len()].get_points().0 }));
    }
    #[test]
    #[should_panic]
    fn test_panic_one_phone() {
        let phone_size = vec![(1., 1.)];
        Game::new(Vec::new(), 1, &phone_size).unwrap();
    }
}
