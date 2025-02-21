[![geo](https://avatars1.githubusercontent.com/u/10320338?v=4&s=50)](https://github.com/TimTheBig)

<!-- todo update -->
[![Coverage Status](https://img.shields.io/coverallsCoverage/github/georust/geo.svg)](https://coveralls.io/github/georust/geo?branch=trying)
[![Documentation](https://img.shields.io/docsrs/geo-3d/latest.svg)](https://docs.rs/geo-3d)

# geo 3D

## 3D Geospatial Primitives, Algorithms, and Utilities

### TODO
- make all methods work with 3d
- fix all doctests/tests
- add test/benchs for the volume calculation mod

The `geo-3d` crate provides geospatial primitive types such as `Point`, `LineString`, and `Polygon`, and provides algorithms and operations such as:
- Volume, Area and centroid calculation
- Simplification and convex hull operations
- Euclidean and Haversine distance measurement
- Intersection checks
- Affine transforms such as rotation and translation
- All DE-9IM spatial predicates such as contains, crosses, and touches.

Please refer to [the documentation](https://docs.rs/geo-3d) for a complete list.

The primitive types also provide the basis for other functionality in the `Geo` ecosystem, including:

- [Coordinate transformation and projection](https://github.com/georust/proj)

## Example

<!-- todo update -->
```rust
// primitives
use geo::{line_string, polygon};

// algorithms
use geo::ConvexHull;

// An L shape
let poly = polygon![
    (x: 0.0, y: 0.0, z: 0.0),
    (x: 4.0, y: 0.0, z: 4.0),
    (x: 4.0, y: 1.0, z: 2.5),
    (x: 1.0, y: 1.0, z: 1.0),
    (x: 1.0, y: 4.0, z: 7.0),
    (x: 0.0, y: 4.0, z: 0.0),
    (x: 0.0, y: 0.0, z: 0.0),
];

// Calculate the polygon's convex hull
let hull = poly.convex_hull();

assert_eq!(
    hull.exterior(),
    &line_string![
        (x: 4.0, y: 0.0),
        (x: 4.0, y: 1.0),
        (x: 1.0, y: 4.0),
        (x: 0.0, y: 4.0),
        (x: 0.0, y: 0.0),
        (x: 4.0, y: 0.0),
    ]
);
```

## Contributing

Contributions are welcome! Have a look at the [issues](https://github.com/TimTheBig/geo-3d/issues), and open a pull request if you'd like to add an algorithm or some functionality.

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or https://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or https://opensource.org/licenses/MIT)

at your option.
