fn levenshtein_distance(a: &str, b: &str) -> usize {
    let mut costs = vec![0; b.len() + 1];

    for j in 0..=b.len() {
        costs[j] = j;
    }

    for i in 1..=a.len() {
        let mut last_cost = i - 1;
        costs[0] = i;
        for j in 1..=b.len() {
            let new_cost = costs[j];
            costs[j] = std::cmp::min(
                std::cmp::min(costs[j] + 1, costs[j - 1] + 1),
                last_cost
                    + if a.as_bytes()[i - 1] == b.as_bytes()[j - 1] {
                        0
                    } else {
                        1
                    },
            );
            last_cost = new_cost;
        }
    }

    costs[b.len()]
}

fn closest_word<'a>(word: &str, word_list: &[&'a str]) -> (&'a str, usize) {
    let mut min_distance = usize::MAX;
    let mut closest = word_list[0];

    for &candidate in word_list {
        let distance = levenshtein_distance(word, candidate);
        if distance < min_distance {
            min_distance = distance;
            closest = candidate;
        }
    }

    (closest, min_distance)
}

pub fn check_closest_word<'a>(
    word: &'a str,
    word_list: &[&'a str],
    threshold: usize,
) -> Option<&'a str> {
    let (closest, distance) = closest_word(word, word_list);
    if distance <= threshold {
        Some(closest)
    } else {
        None
    }
}
