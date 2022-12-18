// quite proud of this but needs a bit of extra finesse
// breaks down for longer cycles or long sequences without cycles
#[derive(Debug, PartialEq, Eq)]
pub struct Cycle {
    pub start: usize,
    pub sequence: Vec<usize>,
    pub length: usize,
}
pub fn detect_cycle(sequence: &Vec<usize>, mut min_k: usize) -> Cycle {
    let mut best_match: Vec<usize> = Vec::new();
    let mut best_start = 0;
    let mut best_len = 0;
    let search_key = sequence[0]; // the cycle must contain the search key

    if sequence.len() < min_k * 2 {
        return Cycle {
            start: best_start,
            sequence: best_match,
            length: best_len,
        };
    }

    let mut p1: usize;
    'search: for i in 0..sequence.len() {
        // find next element of seq matching the search key
        if sequence[i] != search_key {
            continue;
        }
        p1 = i;

        let mut last_p2 = i;
        'joop: loop {
            // find next element of seq after i matching the search key
            // restrict iteration to elements in seq not already seen by p2
            let mut p2: Option<usize> = None;
            for j in last_p2 + 1..sequence.len() {
                // ignore p2 whose cycle would extend beyond sequence
                if j - p1 > sequence.len() - j {
                    break 'joop;
                }
                // dont bother with potential cycles smaller than min_k
                if sequence[j] == search_key && j - p1 > min_k {
                    p2 = Some(j);
                    break;
                }
            }
            // reached the end of sequence or otherwise aborted
            // without finding candidate for p2, start over with new i
            let p2 = match p2 {
                Some(p2) => p2,
                None => break 'joop,
            };

            // ensure next joop doesn't waste time searching j between
            // i and this candidate again
            last_p2 = p2;

            // we're in business
            // search p1..p2..p2+k for a cycle
            let mut this_match: Vec<usize> = Vec::new();
            for k in 0..(p2 - p1) {
                if sequence[p1 + k] != sequence[p2 + k] {
                    // move to next candidate p2
                    continue 'joop;
                }
                this_match.push(sequence[p1 + k]);
            }

            // we've found a cycle. we only check for cycles larger
            // than min_k so by definition it is the best, but we
            // should check its not the previous cycle glued together
            if best_len > 0 && best_match == this_match[0..best_len] {
                // we've found THE cycle
                break 'search;
            }
            best_len = this_match.len();
            best_match = this_match.clone();
            best_start = p1;
            min_k = p2 - p1; // ignore future cycles smaller than this
        }
    }
    Cycle {
        start: best_start,
        sequence: best_match,
        length: best_len,
    }
}
