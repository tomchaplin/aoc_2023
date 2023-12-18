use hashbrown::HashMap;
use num::integer::lcm;

use crate::Problem;
use aoc_parse::{parser, prelude::*};
pub struct Solution {}

type Key = (char, char, char);

fn string_to_key(s: String) -> Key {
    let chars_vec: Vec<_> = s.chars().collect();
    (chars_vec[0], chars_vec[1], chars_vec[2])
}

struct Tree(HashMap<Key, (Key, Key)>);

impl Tree {
    fn follow(&self, start: &mut Key, next_move: &Move) {
        let pair = self.0.get(start).expect("start should be a key");
        *start = match next_move {
            Move::Left => pair.0,
            Move::Right => pair.1,
        };
    }
}

enum Move {
    Left,
    Right,
}

impl Move {
    fn parse(c: char) -> Move {
        match c {
            'L' => Move::Left,
            'R' => Move::Right,
            _ => panic!("Unrecognised move"),
        }
    }
}

fn parse_input(input: &str) -> (Vec<Move>, Tree) {
    let moves_p = parser!(string(char_of("LR")+));
    let node_p = parser!(
        string(upper+) " = (" string(upper+) ", " string(upper+) ")"
    );
    let p = parser!(
        section(line(moves_p))
        section(lines(node_p))
    );
    let parsed_raw = p.parse(input).unwrap();
    let moves: Vec<_> = parsed_raw.0.chars().map(Move::parse).collect();

    let mut tree = HashMap::new();
    for (root, left, right) in parsed_raw.1 {
        tree.insert(
            string_to_key(root),
            (string_to_key(left), string_to_key(right)),
        );
    }
    (moves, Tree(tree))
}

impl Problem for Solution {
    fn solve_a(&self, input: &str) -> Option<String> {
        let (moves, tree) = parse_input(input);
        let mut move_cycle = moves.iter().cycle();
        let mut key = ('A', 'A', 'A');
        let mut n_moves = 0;

        while key != ('Z', 'Z', 'Z') {
            let next_move = move_cycle.next().unwrap();
            tree.follow(&mut key, next_move);
            n_moves += 1;
        }

        Some(n_moves.to_string())
    }

    fn solve_b(&self, input: &str) -> Option<String> {
        let (moves, tree) = parse_input(input);

        let ends_a_or_z = |k: &Key| k.2 == 'A' || k.2 == 'Z';
        let ends_a = |k: &Key| k.2 == 'A';

        let keys: Vec<_> = tree.0.keys().copied().filter(ends_a_or_z).collect();

        let mut endpoints: HashMap<Key, Key> = HashMap::new();
        let mut lengths: HashMap<Key, usize> = HashMap::new();

        // When does a (*, *, A|Z) key loop to another such key?
        // How long does it take
        for start_key in keys.iter() {
            let mut key = start_key.clone();
            let mut cont = true;
            let mut move_cycle = moves.iter().cycle();
            let mut n_moves = 0;
            while cont {
                let next_move = move_cycle.next().unwrap();
                tree.follow(&mut key, next_move);
                n_moves += 1;

                cont = !ends_a_or_z(&key);
            }
            endpoints.insert(start_key.clone(), key);
            lengths.insert(start_key.clone(), n_moves);
        }

        let mut answer = 1;

        // Assert that **A keys and **Z keys are paired up
        // and the loop goes A_key -> Z_key -> Z_key -> ...
        // with both paths being the same length
        for start_key in keys.into_iter().filter(ends_a) {
            let endp = endpoints.get(&start_key).unwrap();
            let second_endp = endpoints.get(endp).unwrap();
            assert_eq!(endp, second_endp);
            let l1 = lengths.get(&start_key).unwrap();
            let l2 = lengths.get(endp).unwrap();
            assert_eq!(l1, l2);
            answer = lcm(answer, *l1);
        }

        // Hence answer is LCM of cycle length

        Some(answer.to_string())
    }
}
