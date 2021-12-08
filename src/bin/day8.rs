use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use itertools::Itertools;

fn main() {
    let f = File::open("input/input8.txt").unwrap();
    let reader = BufReader::new(f);
    let part = 2;
    let result: usize = reader
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let (digits, output) = parse_input(&line);
            if part == 1 {
                output
                    .into_iter()
                    .filter(|s| s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7)
                    .count()
            } else {
                let mapping = find_mapping(&digits);
                calculate_output(mapping, output)
            }
        })
        .sum();
    println!("{:?}", result);
}

fn parse_input<'a>(line: &'a String) -> (Vec<HashSet<char>>, Vec<HashSet<char>>) {
    let mut iter = line.split(" | ");
    let digits = iter
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    let output = iter
        .next()
        .unwrap()
        .split(" ")
        .map(|s| s.chars().collect::<HashSet<_>>())
        .collect::<Vec<_>>();
    (digits, output)
}

fn find_mapping(digits: &Vec<HashSet<char>>) -> HashMap<&char, char> {
    let mut mapping = HashMap::new();
    // c f
    let one = digits.into_iter().find(|s| s.len() == 2).unwrap();
    // a c f
    let seven = digits.into_iter().find(|s| s.len() == 3).unwrap();
    // b c d f
    let four = digits.into_iter().find(|s| s.len() == 4).unwrap();
    // a b c d e f g
    let eight = digits.into_iter().find(|s| s.len() == 7).unwrap();
    // 5 elements is one of 2, 3, 5
    // two is a c d e g
    // three is a c d f g
    // five is a b d f g
    // 6 elements is one of 0, 6, 9
    // zero is a b c e f g
    // six is a b d e f g
    // nine is a b c d f g

    let mut cf = one.intersection(seven);
    // c and f are the common elements in one and seven
    // a is the element in seven that is not in one
    let a = seven.difference(one).next().unwrap();
    // b and d are the elements in four that are not in one
    let mut bd = four.difference(one);
    // e and g are the elements in eight that are not in four or seven
    let eg = eight
        .difference(four)
        .filter(|&char| char != a)
        .collect::<Vec<_>>();

    let five_elements = digits
        .into_iter()
        .filter(|s| s.len() == 5)
        .collect::<Vec<_>>();
    let six_elements = digits
        .into_iter()
        .filter(|s| s.len() == 6)
        .collect::<Vec<_>>();

    // b is the element in only one 5 element digit
    // d is the element in all three 5 element digits
    let d = bd
        .clone()
        .filter(|e| five_elements.clone().into_iter().all(|s| s.contains(e)))
        .next()
        .unwrap();
    let b = bd.find(|&char| char != d).unwrap();
    // e is the element in only one 5 element digit
    // g is the element in all three 5 element digits
    let g = eg
        .clone()
        .into_iter()
        .filter(|ele| five_elements.clone().into_iter().all(|s| s.contains(ele)))
        .next()
        .unwrap();
    let e = eg.into_iter().find(|&char| char != g).unwrap();
    // c is the element in two 6 element digits
    // f is the element in all 6 element digits
    let f = cf
        .clone()
        .filter(|e| six_elements.clone().into_iter().all(|s| s.contains(e)))
        .next()
        .unwrap();
    let c = cf.find(|&char| char != f).unwrap();
    mapping.insert(a, 'a');
    mapping.insert(b, 'b');
    mapping.insert(c, 'c');
    mapping.insert(d, 'd');
    mapping.insert(e, 'e');
    mapping.insert(f, 'f');
    mapping.insert(g, 'g');
    mapping
}

fn calculate_output(mapping: HashMap<&char, char>, output: Vec<HashSet<char>>) -> usize {
    output
        .into_iter()
        .map(|out| {
            let real_segments = out.into_iter().map(|c| *mapping.get(&c).unwrap()).collect();
            get_digit_from_segments(real_segments)
        })
        .join("")
        .parse::<usize>()
        .unwrap()
}

fn get_digit_from_segments(segments: HashSet<char>) -> usize {
    let mut segment_mappings = vec![];
    segment_mappings.push((HashSet::from(['a', 'b', 'c', 'e', 'f', 'g']), 0));
    segment_mappings.push((HashSet::from(['c', 'f']), 1));
    segment_mappings.push((HashSet::from(['a', 'c', 'd', 'e', 'g']), 2));
    segment_mappings.push((HashSet::from(['a', 'c', 'd', 'f', 'g']), 3));
    segment_mappings.push((HashSet::from(['b', 'c', 'd', 'f']), 4));
    segment_mappings.push((HashSet::from(['a', 'b', 'd', 'f', 'g']), 5));
    segment_mappings.push((HashSet::from(['a', 'b', 'd', 'e', 'f', 'g']), 6));
    segment_mappings.push((HashSet::from(['a', 'c', 'f']), 7));
    segment_mappings.push((HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']), 8));
    segment_mappings.push((HashSet::from(['a', 'b', 'c', 'd', 'f', 'g']), 9));
    segment_mappings
        .into_iter()
        .find(|(set, _)| set.eq(&segments))
        .unwrap()
        .1
}
