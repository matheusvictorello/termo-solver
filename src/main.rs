use std::cmp;
use std::collections::HashMap;
use std::fs;

use float_ord::FloatOrd;

const FILENAME: &str = "termo_words_no_accents.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Status {
    Right,
    Wrong,
    Place,
}

#[derive(Debug, PartialEq, Eq)]
struct Pattern(Status, Status, Status, Status, Status);

impl Into<usize> for Pattern {
    fn into(self) -> usize {
        0 + match self.0 {
            Status::Right =>   0,
            Status::Place =>   1,
            Status::Wrong =>   2,
        } + match self.1 {
            Status::Right =>   0,
            Status::Place =>   3,
            Status::Wrong =>   6,
        } + match self.2 {
            Status::Right =>   0,
            Status::Place =>   9,
            Status::Wrong =>  18,
        } + match self.3 {
            Status::Right =>   0,
            Status::Place =>  27,
            Status::Wrong =>  54,
        } + match self.4 {
            Status::Right =>   0,
            Status::Place =>  81,
            Status::Wrong => 162,
        }
    }
}

impl From<[Status; 5]> for Pattern {
    fn from(s: [Status; 5]) -> Self {
        Pattern(s[0], s[1], s[2], s[3], s[4])
    }
}

fn main() {
    match fs::read_to_string(FILENAME) {
        Ok(contents) => {
            let words = contents.lines().collect::<Vec<_>>();
            termo(words);
        }
        Err(e) => {
            eprintln!("{:?}", e);
        }
    }
}

