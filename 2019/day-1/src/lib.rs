/* Takes in the mass of a component and returns the fuel required to launch it */
pub fn calc_fuel(mass: usize) -> usize {
    (mass / 3) - 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn calc_fuel_test() {
        let fuel = calc_fuel(11);
        assert_eq!(fuel, 1);

        let fuel = calc_fuel(9);
        assert_eq!(fuel, 1);

        let fuel = calc_fuel(10293);
        assert_eq!(fuel, 3429);
    }

    #[test]
    #[should_panic]
    fn calc_fuel_panics_on_underflow() {
        let fuel = calc_fuel(4);
    }
}
