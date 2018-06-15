pub struct Unit {
    health: u16,
}

impl Unit {
    pub fn soldier() -> Unit {
        Unit {
            health: 10,
        }
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

    pub fn attack(&mut self, other: &mut Unit) {
        other.take_damage(2);
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
    fn attack(){
        let mut unit1 = Unit::soldier();
        let mut unit2 = Unit::soldier();

        unit1.attack(&mut unit2);

        assert!(unit1.get_health() > unit2.get_health());
    }

}
