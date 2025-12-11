use utils::process_file;
use itertools::Itertools;


fn main() {
    let input = process_file("day_10_input.txt");
    let parsed_input: Vec<Machine> = input.iter().map(|n| n.into()).collect();
    let parsed_input_2: Vec<Machine> = input.iter().map(|n| n.into()).collect();
    let mut part1_total: usize = 0;
    let mut part2_total: usize = 0;

    for mut machine in parsed_input {
        part1_total += get_minimum_buttons(&mut machine);
    }

    for mut machine in parsed_input_2 {
        part2_total += get_min_joltage_buttons(&mut machine);
    }

    println!("{:?}", part1_total);
    println!("{:?}", part2_total);
}


#[derive(Debug)]
struct Machine {
    lights: Vec<bool>,
    on_sequence: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<usize>,
    joltage_requirements: Vec<usize>,
}

impl From<&String> for Machine {
    fn from(item: &String) -> Self {
        let mut parsed_item: Vec<String> = item.split_whitespace().map(String::from).collect();
        let mut lights_str = parsed_item[0].chars();
        lights_str.next();
        lights_str.next_back();
        let on_sequence: Vec<bool> = lights_str.map(|n| n == '#').collect();
        let joltage_requirements: Vec<usize> = parsed_item
            .pop()
            .expect("no joltage?")
            .split(',')
            .map(|n| {
                n.replace(['{', '}'], "")
                    .parse::<usize>()
                    .expect("not a number")
            })
            .collect();
        let buttons = parsed_item[1..]
            .iter()
            .map(|i| {
                i.split(',')
                    .map(|n| {
                        n.replace(['(', ')'], "")
                            .parse::<usize>()
                            .expect("not a number")
                    })
                    .collect()
            })
            .collect();

        Machine {
            lights: vec![false; on_sequence.len()],
            on_sequence,
            buttons,
            joltage: vec![0; joltage_requirements.len()],
            joltage_requirements,
        }
    }
}


impl Machine {
    fn press_buttons(&mut self, buttons: Vec<usize>) {
        for button in buttons {
            let toggles = self.buttons.get(button).expect("outside of button range");
            for toggle in toggles {
                self.lights[*toggle] = !self.lights[*toggle];
            }
        }
    }

    fn press_joltage_buttons(&mut self, buttons: Vec<usize>) {
        for button in buttons {
            let toggles = self.buttons.get(button).expect("outside of button range");
            for toggle in toggles {
                self.joltage[*toggle] += 1;
            }
        }
    }

    fn reset(&mut self) {
        self.lights = vec![false; self.lights.len()];
        self.joltage = vec![0; self.joltage_requirements.len()];
    }
}


fn get_minimum_buttons(machine: &mut Machine) -> usize {
    machine.reset();
    let mut minimum_buttons = 1;
    let mut found = false;

    loop {
        let combs = (0..machine.buttons.len()).combinations(minimum_buttons);
        for comb in combs {
            machine.reset();
            machine.press_buttons(comb);
            if machine.lights == machine.on_sequence {
                found = true;
                break
            }
        }

        if found { break }
        if minimum_buttons >= 30 { break } //arbitrary
        minimum_buttons += 1;
    }

    minimum_buttons
}


fn get_min_joltage_buttons(machine: &mut Machine) -> usize {
    machine.reset();
    let mut minimum_buttons: usize = *machine.joltage_requirements.iter().max().expect("joltage issue");
    let mut found = false;
    // todo! may base this on the minimum buttons needed for each joltage position instead?

    loop {
        let combs = (0..machine.buttons.len()).combinations_with_replacement(minimum_buttons);
        dbg!(minimum_buttons);
        for comb in combs {
            machine.reset();
            machine.press_joltage_buttons(comb);
            if machine.joltage == machine.joltage_requirements {
                dbg!("joltage found");
                found = true;
                break
            }
        }

        if found { break }
        if minimum_buttons >= 300 { break } //arbitrary
        minimum_buttons += 1;
    }

    minimum_buttons
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_press_button() {
        let mut m1: Machine = (&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()).into();
        m1.press_buttons(vec![4,5]);
        assert_eq!(m1.lights, m1.on_sequence);
    }

    #[test]
    fn test_reset() {
        let mut m1: Machine = (&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()).into();
        m1.press_buttons(vec![4,5]);
        m1.press_joltage_buttons(vec![2,3,4]);
        m1.reset();
        assert_eq!(m1.lights, vec![false;4]);
    }

    #[test]
    fn test_get_minimum_buttons() {
        let mut m1: Machine = (&"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string()).into();
        assert_eq!(get_minimum_buttons(&mut m1), 3);

    }

    #[test]
    fn test_get_min_joltage_buttons() {
        let mut m1: Machine = (&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()).into();
        assert_eq!(get_min_joltage_buttons(&mut m1), 10);
    }
}
