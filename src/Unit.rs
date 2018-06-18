pub struct Unit {
    health: u16,
    attack: f32,
    defense: f32
}

impl Unit {
    pub fn soldier() -> Unit {
        Unit {
            health: 10,
            attack: 2f32,
            defense: 1f32,
        }
    }
    pub fn tank() -> Unit {
        Unit {
            health: 10,
            attack: 4f32,
            defense: 2f32,
        }
    }

    fn get_attack(&self) -> f32 {
        self.attack * self.health as f32 / 10f32
    }

    fn is_alive(&self) -> bool {
        self.health > 0
    }

    pub fn get_health(&self) -> u16 {
        self.health
    }

    pub fn take_damage(&mut self, damage: u16) -> bool {
        self.health = self.health.saturating_sub(damage);
        self.is_alive()
    }

    pub fn attack(&mut self, other: &mut Unit) -> (bool, bool) {
        let other_damage = (self.get_attack() / other.defense) as u16;
        other.take_damage(other_damage);
        let self_damage = ((other.get_attack() / self.defense) / 2f32) as u16;
        self.take_damage(self_damage);

        (self.is_alive(), other.is_alive())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_health_default_ten(){
        let unit = Unit::soldier();
        assert_eq!(10, unit.get_health());
    }

    #[test]
    fn take_damage(){
        let mut unit = Unit::soldier();

        assert!(unit.take_damage(9));
        assert_eq!(1, unit.get_health());

        assert!(!unit.take_damage(9));
        assert_eq!(0, unit.get_health());
    }

    #[test]
    fn attack_defender_is_more_heavily_damaged(){
        let mut unit1 = Unit::soldier();
        let mut unit2 = Unit::soldier();

        unit1.attack(&mut unit2);

        assert!(unit1.get_health() > unit2.get_health());
    }

    #[test]
    fn attack_tank_is_more_powerful_than_soldier(){
        let mut attacking_solider = Unit::soldier();
        let mut attacking_tank = Unit::tank();
        let mut unit1 = Unit::soldier();
        let mut unit2 = Unit::soldier();

        attacking_solider.attack(&mut unit1);
        attacking_tank.attack(&mut unit2);

        assert!(unit1.get_health() > unit2.get_health());
    }

    #[test]
    fn attack_attacking_unit_is_also_damaged(){
        let mut unit1 = Unit::soldier();
        let mut unit2 = Unit::tank();

        unit1.attack(&mut unit2);

        assert!(unit1.get_health() < 10);
    }

    #[test]
    fn attack_damaged_units_are_less_powerful_when_attacking(){
        let mut unit1 = Unit::soldier();
        let mut unit2 = Unit::soldier();
        unit1.attack(&mut unit2);
        let full_health_damage = 10 - unit2.get_health();

        unit1 = Unit::soldier();
        unit2 = Unit::soldier();
        unit1.take_damage(5);
        unit1.attack(&mut unit2);
        let half_health_damage = 10 - unit2.get_health();

        assert!(full_health_damage > half_health_damage);
    }

    #[test]
    fn attack_damaged_units_are_less_powerful_when_defending(){
        let mut unit1 = Unit::soldier();
        let mut unit2 = Unit::tank();
        unit1.attack(&mut unit2);
        let full_health_damage = 10 - unit1.get_health();

        unit1 = Unit::soldier();
        unit2 = Unit::tank();
        unit2.take_damage(5);
        unit1.attack(&mut unit2);
        let half_health_damage = 10 - unit1.get_health();

        assert!(full_health_damage > half_health_damage);
    }

    #[test]
    fn attack_tank_defense_is_greater_than_soldier_defense(){
        let mut attacking_tank = Unit::tank();
        let mut defending_soldier = Unit::soldier();
        let mut defending_tank = Unit::tank();

        attacking_tank.attack(&mut defending_soldier);

        attacking_tank = Unit::tank();
        attacking_tank.attack(&mut defending_tank);

        assert!(defending_tank.get_health() > defending_soldier.get_health());
    }

    #[test]
    fn attack_returns_alive_state(){
        let mut tank = Unit::tank();
        let mut soldier = Unit::soldier();

        assert_eq!((true, true), soldier.attack(&mut tank));

        tank = Unit::tank();
        soldier = Unit::soldier();
        soldier.take_damage(9);

        assert_eq!((true, false), tank.attack(&mut soldier));
    }

}
