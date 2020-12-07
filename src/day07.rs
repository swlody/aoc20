type BagMap<'a> = fnv::FnvHashMap<&'a str, Vec<(&'a str, u32)>>;

fn contains_shiny_gold_recursive(map: &BagMap, color: &str) -> bool {
    if color == "shiny gold" {
        true
    } else {
        let contained = map.get(color).unwrap();
        contained
            .iter()
            .any(|(color, _count)| contains_shiny_gold_recursive(map, color))
    }
}

pub fn input_generator(input: &str) -> BagMap {
    input
        .lines()
        .map(|rule| {
            let (base_color, rest) = rule.split_once(" bags contain ").unwrap();
            let contained_bags = rest
                .split(", ")
                .filter_map(|contained| {
                    if contained == "no other bags." {
                        None
                    } else {
                        let (count, contained_color) = contained.split_once(' ').unwrap();
                        let count = count.parse::<u32>().unwrap();
                        let contained_color = contained_color.rsplit_once(' ').unwrap().0;
                        Some((contained_color, count))
                    }
                })
                .collect::<Vec<_>>();
            (base_color, contained_bags)
        })
        .collect()
}

pub fn solve_part1(map: &BagMap) -> usize {
    map.values()
        .filter(|contained| {
            contained
                .iter()
                .any(|(color, _count)| contains_shiny_gold_recursive(map, color))
        })
        .count()
}

fn get_contained_bag_count_recursive(map: &BagMap, color: &str) -> u32 {
    map.get(color)
        .unwrap()
        .iter()
        .fold(0, |acc, (color, count)| {
            acc + count + count * get_contained_bag_count_recursive(map, color)
        })
}

pub fn solve_part2(map: &BagMap) -> u32 {
    get_contained_bag_count_recursive(map, "shiny gold")
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
