
fn calculate_needed_fuel(mass: i32) -> i32 {
    if mass <= 0 { 
        // println!("Needed fuel for mass {} was 0", mass);
        return 0;
    }
    let mut fuel_needed_for_mass = mass / 3 - 2;
    if fuel_needed_for_mass < 0 {
        fuel_needed_for_mass = 0;
    }
    let fuel_needed_for_fuel = calculate_needed_fuel(fuel_needed_for_mass);
    // println!("Needed fuel for mass {} was {} for mass and {} for fuel", mass, fuel_needed_for_mass, fuel_needed_for_fuel);
    return fuel_needed_for_mass + fuel_needed_for_fuel;
}


fn main() {
    let modules = [
        109364, 144584, 87498, 130293, 91960, 117563,
91730, 138879, 144269, 89058, 89982, 115609, 114728, 85422,
111803, 148524, 130035, 107558, 138936, 95622, 58042, 50697,
86848, 123301, 123631, 143125, 76434, 78004, 91115, 89062,
58465, 141127, 139993, 80958, 104184, 145131, 87438, 74385,
102113, 97392, 105986, 58600, 147156, 54377, 61409, 73552,
87138, 63168, 149602, 111776, 113191, 80137, 145985, 145177,
73192, 141631, 132979, 52565, 126574, 92005, 134655, 115894,
89175, 127328, 139873, 50072, 78814, 134750, 120848, 132950,
126523, 58206, 70885, 85482, 70889, 100029, 68447, 95111,
79896, 138650, 83079, 83112, 117762, 57223, 138122, 145193,
85251, 103331, 134501, 77023, 148189, 141341, 75994, 67024,
137767, 86260, 58705, 58771, 60684, 79655    ];
    let mut needed_fuel = 0;
    for module_mass in modules.iter() {
        needed_fuel += calculate_needed_fuel(*module_mass);
    }
    println!("fuel required = {}", needed_fuel);
}