use crate::Position;

pub fn get_spawn_positions() -> Vec<Position> {
    return {
        vec![
            Position {
                x: -7.9,
                y: 0.2,
                z: -6.0,
            },
            Position {
                x: 3.9,
                y: 0.2,
                z: -6.3,
            },
            Position {
                x: 9.5,
                y: 0.2,
                z: -6.7,
            },
            Position {
                x: 9.1,
                y: 0.2,
                z: 10.5,
            },
            Position {
                x: -6.1,
                y: 0.2,
                z: 10.2,
            },
            Position {
                x: -5.2,
                y: 0.2,
                z: 2.2,
            },
            Position {
                x: 1.9,
                y: 0.2,
                z: -1.4,
            },
            Position {
                x: 0.2,
                y: 0.2,
                z: -2.1,
            },
            Position {
                x: -2.1,
                y: 0.2,
                z: 6.2,
            },
            Position {
                x: 3.0,
                y: 0.2,
                z: 3.8,
            },
        ]
    };
}
