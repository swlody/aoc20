use std::collections::BTreeMap;

type Passport = BTreeMap<String, String>;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Option<Vec<Passport>> {
    input
        .split("\n\n")
        .map(|entry| {
            entry
                .split_ascii_whitespace()
                .map(|field| {
                    let (key, value) = field.split_once(':')?;
                    Some((key.to_owned(), value.to_owned()))
                })
                .collect()
        })
        .collect()
}

#[aoc(day4, part1)]
fn solve_part1(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|entry| {
            let num_fields = entry.len();
            num_fields == 8 || num_fields == 7 && !entry.contains_key("cid")
        })
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[Passport]) -> usize {
    input
        .iter()
        .filter(|entry| {
            match entry.get("byr").and_then(|byr| byr.parse::<u16>().ok()) {
                Some(birth_year) if (1920..=2002).contains(&birth_year) => {}
                _ => return false,
            }

            match entry.get("iyr").and_then(|iyr| iyr.parse::<u16>().ok()) {
                Some(issue_year) if (2010..=2020).contains(&issue_year) => {}
                _ => return false,
            }

            match entry.get("eyr").and_then(|eyr| eyr.parse::<u16>().ok()) {
                Some(expiration_year) if (2020..=2030).contains(&expiration_year) => {}
                _ => return false,
            }

            match entry.get("hgt").and_then(|hgt| {
                let (height, measure) = hgt.split_at(hgt.len() - 2);
                Some((height.parse::<u16>().ok()?, measure))
            }) {
                Some((val, "cm")) if (150..=193).contains(&val) => {}
                Some((val, "in")) if (59..=76).contains(&val) => {}
                _ => return false,
            }

            match entry.get("hcl").map(|hcl| hcl.split_at(1)) {
                Some(("#", hair_color))
                    if hair_color.len() == 6
                        && hair_color.chars().all(|c| c.is_ascii_hexdigit()) => {}
                _ => return false,
            }

            match entry.get("ecl").map(String::as_str) {
                Some("amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth") => {}
                _ => return false,
            }

            match entry.get("pid") {
                Some(passport_id)
                    if passport_id.len() == 9
                        && passport_id.chars().all(|c| c.is_ascii_digit()) => {}
                _ => return false,
            }

            true
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        static INPUT: &str = "ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in";

        let input = input_generator(&INPUT).unwrap();
        assert_eq!(2, solve_part1(&input));
    }

    #[test]
    fn test_part2() {
        static INPUT1: &str = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
        static INPUT2: &str = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";

        let input = input_generator(&INPUT1).unwrap();
        assert_eq!(0, solve_part2(&input));

        let input = input_generator(&INPUT2).unwrap();
        assert_eq!(4, solve_part2(&input));
    }
}
