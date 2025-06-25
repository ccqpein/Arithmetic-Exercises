// pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
//     use std::collections::HashSet;

//     let r_count = grid.len() as i32;
//     let c_count = match grid.get(0) {
//         Some(c) => c.len() as i32,
//         None => return 0,
//     };

//     let mut all_visit = HashSet::new();
//     let mut this_round = HashSet::new();
//     let mut next = vec![];
//     for r in 0..r_count {
//         for c in 0..c_count {
//             if all_visit.contains(&(r, c)) {
//                 continue;
//             }

//             if grid[r as usize][c as usize] == 0 {
//                 all_visit.insert((r, c));
//                 continue;
//             }

//             next.push((r, c));

//             loop {
//                 let (r, c) = match next.pop() {
//                     Some((r, c)) => (r, c),
//                     None => break,
//                 };

//                 if grid[r as usize][c as usize] == 0 {
//                     all_visit.insert((r, c));
//                     continue;
//                 }

//                 this_round.insert((r, c));

//                 for (r_offset, c_offset) in vec![(-1, 0), (1, 0), (0, -1), (0, 1)] {
//                     if grid.get((r + r_offset) as usize).is_some() {
//                         if grid.get((c + c_offset) as usize).is_some() {
//                             next.push((r + r_offset, c + c_offset));
//                         }
//                     }
//                 }
//             }
//         }
//     }
// }

use std::collections::HashSet;

#[derive(Debug)]
pub struct Map<T> {
    /// total length of row and col
    r_len: usize,
    c_len: usize,

    inner: Vec<T>,
}

impl<T: Clone> Map<T> {
    pub fn new(r: usize, c: usize, init_v: T) -> Self {
        Self {
            r_len: r,
            c_len: c,
            inner: vec![init_v; r * c],
        }
    }

    pub fn row_len(&self) -> usize {
        self.r_len
    }

    pub fn col_len(&self) -> usize {
        self.c_len
    }

    fn coop_cal(&self, r: usize, c: usize) -> usize {
        // c and r start from 0
        self.c_len * r + c
    }

    pub fn get(&self, (x, y): (usize, usize)) -> Option<&T> {
        self.inner.get(self.coop_cal(x, y))
    }

    pub fn get_mut(&mut self, r: usize, c: usize) -> Option<&mut T> {
        let x = self.coop_cal(r, c);
        self.inner.get_mut(x)
    }

    pub fn set(&mut self, r: usize, c: usize, v: T) -> Result<(), String> {
        *self.get_mut(r, c).ok_or("not found".to_string())? = v;
        Ok(())
    }

    pub fn get_around(&self, (r, c): (usize, usize)) -> impl Iterator<Item = ((usize, usize), &T)> {
        let (r, c) = (r as isize, c as isize);
        [
            (r - 1, c - 1),
            (r - 1, c),
            (r - 1, c + 1),
            (r, c - 1),
            (r, c + 1),
            (r + 1, c - 1),
            (r + 1, c),
            (r + 1, c + 1),
        ]
        .into_iter()
        .filter_map(|(r, c)| {
            if r < 0 || c < 0 || r as usize >= self.r_len || c as usize >= self.c_len {
                None
            } else {
                Some((
                    (r as usize, c as usize),
                    self.get((r as usize, c as usize)).unwrap(),
                ))
            }
        })
    }

    pub fn get_around_horiz(
        &self,
        (r, c): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        let (r, c) = (r as isize, c as isize);
        [(r - 1, c), (r, c - 1), (r, c + 1), (r + 1, c)]
            .into_iter()
            .filter_map(|(r, c)| {
                if r >= 0 && c >= 0 && (r as usize) < self.r_len && (c as usize) < self.c_len {
                    Some((
                        (r as usize, c as usize),
                        self.get((r as usize, c as usize)).unwrap(),
                    ))
                } else {
                    None
                }
            })
    }

    /// get the line from coordinate to upper edge.
    /// order is from coord to edge. including the start element self.
    pub fn go_through_up(
        &self,
        (r, c): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        let coops = (0..=r).rev().into_iter().map(move |rr| (rr, c));
        coops.filter_map(|(rr, cc)| match self.get((rr, cc)) {
            Some(v) => Some(((rr, cc), v)),
            None => None,
        })
    }

