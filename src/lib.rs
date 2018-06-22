mod unit;
use unit::Unit;
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
        let mut possible_next_positions = vec![current_position];
        let movement_distance = 1;
        for x in 0..self.size {
            for y in 0..self.size {
                let potential_new_position = Position::new(x,y);
                if self.occupied(potential_new_position) {continue;}
                if potential_new_position.get_distance_to(current_position) <= movement_distance {
                    possible_next_positions.push(potential_new_position);
                }
            }
        }
        possible_next_positions
    }

    fn occupied(&self, position: Position) -> bool {
        self.map.contains_key(&position)
    }

    fn move_unit(&mut self, current_position: Position, new_position: Position) -> bool {
        let movement_distance = 1;

        if !self.occupied(current_position) ||
            self.occupied(new_position) ||
            current_position.get_distance_to(new_position) > movement_distance {

            return false
        }

        let unit = self.map.remove(&current_position).unwrap();
        self.map.insert(new_position, unit);

        true
    }

    fn attack(&mut self, attack_position: Position, defend_position: Position) {
        if attack_position.get_distance_to(defend_position) > 1 {return;}

        let mut attacker = self.map.remove(&attack_position)
            .expect("No unit found at attack position");
        let mut defender = self.map.remove(&defend_position)
            .expect("No unit found at defense position");

        let (attacker_is_alive, defender_is_alive) = attacker.attack(&mut defender);

        if attacker_is_alive { self.map.insert(attack_position, attacker); }
        if defender_is_alive { self.map.insert(defend_position, defender); }
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

    #[test]
    fn world_occupied() {
        let mut world = World::new();
        let soldier1 = Unit::soldier();
        let current_position = Position::new(0,0);
        world.insert_unit(soldier1, current_position);

        assert!(world.occupied(current_position));
        assert!(!world.occupied(Position::new(1,1)));
    }

    #[test]
    fn world_get_possible_next_positions_cannot_move_to_occupied_location() {
        let mut world = World::new();
        let soldier1 = Unit::soldier();
        let current_position = Position::new(0,0);
        world.insert_unit(soldier1, current_position);
        world.insert_unit(Unit::soldier(), Position::new(0,1));

        let possible_next_positions = world.get_possible_next_positions(current_position);
        let expected_possible_next_positions = vec![Position::new(0,0), Position::new(1,0)];

        assert_eq!(expected_possible_next_positions, possible_next_positions);
    }

    #[test]
    fn world_move_success() {
        let mut world = World::new();
        let soldier1 = Unit::soldier();
        let current_position = Position::new(0,0);
        world.insert_unit(soldier1, current_position);
        let new_position = Position::new(0,1);

        let success = world.move_unit(current_position, new_position);
        assert!(success);
        assert!(world.occupied(new_position));
        assert!(!world.occupied(current_position));
    }

    #[test]
    fn world_move_too_far() {
        let mut world = World::new();
        let soldier1 = Unit::soldier();
        let current_position = Position::new(0,0);
        world.insert_unit(soldier1, current_position);
        let new_position = Position::new(3,3);

        let success = world.move_unit(current_position, new_position);
        assert!(!success);
        assert!(!world.occupied(new_position));
        assert!(world.occupied(current_position));
    }

    #[test]
    fn world_attack_no_units_removed(){
        let mut world = World::new();
        let mut soldier = Unit::soldier();
        let tank = Unit::tank();
        let soldier_position = Position::new(0,0);
        let tank_position = Position::new(0,1);

        world.insert_unit(soldier, soldier_position);
        world.insert_unit(tank, tank_position);

        world.attack(tank_position, soldier_position);
        assert_eq!(2, world.map.len());
    }

    #[test]
    fn world_attack_killed_defender_is_removed(){
        let mut world = World::new();
        let mut soldier = Unit::soldier();
        soldier.take_damage(9);
        let tank = Unit::tank();
        let soldier_position = Position::new(0,0);
        let tank_position = Position::new(0,1);

        world.insert_unit(soldier, soldier_position);
        world.insert_unit(tank, tank_position);

        world.attack(tank_position, soldier_position);
        assert_eq!(1, world.map.len());
    }

    #[test]
    fn world_attack_killed_attacker_is_removed(){
        let mut world = World::new();
        let mut soldier = Unit::soldier();
        soldier.take_damage(9);
        let tank = Unit::tank();
        let soldier_position = Position::new(0,0);
        let tank_position = Position::new(0,1);

        world.insert_unit(soldier, soldier_position);
        world.insert_unit(tank, tank_position);

        world.attack(soldier_position, tank_position);
        assert_eq!(1, world.map.len());
    }

    #[test]
    fn world_attack_out_of_range(){
        let mut world = World::new();
        let mut soldier = Unit::soldier();
        soldier.take_damage(9);
        let tank = Unit::tank();
        let soldier_position = Position::new(0,0);
        let tank_position = Position::new(2,2);

        world.insert_unit(soldier, soldier_position);
        world.insert_unit(tank, tank_position);

        world.attack(tank_position, soldier_position);
        // soldier is not killed because he is out of range of the tank
        assert_eq!(2, world.map.len());
    }
}
