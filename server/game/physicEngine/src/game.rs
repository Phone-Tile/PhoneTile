use crate::bezier::*;
use crate::vehicle::*;
use plotters::prelude::*;
use tqdm::tqdm;

const ACC_RATE: f64 = 1.;
const DECC_RATE: f64 = -1.2;
const SPEED_EXCESS: f64 = 0.3;
const FRICTION: f64 = 0.1;
const DT: f64 = 1. / 60. * 0.01;

/// The game structure. `Game.map` is the continuous sequence of bezier curves forming the curcuit and `Game.cars` is the list of cars present on the circuit.
pub struct Game {
    map: Vec<Bezier>,
    cars: Vec<Vehicle>,
}

impl Game {
    /// Create a new game structure. If the map parameter is an empty list, the map is randomly created.
    pub fn new(mut map: Vec<Bezier>, n_cars: usize) -> Self {
        let mut cars = Vec::new();
        for i in 0..n_cars {
            cars.push(Vehicle::new(0, i));
        }
        if map.is_empty() {
            let dimensions = vec![(0.3, 0.7), (0.5, 0.8), (0.4, 0.9)];
            let io_map = Game::get_io_map(&dimensions);
            map = Bezier::random_map(dimensions, io_map);
        }
        assert!(
            map[0].get_points().0 == map[map.len() - 1].get_points().3,
            "The given circuit is not closed. Please make sure the first and last point coincide."
        );
        Game { map, cars }
    }

    /// Generate the points at a third of the minimal height of two consecutive phones to build the Bezier curves. The circuit is build anticlockwise.
    fn get_io_map(dimensions: &Vec<(f64, f64)>) -> Vec<((f64, f64), (f64, f64))> {
        assert!(dimensions.len() > 1, "There must be at least two phones.");
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
        min_heights.remove(0);
        min_heights.remove(0);
        let eps = 1e-1;
        let mut curr_width = 0.;
        // io_points[i] = (input_point, output_point)
        let mut io_points = Vec::new();
        for (i, &curr_height) in min_heights.iter().enumerate() {
            //
            //    |¯¯¯¯¯¯¯|         |¯¯¯¯¯¯¯|
            //    |       ||¯¯¯¯¯¯¯||       |
            //    |     i#||#o   i#||#o     |
            //    |     o#||#i   o#||#i     |
            //    |       ||___O___||       |
            //    |___O___|         |___O___|
            //
            // Choose the '#' points. The bottom left corner of the frame is (0.,0.), the top right is (max(height), sum(width)).
            // #i goes to inputs and #o to outputs. It represents the input and output points for the future Bezier curve.
            // Recall that I chose to travel anticlockwise, elsewise invert #i with #o points.
            // A #o point should be followed by a #i point, followed then by a #o point.
            // Separate each of these points with a width of 2*eps to smooth the Bezier curves.
            // `curr_weight` is expected to be equal to the sum of the weights up to the current phone (not included).
            // At each phone, we add the connextion between the points #i' and #o' AND the connexion with the previous (#i,#o) pair (i.e. connect #o to #i').
            //
            if i == 0 {
                let min_height = f64::min(dimensions[0].1, dimensions[1].1);
                let offset = (total_height - min_height) / 2.;
                io_points.push((
                    (dimensions[0].0 - eps, 2. * min_height / 3. + offset),
                    (dimensions[0].0 - eps, min_height / 3. + offset),
                ));
            } else if i == min_heights.len() - 1 {
                let offset = (total_height - curr_height) / 2.;
                io_points.push((
                    (curr_width - eps, curr_height / 3. + offset),
                    (curr_width + eps, curr_height / 3. + offset),
                ));
                io_points.push((
                    (curr_width + eps, curr_height / 3. + offset),
                    (curr_width + eps, 2. * curr_height / 3. + offset),
                ));
                io_points.push((
                    (curr_width + eps, 2. * curr_height / 3. + offset),
                    (curr_width - eps, 2. * curr_height / 3. + offset),
                ));
            } else {
                let offset = (total_height - curr_height) / 2.;
                // bottom segment
                io_points.push((
                    (curr_width - eps, curr_height / 3. + offset),
                    (curr_width + eps, curr_height / 3. + offset),
                ));
                io_points.push((
                    (curr_width + eps, curr_height / 3. + offset),
                    (
                        curr_width + dimensions[i].0 - eps,
                        2. * min_heights[i + 1] / 3. + offset,
                    ),
                ));
                // top segment
                io_points.push((
                    (curr_width + eps, 2. * curr_height / 3. + offset),
                    (curr_width - eps, 2. * curr_height / 3. + offset),
                ));
                io_points.push((
                    (
                        curr_width + dimensions[i].0 - eps,
                        2. * curr_height / 3. + offset,
                    ),
                    (curr_width + eps, 2. * curr_height / 3. + offset),
                ));
            }
            curr_width += dimensions[i].0;
        }
        io_points
    }

