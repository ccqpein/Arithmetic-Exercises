use core::iter::Iterator;
use nalgebra::{Dim, Dynamic, Matrix, VecStorage, Vector};
use std::collections::HashMap;
use std::io::{Error, ErrorKind, Result};

type StatusMatrix = Matrix<f32, Dynamic, Dynamic, VecStorage<f32, Dynamic, Dynamic>>;
type StatusVec = Vec<String>;
type WordsIndex = HashMap<String, usize>;

//
#[derive(Debug)]
pub struct SingleWord {
    this_word: String,
    next_word_map: HashMap<String, i64>,
    sum_count: i64,
}

impl SingleWord {
    fn new(w: String) -> Self {
        SingleWord {
            this_word: w,
            next_word_map: HashMap::new(),
            sum_count: 0,
        }
    }

    fn count_next(&mut self, next: &String) {
        if next == "" {
            return;
        };

        *self.next_word_map.entry(next.clone()).or_insert(0) += 1;
        self.sum_count += 1;
    }

    fn vector_maker(
        &self,
        l: &WordIndex,
    ) -> Vector<f32, Dynamic, VecStorage<f32, Dynamic, Dynamic>> {
        let mut ve = VecStorage::new(l.iter().count(), Dim.from_usize(1), vec![]);
        //iter map
        for (w, id) in self.next_word_map.iter() {}
    }
}

#[derive(Debug)]
pub struct WholeArticleMap {
    words_map: HashMap<String, SingleWord>,
    sum_num: i64,
}

impl WholeArticleMap {
    pub fn new() -> Self {
        WholeArticleMap {
            words_map: HashMap::new(),
            sum_num: 0,
        }
    }

    pub fn read_to_end(&mut self, buf: Vec<String>) -> Result<(i64, StatusVec)> {
        let mut head: String;
        let mut buf = buf.iter();
        let mut count = 0;
        let mut list: Vec<String> = vec![];

        if let Some(item) = buf.next() {
            head = item.into();
            count += 1;
            list.push(head.clone())
        } else {
            return Err(Error::from(ErrorKind::InvalidInput));
        }

        while let Some(b) = buf.next() {
            if b == "" {
                continue;
            }
            self.words_map
                .entry(head.clone())
                .or_insert(SingleWord::new(head.clone()))
                .count_next(b);
            head = b.clone();
            count += 1;
            list.push(head.clone());
        }

        self.sum_num = count;
        Ok((count, list))
    }
}

pub fn make_words_index(l: StatusVec) -> WordsIndex {
    l.iter()
        .cloned()
        .zip(0 as usize..)
        .collect::<HashMap<String, usize>>()
}
