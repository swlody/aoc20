use std::collections::BTreeMap;

pub fn can_contain_shiny_gold(map: &BTreeMap<&str, Vec<(&str, u32)>>, color: &str) -> bool {
    if color == "shiny gold" {
        return true;
    }
    for (color, _count) in map.get(color).unwrap() {
        if can_contain_shiny_gold(&map, color) {
            return true;
        }
    }
    false
}

pub fn input_generator(input: &str) -> BTreeMap<&str, Vec<(&str, u32)>> {
    let rules = input.lines();
    let mut map = BTreeMap::<&str, Vec<(&str, u32)>>::new();
    for rule in rules {
        let (base_color, rest) = rule.split_once(" bags contain ").unwrap();
        let mut vec = Vec::new();

        let all_colors = rest.split(", ");

        for color in all_colors {
            if color == "no other bags." {
                break;
            }
            let (num, color) = color.split_once(' ').unwrap();
            let num = num.parse::<u32>().unwrap();
            let contained_color = color.rsplit_once(' ').unwrap().0;
            vec.push((contained_color, num));
        }

        map.insert(base_color, vec);
    }
    map
}

pub fn solve_part1(map: &BTreeMap<&str, Vec<(&str, u32)>>) -> u32 {
    let mut total_count = 0;
    for (_color, contained) in map.iter() {
        for (color, _count) in contained.iter() {
            if can_contain_shiny_gold(&map, color) {
                total_count += 1;
                break;
            }
        }
    }
    total_count
}

fn get_count(map: &BTreeMap<&str, Vec<(&str, u32)>>, color: &str) -> u32 {
    let mut total_count = 0;
    for (color, count) in map.get(color).unwrap() {
        total_count += count;
        total_count += count * get_count(&map, color);
    }
    total_count
}

pub fn solve_part2(map: &BTreeMap<&str, Vec<(&str, u32)>>) -> u32 {
    let mut total_count = 0;
    for (color, count) in map.get("shiny gold").unwrap() {
        total_count += count;
        total_count += count * get_count(&map, color);
    }
    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        static INPUT: &str = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";

        let input = input_generator(INPUT);
        assert_eq!(4, solve_part1(&input));
    }

    #[test]
    fn test_part2() {
        static INPUT: &str = "shiny gold bags contain 2 dark red bags.
dark red bags contain 2 dark orange bags.
dark orange bags contain 2 dark yellow bags.
dark yellow bags contain 2 dark green bags.
dark green bags contain 2 dark blue bags.
dark blue bags contain 2 dark violet bags.
dark violet bags contain no other bags.";

        let input = input_generator(INPUT);
        assert_eq!(126, solve_part2(&input));
    }
}
