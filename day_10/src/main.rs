use itertools::Itertools;
use utils::{split_str, process_file};

fn main() {
    let input = process_file("day_10_input.txt");
    let parsed_input: Vec<Machine> = input.iter().map(|n| n.into()).collect();
    let parsed_input_2: Vec<Machine> = input.iter().map(|n| n.into()).collect();
    let mut part1_total: usize = 0;
    let mut part2_total: usize = 0;

    for mut machine in parsed_input {
        part1_total += get_min_buttons(&mut machine);
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
    joltage_reqs: Vec<usize>,
}

impl From<&String> for Machine {
    fn from(item: &String) -> Self {
        let mut parsed_item: Vec<String> = split_str!(item);
        let mut lights_str = parsed_item[0].chars();
        lights_str.next();
        lights_str.next_back();
        let on_sequence: Vec<bool> = lights_str.map(|n| n == '#').collect();
        let joltage_reqs: Vec<usize> =
            split_str!(parsed_item.pop().expect("no joltage?"), ',', ['{', '}']);
        let buttons: Vec<Vec<usize>> = parsed_item[1..]
            .iter()
            .map(|i| split_str!(i, ',', ['(', ')']))
            .collect();

        Machine {
            lights: vec![false; on_sequence.len()],
            on_sequence,
            buttons,
            joltage: vec![0; joltage_reqs.len()],
            joltage_reqs,
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
        self.joltage = vec![0; self.joltage_reqs.len()];
    }

    fn buttons_for_jolts(&mut self) -> Vec<(usize, Vec<usize>)> {
        let mut buttons_list: Vec<(usize, Vec<usize>)> = vec![];
        for (j, jolts) in self.joltage_reqs.iter().enumerate() {
            let mut jolts_buttons: Vec<usize> = vec![];
            for (i, button) in self.buttons.iter().enumerate() {
                if button.contains(&j) {
                    jolts_buttons.push(i);
                }
            }
            buttons_list.push((*jolts, jolts_buttons));
        }

        buttons_list
    }
}

fn get_min_buttons(machine: &mut Machine) -> usize {
    machine.reset();
    let mut min_buttons = 1;
    let mut found = false;

    loop {
        let combs = (0..machine.buttons.len()).combinations(min_buttons);
        for comb in combs {
            machine.reset();
            machine.press_buttons(comb);
            if machine.lights == machine.on_sequence {
                found = true;
                break;
            }
        }

        if found {
            break;
        }
        if min_buttons >= 30 {
            break;
        } //arbitrary
        min_buttons += 1;
    }

    min_buttons
}

fn get_min_joltage_buttons(machine: &mut Machine) -> usize {
    machine.reset();
    let mut min_buttons: usize = *machine.joltage_reqs.iter().max().expect("joltage issue");
    let mut _found = false;
    // todo! may base this on the minimum buttons needed for each joltage position instead?

    let _max_jolts = machine.joltage_reqs.iter().position(|n| *n == min_buttons);

    min_buttons
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_press_button() {
        let mut m1: Machine =
            (&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()).into();
        m1.press_buttons(vec![4, 5]);
        assert_eq!(m1.lights, m1.on_sequence);
    }

    #[test]
    fn test_reset() {
        let mut m1: Machine =
            (&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()).into();
        m1.press_buttons(vec![4, 5]);
        m1.press_joltage_buttons(vec![2, 3, 4]);
        m1.reset();
        assert_eq!(m1.lights, vec![false; 4]);
    }

    #[test]
    fn test_buttons_for_jolts() {
        let mut m1: Machine =
            (&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()).into();
        let buttons = vec![
            (3, vec![4, 5]),
            (5, vec![1, 5]),
            (4, vec![2, 3, 4]),
            (7, vec![0, 1, 3]),
        ];
        assert_eq!(m1.buttons_for_jolts(), buttons);
    }

    #[test]
    fn test_get_min_buttons() {
        let mut m1: Machine =
            (&"[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}".to_string()).into();
        assert_eq!(get_min_buttons(&mut m1), 3);
    }

    #[test]
    fn test_get_min_joltage_buttons() {
        let mut m1: Machine =
            (&"[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}".to_string()).into();
        assert_eq!(get_min_joltage_buttons(&mut m1), 10);
    }
}