fn termo(all_words: Vec<&str>) {
    let multi_entries: Vec<Vec<(&str, &str)>> = vec![
        vec![
        ],
    ];

    let multi_words = multi_entries.iter()
        .map(|entries| {
            let mut local_words: Box<dyn Iterator<Item=&&str>> = Box::new(all_words.iter());

            for (used_word, pattern_str) in entries.iter() {
                let pattern = str_to_pattern(pattern_str).expect("Wrong Pattern string!");

                local_words = Box::new(
                    local_words.filter(move |hidden_word| try_fit(used_word, hidden_word) == pattern)
                );
            }

            local_words
                .map(|w| *w)
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<_>>();

    let scores = multi_words.iter()
        .map(|words| calc_score(&all_words, &words))
        .fold(HashMap::new(), |mut hm, scores| {
            for (score, word) in scores {
                let curr_score = hm.entry(word).or_insert(0.0);
                *curr_score += score; 
            }

            hm
        });

    let best = scores
        .into_iter()
        .map(|(word, score)| (FloatOrd(score), word))
        .max();

    println!("{:?}", best);
}

fn str_to_pattern(s: &str) -> Option<Pattern> {
    let s: &[u8] = s.as_bytes();

    if s.len() != 5 {
        return None;
    }

    let p0 = match s[0] {
        b'R' => Status::Right,
        b'W' => Status::Wrong,
        b'P' => Status::Place,
        _    => { return None; }
    };

    let p1 = match s[1] {
        b'R' => Status::Right,
        b'W' => Status::Wrong,
        b'P' => Status::Place,
        _    => { return None; }
    };

    let p2 = match s[2] {
        b'R' => Status::Right,
        b'W' => Status::Wrong,
        b'P' => Status::Place,
        _    => { return None; }
    };

    let p3 = match s[3] {
        b'R' => Status::Right,
        b'W' => Status::Wrong,
        b'P' => Status::Place,
        _    => { return None; }
    };

    let p4 = match s[4] {
        b'R' => Status::Right,
        b'W' => Status::Wrong,
        b'P' => Status::Place,
        _    => { return None; }
    };

    Some(Pattern(p0, p1, p2, p3, p4))
}

fn calc_score<'a>(w_words: &Vec<&'a str>, h_words: &Vec<&'a str>) -> Vec<(f32, &'a str)> {
    let total_h: usize = h_words.len();

    if total_h == 1 {
        return vec![(f32::INFINITY, h_words.get(0).unwrap())];
    }

    let total_w: usize = w_words.len();

    let mut scores = Vec::with_capacity(total_w);

    for w in w_words.iter() {
        let mut c = [0; 243];

        let mut s: f32 = 0.0;

        for h in h_words.iter() {
            let p = try_fit(w, h);

            let idx: usize = p.into();

            c[idx] += 1;
        }

        for f in c.iter() {
            let f = *f;

            if f == 0 {
                continue;
            }

            let p = (f as f32)/(total_h as f32);
            let i = (1.0/p).log(2.0);
            s    += p * i;
        }

        let r = (s, *w);

        println!("{:?}", r);
        
        scores.push(r);
    }

    scores
}

fn _try_fit(w: &str, h: &str) -> Pattern {
    let w = w.as_bytes();
    let h = h.as_bytes();

    let mut c = [0; 26];
    let mut p = [Status::Wrong; 5];

    for i in 0..5 {
        if w[i] == h[i] {
            p[i] = Status::Right;
        } else {
            let idx = (h[i] - b'a') as usize;
            
            c[idx] += 1;
        }
    }

    for i in 0..5 {
        if p[i] == Status::Wrong {
            let idx = (w[i] - b'a') as usize;
            
            let v = &mut c[idx];

            if *v > 0 {
                *v -= 1;
                p[i] = Status::Place;
            }
        }
    }

    Pattern::from(p)
}

fn try_fit(w: &str, h: &str) -> Pattern {
    let w = w.as_bytes();
    let h = h.as_bytes();

    let mut c = [0; 26];
    let mut p = Pattern(Status::Wrong, Status::Wrong, Status::Wrong, Status::Wrong, Status::Wrong);

    let w0 = w[0];
    let w1 = w[1];
    let w2 = w[2];
    let w3 = w[3];
    let w4 = w[4];

    let h0 = h[0];
    let h1 = h[1];
    let h2 = h[2];
    let h3 = h[3];
    let h4 = h[4];

    if w0 == h0 {
        p.0 = Status::Right;
    } else {
        let idx = (h0 - b'a') as usize;
        c[idx] += 1;
    }

    if w1 == h1 {
        p.1 = Status::Right;
    } else {
        let idx = (h1 - b'a') as usize;
        c[idx] += 1;
    }

    if w2 == h2 {
        p.2 = Status::Right;
    } else {
        let idx = (h2 - b'a') as usize;
        c[idx] += 1;
    }

    if w3 == h3 {
        p.3 = Status::Right;
    } else {
        let idx = (h3 - b'a') as usize;
        c[idx] += 1;
    }

    if w4 == h4 {
        p.4 = Status::Right;
    } else {
        let idx = (h4 - b'a') as usize;
        c[idx] += 1;
    }

    if p.0 == Status::Wrong {
        let idx = (w0 - b'a') as usize;
        
        let v = &mut c[idx];

        if *v > 0 {
            *v -= 1;
            p.0 = Status::Place;
        }
    }

    if p.1 == Status::Wrong {
        let idx = (w1 - b'a') as usize;
        
        let v = &mut c[idx];

        if *v > 0 {
            *v -= 1;
            p.1 = Status::Place;
        }
    }

    if p.2 == Status::Wrong {
        let idx = (w2 - b'a') as usize;
        
        let v = &mut c[idx];

        if *v > 0 {
            *v -= 1;
            p.2 = Status::Place;
        }
    }

    if p.3 == Status::Wrong {
        let idx = (w3 - b'a') as usize;
        
        let v = &mut c[idx];

        if *v > 0 {
            *v -= 1;
            p.3 = Status::Place;
        }
    }

    if p.4 == Status::Wrong {
        let idx = (w4 - b'a') as usize;
        
        let v = &mut c[idx];

        if *v > 0 {
            *v -= 1;
            p.4 = Status::Place;
        }
    }

    Pattern::from(p)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn f() {
        main();
    }
}