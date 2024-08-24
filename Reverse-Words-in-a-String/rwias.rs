pub fn reverse_words(s: String) -> String {
    let mut res = vec![];
    for w in s.split(' ').rev().filter(|w| *w != "") {
        res.push(" ");
        res.push(w);
    }

    let mut a = res.iter().map(|aa| *aa);
    a.next();
    String::from_iter(a)
}

fn main() {
    dbg!(reverse_words("the sky is blue".into()));
}
