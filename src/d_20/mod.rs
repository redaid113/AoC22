#[derive(Debug)]
struct Element {
    value: i64,
    index: usize,
}

fn parse_input(contents: &str) -> Vec<Element> {
    contents
        .split('\n')
        .map(|num| num.parse().unwrap())
        .enumerate()
        .map(|(index, value)| Element { value, index })
        .collect()
}

fn number_toss(list: &mut Vec<Element>) {
    let total_size = list.len();

    for i in 0..total_size {
        let (index, element) = list
            .iter()
            .enumerate()
            .find(|(_, item)| item.index == i)
            .unwrap();
        let mut next_index = (index as i64 + element.value).rem_euclid(total_size as i64 - 1);
        if next_index == 0 && index != 0 {
            next_index = total_size as i64 - 1;
        }
        let e = list.remove(index);
        list.insert(next_index as usize, e);
    }
}

fn part1(contents: &str) -> i64 {
    let mut list = parse_input(contents);

    let total_size = list.len();

    number_toss(&mut list);
    let (index, _) = list
        .iter()
        .enumerate()
        .find(|(_, item)| item.value == 0)
        .unwrap();

    list[(index + 1000) % total_size].value
        + list[(index + 2000) % total_size].value
        + list[(index + 3000) % total_size].value
}

fn part2(contents: &str) -> i64 {
    let mut list = parse_input(contents);

    list.iter_mut().for_each(|e| e.value *= 811589153);

    let total_size = list.len();

    for _ in 0..10 {
        number_toss(&mut list);
    }

    let (index, _) = list
        .iter()
        .enumerate()
        .find(|(_, item)| item.value == 0)
        .unwrap();

    list[(index + 1000) % total_size].value
        + list[(index + 2000) % total_size].value
        + list[(index + 3000) % total_size].value
}

pub fn run(contents: &str) {
    println!("Part 1: {}", part1(contents));
    println!("Part 2: {}", part2(contents));
}
