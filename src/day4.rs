use std::collections::BTreeMap;

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Option<Vec<BTreeMap<String, String>>> {
    input
        .split("\n\n")
        .map(|entry| -> Option<BTreeMap<String, String>> {
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
fn solve_part1(input: &[BTreeMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|entry| {
            let num_fields = entry.len();
            num_fields == 8 || num_fields == 7 && !entry.contains_key("cid")
        })
        .count()
}

#[aoc(day4, part2)]
fn solve_part2(input: &[BTreeMap<String, String>]) -> usize {
    input
        .iter()
        .filter(|entry| {
            if let Some(birth_year) = entry
                .get("byr")
                .and_then(|birth_year| birth_year.parse::<u16>().ok())
            {
                if !(1920..=2002).contains(&birth_year) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(issue_year) = entry
                .get("iyr")
                .and_then(|issue_year| issue_year.parse::<u16>().ok())
            {
                if !(2010..=2020).contains(&issue_year) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(expiration_year) = entry
                .get("eyr")
                .and_then(|expiration_year| expiration_year.parse::<u16>().ok())
            {
                if !(2020..=2030).contains(&expiration_year) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(height) = entry.get("hgt") {
                if !height.ends_with("cm") && !height.ends_with("in") {
                    return false;
                }
                if let Some(hgt) = height
                    .rsplit_once("in")
                    .and_then(|hgt| hgt.0.parse::<u16>().ok())
                {
                    if !(59..=76).contains(&hgt) {
                        return false;
                    }
                } else if let Some(hgt) = height
                    .rsplit_once("cm")
                    .and_then(|hgt| hgt.0.parse::<u16>().ok())
                {
                    if !(150..=193).contains(&hgt) {
                        return false;
                    }
                } else {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(hair_color) = entry.get("hcl") {
                if !hair_color.starts_with('#')
                    || hair_color.len() != 7
                    || !hair_color.chars().skip(1).all(|c| c.is_ascii_hexdigit())
                {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(eye_color) = entry.get("ecl") {
                if !matches!(
                    eye_color.as_str(),
                    "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth"
                ) {
                    return false;
                }
            } else {
                return false;
            }

            if let Some(passport_id) = entry.get("pid") {
                if passport_id.len() != 9 || !passport_id.chars().all(|c| c.is_ascii_digit()) {
                    return false;
                }
            } else {
                return false;
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