    fn leave_road(&mut self, car_idx: usize, direction: Point) {
        // Set the fact that the car will leave the road
        self.cars[car_idx].is_leaving = Some(direction);
    }

    fn update_position(&mut self, car_idx: usize, accelerate: bool) {
        // Random bullshit, GO! v(t+dt) = (a-f*v(t))*dt
        self.cars[car_idx].speed += ((if accelerate { ACC_RATE } else { DECC_RATE })
            - FRICTION * self.cars[car_idx].speed)
            * DT;

        let grad = self.map[self.cars[car_idx].curve_index].compute_grad(self.cars[car_idx].t);
        // The distance traveled will be total_distance/||grad|| in order to keep a constant speed accross the curve
        let mut new_t = self.cars[car_idx].speed / grad.l2_norm() + self.cars[car_idx].t;
        let new_curve =
            (self.cars[car_idx].curve_index + ((new_t > 1.) as usize)).rem_euclid(self.map.len());

        // If the speed is too high for the curve, leave the road. Else, keep going.
        if self.map[new_curve]
            .compute_grad(new_t)
            .scalar(grad.normal())
            .abs()
            > SPEED_EXCESS
        {
            let pos = self.get_pos(car_idx);
            self.leave_road(car_idx, pos + grad * self.cars[car_idx].speed)
        } else {
            if new_t > 1. {
                self.cars[car_idx].curve_index = new_curve;
                new_t -= 1.;
            }
            self.cars[car_idx].t = new_t;
        }
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

    pub fn plot_game(&mut self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut car_pos = Vec::new();
        println!("Accelerating");
        for _ in 0..70 {
            self.update_position(0, true);
            car_pos.push(self.get_pos(0));
            println!("{:?}", self.get_pos(0));
        }
        println!("Breaking");
        for _ in 0..10 {
            self.update_position(0, false);
            car_pos.push(self.get_pos(0));
            println!("{:?}", self.get_pos(0));
        }
        println!("Accelerating");
        for _ in 0..50 {
            self.update_position(0, true);
            car_pos.push(self.get_pos(0));
            println!("{:?}", self.get_pos(0));
        }
        println!("Breaking");
        for _ in 0..20 {
            self.update_position(0, false);
            car_pos.push(self.get_pos(0));
            println!("{:?}", self.get_pos(0));
        }

        let root = BitMapBackend::new(name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Game"), ("sans-serif", 50))
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

    pub fn plot_map(&self, name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let (total_width, total_height) =
            self.map
                .iter()
                .fold((0., 0.), |(width, max_height), new_bezier| {
                    let points = new_bezier.get_points();
                    let width_bez = f64::max(points.1 .0, points.2 .0);
                    let height_bez = f64::max(points.1 .1, points.2 .1);
                    (f64::max(width, width_bez), f64::max(max_height, height_bez))
                });
        println!("current {}, {}", total_width, total_height);
        let root = BitMapBackend::new(name, (1024, 768)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .caption(format!("Game map"), ("sans-serif", 50))
            .build_cartesian_2d(-0.01f64..total_width, -0.1f64..total_height)?;
        for curve in self.map.iter() {
            chart.draw_series(LineSeries::new(curve.approx_points(100), RED))?;
        }
        root.present()?;
        println!("Map has been saved to {name}");

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_random_map() {
        // Can generate the points as required
        assert!(Game::get_io_map(&vec![(1., 1.), (0.3, 0.3), (1., 1.)])
            .iter()
            .zip(vec![
                ((0.9, 0.55), (0.9, 0.45)),
                ((0.9, 0.45), (1.1, 0.45)),
                ((1.1, 0.45), (1.2, 0.45)),
                ((1.2, 0.55), (1.1, 0.55)),
                ((1.1, 0.55), (0.9, 0.55)),
                ((1.2, 0.45), (1.4, 0.45)),
                ((1.4, 0.45), (1.4, 0.55)),
                ((1.4, 0.55), (1.2, 0.55)),
            ])
            .all(|(&(f1, f2), (f3, f4))| (f1.0 - f3.0) < f64::EPSILON
                && (f1.1 - f3.1) < f64::EPSILON
                && (f2.1 - f4.1) < f64::EPSILON
                && (f2.0 - f4.0) < f64::EPSILON));
    }
}