    /// get the line from coordinate to bottom edge.
    /// order is from coord to edge. including the start element self.
    pub fn go_through_down(
        &self,
        (r, c): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        let coops = (r..self.r_len).into_iter().map(move |rr| (rr, c));
        coops.filter_map(|(rr, cc)| match self.get((rr, cc)) {
            Some(v) => Some(((rr, cc), v)),
            None => None,
        })
    }

    /// get the line from coordinate to left edge.
    /// order is from coord to edge. including the start element self.
    pub fn go_through_left(
        &self,
        (r, c): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        let coops = (0..=c).rev().into_iter().map(move |cc| (r, cc));
        coops.filter_map(|(rr, cc)| match self.get((rr, cc)) {
            Some(v) => Some(((rr, cc), v)),
            None => None,
        })
    }

    /// get the line from coordinate to right edge.
    /// order is from coord to edge. including the start element self.
    pub fn go_through_right(
        &self,
        (r, c): (usize, usize),
    ) -> impl Iterator<Item = ((usize, usize), &T)> {
        let coops = (c..self.c_len).into_iter().map(move |cc| (r, cc));
        coops.filter_map(|(rr, cc)| match self.get((rr, cc)) {
            Some(v) => Some(((rr, cc), v)),
            None => None,
        })
    }

    /// give map four corners, clockwised. they are indexable coop
    /// if map just one row, still return four corners but two of them are duplication
    pub fn four_corners(&self) -> [(usize, usize); 4] {
        let r = if self.row_len() == 0 {
            0
        } else {
            self.row_len() - 1
        };

        let c = if self.col_len() == 0 {
            0
        } else {
            self.col_len() - 1
        };

        [(0, 0), (0, c), (r, c), (r, 0)]
    }
}
#[derive(Debug)]
pub struct Segment<'m, T: Clone> {
    coops: HashSet<(usize, usize)>,
    value: T,
    map_inner: &'m Map<T>,
}

impl<'m, T: Clone + PartialEq + Eq> Segment<'m, T> {
    pub fn gen_segments(m: &'m Map<T>) -> Vec<Segment<'m, T>> {
        let mut result = vec![];
        let mut segmented = HashSet::new();
        let mut next = vec![];
        let mut visited = HashSet::new(); // visited in segmenting
        for r in 0..m.row_len() {
            for c in 0..m.col_len() {
                if segmented.get(&(r, c)).is_some() {
                    continue;
                }

                next.clear();
                next.push((r, c));
                let this_v = m.get((r, c)).unwrap();

                loop {
                    if next.is_empty() {
                        break;
                    }

                    let this_coop = next.pop().unwrap();
                    for (coop, v) in m.get_around_horiz(this_coop) {
                        if v == this_v && !visited.contains(&coop) {
                            next.push(coop);
                        }
                    }

                    visited.insert(this_coop);
                }

                result.push(Segment {
                    coops: visited.clone(),
                    value: this_v.clone(),
                    map_inner: m,
                });

                visited.iter().for_each(|vv| {
                    segmented.insert(*vv);
                });

                visited.clear();
            }
        }
        result
    }

    pub fn all_coops(&self) -> &HashSet<(usize, usize)> {
        &self.coops
    }

    pub fn value(&self) -> &T {
        &self.value
    }
}

impl<T> From<Vec<Vec<T>>> for Map<T> {
    fn from(v: Vec<Vec<T>>) -> Self {
        let r = v.len();
        let c = v[0].len();
        let vv = v.into_iter().map(|l| l.into_iter()).flatten().collect();

        Map {
            r_len: r,
            c_len: c,
            inner: vv,
        }
    }
}

pub fn max_area_of_island(grid: Vec<Vec<i32>>) -> i32 {
    let m: Map<i32> = grid.into();
    let se = Segment::gen_segments(&m);
    se.into_iter()
        .filter(|s| s.value == 1)
        .map(|s| s.all_coops().len())
        .max()
        .unwrap_or(0) as i32
}

fn main() {}
