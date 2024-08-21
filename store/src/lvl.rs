use crate::Position;

pub fn get_spawn_positions(lvl: usize) -> Vec<Position> {
    match lvl {
        1 =>
            vec![
                Position { x: 1.0, y: 2.0, z: 3.0 },
                Position { x: 4.0, y: 5.0, z: 6.0 },
                Position { x: 7.0, y: 8.0, z: 9.0 },
                Position { x: 10.0, y: 11.0, z: 12.0 }
            ],
        2 =>
            vec![
                Position { x: -1.0, y: -2.0, z: -3.0 },
                Position { x: -4.0, y: -5.0, z: -6.0 },
                Position { x: 3.0, y: 7.0, z: -1.0 },
                Position { x: 0.0, y: 0.0, z: 0.0 },
                Position { x: 9.0, y: -7.0, z: 5.0 },
                Position { x: 2., y: 4., z: 6. }
            ],

        3 =>
            vec![
                Position { x: 14., y: -2., z: 6. },
                Position { x: 5., y: 9., z: 12. },
                Position { x: 0., y: -10., z: -10. },
                Position { x: 7., y: 3., z: 1. },
                Position { x: 11., y: 5., z: -8. },
                Position { x: 6., y: -4., z: 13. },
                Position { x: -2., y: -9., z: -4. }
            ],
        _ => unreachable!(),
    }
}
