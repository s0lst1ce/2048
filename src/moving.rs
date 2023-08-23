use crate::{settings::Keybinds, *};

use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Direction>().add_systems(
            Update,
            (
                select_direction,
                apply_move.after(select_direction).before(spawn_tile),
            ),
        );
    }
}

#[derive(Debug, Event, Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

fn select_direction(
    keys: Res<Input<KeyCode>>,
    mut next_direction: EventWriter<Direction>,
    keybinds: Res<Keybinds>,
) {
    let direction = if keys.just_released(keybinds.move_left) {
        Direction::Left
    } else if keys.just_released(keybinds.move_up) {
        Direction::Up
    } else if keys.just_released(keybinds.move_right) {
        Direction::Right
    } else if keys.just_released(keybinds.move_down) {
        Direction::Down
    } else {
        return;
    };
    next_direction.send(direction)
}

fn apply_move(
    mut commands: Commands,
    mut tiles: Query<(Entity, &mut Position, &mut TileKind), With<Tile>>,
    mut next_direction: EventReader<Direction>,
    mut new_tile: EventWriter<SpawnTile>,
    board: Res<Board>,
) {
    let Some(&direction) = next_direction.iter().next() else {
        return;
    };

    let mut positions = vec![0; board.rows * board.columns];
    tiles
        .iter()
        .for_each(|(_, pos, kind)| positions[pos.index()] = kind.power());

    use self::tracker::MoveTracker;
    let mut tracker = MoveTracker::new(board.clone(), positions);

    tracker.go(direction);
    tracker.merge(direction);
    tracker.go(direction);

    if tracker.has_changed() {
        let tracker_tiles = tracker.tiles();
        let mut new_tiles = tracker_tiles.into_iter();

        for (entity, mut pos, mut kind) in tiles.iter_mut() {
            if let Some((new_pos, new_kind)) = new_tiles.next() {
                *pos = new_pos;
                *kind = new_kind;
            } else {
                commands.entity(entity).despawn_recursive();
            }
        }

        //the state remains so we don't start over, the iterator is the same as before
        for (pos, kind) in new_tiles {
            new_tile.send(SpawnTile {
                position: Some(pos),
                kind: Some(kind),
            })
        }

        new_tile.send(SpawnTile::default());
    }
}

mod tracker {
    use crate::{Board, Position, TileKind};

    use super::Direction;

    #[derive(Debug)]
    pub struct MoveTracker {
        board: Board,
        /// The current state of the board
        tiles: Vec<usize>,
        changed: bool,
    }

    impl MoveTracker {
        pub fn new(board: Board, tiles: Vec<usize>) -> Self {
            Self {
                board,
                tiles,
                changed: false,
            }
        }

        /// Slice of (`real_position`, `kind`), the `nth` one for `direction`
        fn stack(&self, direction: Direction, nth: usize) -> Option<Vec<(usize, usize)>> {
            use Direction::*;
            if nth >= self.board.rows {
                None
            } else {
                let stack = self
                    .tiles
                    .iter()
                    .enumerate()
                    //todo move this match outside the closure  so we only check once
                    .filter(|(idx, _)| match direction {
                        Left | Right => idx / self.board.columns == nth,
                        Up | Down => idx % self.board.columns == nth,
                    })
                    .map(|(idx, kind)| (idx, *kind));

                let stack = match direction {
                    Right | Down => stack.rev().collect(),
                    Left | Up => stack.collect(),
                };

                Some(stack)
            }
        }

        fn transform<F>(&mut self, direction: Direction, tranformation: F)
        where
            //stack -> (stack, changed)
            F: Fn(Vec<(usize, usize)>) -> (Vec<(usize, usize)>, bool),
        {
            use Direction::*;
            for i in 0..match direction {
                Left | Right => self.board.rows,
                Up | Down => self.board.columns,
            } {
                //applying the changes to the current board
                let (out_stack, changed) = tranformation(self.stack(direction, i).unwrap());
                self.changed |= changed;
                for (real_pos, kind) in out_stack {
                    self.tiles[real_pos] = kind
                }
            }
        }

        pub fn go(&mut self, direction: Direction) {
            self.transform(direction, |mut stack| {
                let mut last = 0;
                let mut cursor = last;
                let mut changed = false;

                while let Some(&(_, kind)) = stack.get(cursor) {
                    if kind != 0 {
                        changed = true;
                        //we must set the cursor's tile to zero first because it cursor==last it'd overwrite the data otherwise
                        stack[cursor].1 = 0; // since the tile was moved there's nothing in its stead
                        stack[last].1 = kind; // tile is put in the last free position

                        last += 1; // the next free spot is the next position
                    }
                    cursor += 1
                }
                (stack, changed)
            });
        }

        pub fn merge(&mut self, direction: Direction) {
            self.transform(direction, |mut stack| {
                let mut changed = false;
                for j in 0..(stack.len() - 1) {
                    if stack[j].1 != 0 && stack[j].1 == stack[j + 1].1 {
                        changed = true;
                        stack[j].1 += 1; //the tile we merge into, we simply increase the index by one since the value is the power, not the shown value itself
                        stack[j + 1].1 = 0; //the tile that has now been destroyed for the merge
                    }
                }
                (stack, changed)
            });
        }

        pub fn tiles(&mut self) -> Vec<(Position, TileKind)> {
            let mut tiles = vec![];
            for (pos, &kind) in self.tiles.iter().enumerate() {
                if kind != 0 {
                    tiles.push((pos.into(), TileKind::from_power(kind as u32)));
                }
            }
            tiles
        }

        pub fn has_changed(&self) -> bool {
            self.changed
        }

        pub fn start_tracking(&mut self) {
            self.changed = false;
        }
    }
}
