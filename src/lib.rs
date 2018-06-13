use std::collections::HashMap;

struct World{
    map: HashMap<Position, Unit>,
    size: u16
}

impl World {
    fn new() -> World {
        World{ map: HashMap::new(), size: 3 }
    }

    fn insert_unit(&mut self, unit: Unit, position: Position){
        self.map.insert(position, unit);
    }

    fn get_map(&self) -> String {
        let mut map = String::new();
        for x in 0..self.size {
            for y in 0..self.size {
                match self.map.get(&Position::new(x,y)){
                    Some(_) => map += "x",
                    None => map += "0"
                }
            }
            map += "\n";
        }
        map
    }

    fn get_size(&self) -> u16 {
        self.size
    }

    fn get_possible_next_positions(&self, current_position: Position) -> Vec<Position> {
        let mut possible_next_positions = vec![];
        let movement_distance = 1;
        for x in 0..self.size {
            for y in 0..self.size {
                let potential_new_position = Position::new(x,y);
                if potential_new_position.get_distance_to(current_position) <= movement_distance {
                    possible_next_positions.push(potential_new_position);
                }
            }
        }
        possible_next_positions
    }
}

struct Unit;

impl Unit {
    fn soldier() -> Unit {
        Unit
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
struct Position {
    x: u16,
    y: u16
}

impl Position {
    fn new(x: u16, y:u16) -> Position {
        Position{x, y}
    }

    fn get_distance_to(&self, position2: Position) -> u16 {
        ((self.x as i32 - position2.x as i32).abs() + (self.y as i32 - position2.y as i32).abs()) as u16
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_constructor() {
        let world = World::new();
        assert_eq!(3, world.get_size());
    }

    #[test]
    fn world_get_map() {
        let mut world = World::new();

        assert_eq!("000\n000\n000\n", world.get_map());

        let soldier1 = Unit::soldier();

        world.insert_unit(soldier1, Position::new(0,0));

        assert_eq!("x00\n000\n000\n", world.get_map());
    }

    #[test]
    fn position_get_distance() {
        let position1 = Position::new(1,1);
        let position2 = Position::new(2,3);
        assert_eq!(3, position1.get_distance_to(position2));
        assert_eq!(3, position2.get_distance_to(position1));
    }

    #[test]
    fn world_get_possible_next_positions() {
        let mut world = World::new();
        let soldier1 = Unit::soldier();
        let current_position = Position::new(0,0);
        world.insert_unit(soldier1, current_position);

        let possible_next_positions = world.get_possible_next_positions(current_position);
        let expected_possible_next_positions = vec![Position::new(0,0), Position::new(0,1), Position::new(1,0)];

        assert_eq!(expected_possible_next_positions, possible_next_positions);
    }
}
