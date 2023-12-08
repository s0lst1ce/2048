use crate::{settings::Keybinds, *};

use bevy::prelude::*;

#[derive(Debug, Copy, Clone)]
pub struct MovingPlugin;

impl Plugin for MovingPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<Merged>()
            .add_event::<Direction>()
            .add_systems(
                Update,
                (
                    select_direction,
                    apply_move.after(select_direction).before(spawn_tile),
                )
                    .run_if(in_state(AppState::InGame)),
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

pub fn apply_move(
    mut commands: Commands,
    mut tiles: Query<(Entity, &mut Position, &mut TileKind), With<Tile>>,
    mut next_direction: EventReader<Direction>,
    mut new_tile: EventWriter<SpawnTile>,
    mut merged: EventWriter<Merged>,
    board: Res<Board>,
) {
    let Some(&direction) = next_direction.read().next() else {
        return;
    };

    let mut positions = vec![0; board.rows * board.columns];
    tiles
        .iter()
        .for_each(|(_, pos, kind)| positions[pos.index()] = kind.power());

    use self::tracker::MoveTracker;
    let mut tracker = MoveTracker::new(board.clone(), positions);

    tracker.start_tracking();
    let changed = tracker.apply(direction);
    merged.send_batch(changed);

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

#[derive(Debug, Clone, Event)]
pub struct Merged(u32);

impl Merged {
    pub fn from_power(pow: impl Into<u32>) -> Self {
        Self(pow.into())
    }

    pub fn power(&self) -> u32 {
        2u32.pow(self.0)
    }
}

mod tracker {
    use crate::{Board, Merged, Position, TileKind};

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

        fn transform<F>(&mut self, direction: Direction, mut transformation: F)
        where
            //stack -> (stack, changed)
            F: FnMut(Vec<(usize, usize)>) -> (Vec<(usize, usize)>, bool),
        {
            use Direction::*;
            for i in 0..match direction {
                Left | Right => self.board.rows,
                Up | Down => self.board.columns,
            } {
                //applying the changes to the current board
                let (out_stack, changed) = transformation(self.stack(direction, i).unwrap());
                self.changed |= changed;
                for (real_pos, kind) in out_stack {
                    self.tiles[real_pos] = kind
                }
            }
        }

        pub fn apply(&mut self, direction: Direction) -> Vec<Merged> {
            //todo re-write so that we can make this a one-pass operation (easy)
            self.go(direction);
            let merged = self.merge(direction);
            self.go(direction);
            merged
        }

        fn go(&mut self, direction: Direction) {
            self.transform(direction, |mut stack| {
                //first free tile
                let mut free = 0;
                //needle tracking the tile currently being operated on
                let mut cursor = free;
                //if any change to the board occured
                let mut changed = false;

                while let Some(&(_, kind)) = stack.get(cursor) {
                    if kind != 0 {
                        if stack[cursor].0 != stack[free].0 {
                            //there's a change only if we actually move the tile from a spot to another instead of in-place
                            changed = true;
                        }
                        //we must set the cursor's tile to zero first because it cursor==free it'd overwrite the data otherwise
                        stack[cursor].1 = 0; // since the tile was moved there's nothing in its stead
                        stack[free].1 = kind; // tile is put in the first free position

                        free += 1; // the next free spot is the next position
                    }
                    cursor += 1
                }

                (stack, changed)
            });
        }

        fn merge(&mut self, direction: Direction) -> Vec<Merged> {
            let mut merged = Vec::new();
            self.transform(direction, |mut stack| {
                let mut changed = false;
                for j in 0..(stack.len() - 1) {
                    if stack[j].1 != 0 && stack[j].1 == stack[j + 1].1 {
                        changed = true;
                        merged.push(Merged::from_power(stack[j].1 as u32));
                        stack[j].1 += 1; //the tile we merge into, we simply increase the index by one since the value is the power, not the shown value itself
                        stack[j + 1].1 = 0; //the tile that has now been destroyed for the merge
                    }
                }
                (stack, changed)
            });
            merged
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
