/* Takes in the mass of a component and returns the fuel required to launch it */
pub fn calc_fuel(mass: usize) -> usize {
    match mass {
        m if m < 9 => 0,
        m if m >= 9 => ((mass / 3) - 2),
        _ => panic!()
    }
}

/* Takes in the mass of a component and returns the fuel required to launch it
   and the fuel used to launch it.
*/
pub fn calc_fuel_recursive(mass: usize) -> usize {
    calc_fuel_recursive_helper(mass, 0)
}

pub fn calc_fuel_recursive_helper(mass: usize, total: usize) -> usize {
    let fuel_required = calc_fuel(mass);
    match calc_fuel(mass) {
        0 => total + fuel_required,
        _ => calc_fuel_recursive_helper(fuel_required, total + fuel_required)
    }
    
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
    fn calc_fuel_recursive_test() {
        let fuel = calc_fuel_recursive(14);
        assert_eq!(fuel, 2);

        let fuel = calc_fuel_recursive(1969);
        assert_eq!(fuel, 966);

        let fuel = calc_fuel_recursive(100756);
        assert_eq!(fuel, 50346);
    }
}
