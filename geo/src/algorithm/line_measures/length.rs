use super::Distance;
use crate::{CoordFloat, Line, LineString, MultiLineString, Point};

pub trait Length<F: CoordFloat> {
    fn length<MetricSpace: Distance<F, Point<F>, Point<F>>>(&self) -> F;
}

impl<F: CoordFloat> Length<F> for Line<F> {
    fn length<MetricSpace: Distance<F, Point<F>, Point<F>>>(&self) -> F {
        MetricSpace::distance(self.start_point(), self.end_point())
    }
}

impl<F: CoordFloat> Length<F> for LineString<F> {
    fn length<MetricSpace: Distance<F, Point<F>, Point<F>>>(&self) -> F {
        let mut length = F::zero();
        for line in self.lines() {
            length = length + line.length::<MetricSpace>();
        }
        length
    }
}

impl<F: CoordFloat> Length<F> for MultiLineString<F> {
    fn length<MetricSpace: Distance<F, Point<F>, Point<F>>>(&self) -> F {
        let mut length = F::zero();
        for line in self {
            length = length + line.length::<MetricSpace>();
        }
        length
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{coord, Euclidean, Geodesic, Haversine, Rhumb};

    #[test]
    fn lines() {
        // london to paris
        let line = Line::new(
            coord!(x: -0.1278f64, y: 51.5074),
            coord!(x: 2.3522, y: 48.8566),
        );

        assert_eq!(
            343_923., // meters
            line.length::<Geodesic>().round()
        );
        assert_eq!(
            341_088., // meters
            line.length::<Rhumb>().round()
        );
        assert_eq!(
            343_557., // meters
            line.length::<Haversine>().round()
        );

        // computing Euclidean length of an unprojected (lng/lat) line gives a nonsense answer
        assert_eq!(
            4., // nonsense!
            line.length::<Euclidean>().round()
        );
        // london to paris in EPSG:3035
        let projected_line = Line::new(
            coord!(x: 3620451.74f64, y: 3203901.44),
            coord!(x: 3760771.86, y: 2889484.80),
        );
        assert_eq!(344_307., projected_line.length::<Euclidean>().round());
    }

    #[test]
    fn line_strings() {
        let line_string = LineString::new(vec![
            coord!(x: -58.3816f64, y: -34.6037), // Buenos Aires, Argentina
            coord!(x: -77.0428, y: -12.0464),    // Lima, Peru
            coord!(x: -47.9292, y: -15.7801),    // Brasília, Brazil
        ]);

        assert_eq!(
            6_302_220., // meters
            line_string.length::<Geodesic>().round()
        );
        assert_eq!(
            6_332_790., // meters
            line_string.length::<Rhumb>().round()
        );
        assert_eq!(
            6_304_387., // meters
            line_string.length::<Haversine>().round()
        );

        // computing Euclidean length of an unprojected (lng/lat) gives a nonsense answer
        assert_eq!(
            59., // nonsense!
            line_string.length::<Euclidean>().round()
        );
        // EPSG:102033
        let projected_line_string = LineString::from(vec![
            coord!(x: 143042.46f64, y: -1932485.45), // Buenos Aires, Argentina
            coord!(x: -1797084.08, y: 583528.84),    // Lima, Peru
            coord!(x: 1240052.27, y: 207169.12),     // Brasília, Brazil
        ]);
        assert_eq!(
            6_237_538.,
            projected_line_string.length::<Euclidean>().round()
        );
    }
}
