use std::collections::VecDeque;

fn main() {
    let decks = vec![(0..10).collect(), (20..30).collect()];

    dbg!(do_stuff_with(decks));
}

fn do_stuff_with(mut decks: Vec<VecDeque<u32>>) -> Vec<VecDeque<u32>> {
    for _ in 0..7 {
        let new_decks: Vec<VecDeque<u32>> = decks
            .iter()
            .map(|d| d.iter().skip(1).cloned().collect())
            .collect();
        if !new_decks[0].is_empty() {
            let new_decks = do_stuff_with(new_decks);
            for (old, new) in decks.iter_mut().zip(new_decks.iter()) {
                old.pop_front();
                old.push_back(*new.front().unwrap());
            }
        }
    }
    decks
}
