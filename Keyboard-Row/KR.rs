pub fn find_words(words: Vec<String>) -> Vec<String> {
    use std::collections::HashSet;
    let one = {
        let temp: HashSet<u8> = "qwertyuiop".as_bytes().iter().cloned().collect();
        //for c in vec![b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p'] {
        //    temp.insert(c);
        //}
        temp
    };
    //dbg!(&one);

    let two = {
        let temp: HashSet<u8> = "asdfghjkl".as_bytes().iter().cloned().collect();
        // for c in vec![b'a', b's', b'd', b'f', b'g', b'g', b'h', b'j', b'k', b'l'] {
        //     temp.insert(c);
        // }
        temp
    };
    //dbg!(&two);

    let three = {
        let temp: HashSet<u8> = "zxcvbnm".as_bytes().iter().cloned().collect();
        // let _ = "zxcvbnm".as_bytes().iter().map(|by| {
        //     temp.insert(by);
        // }); //=> ??????

        // for c in vec![b'z', b'x', b'c', b'v', b'b', b'n', b'm'] {
        //     temp.insert(c);
        // }
        temp
    };
    //dbg!(&three);

    let re = words.iter().map(|x| {
        let low_x = x.to_lowercase();
        let mut bytes = low_x.bytes();
        let temp_set = {
            let first = bytes.next().unwrap();
            let a = if one.contains(&first) {
                &one
            } else if two.contains(&first) {
                &two
            } else {
                &three
            };
            a
        };

        if bytes.all(|x| temp_set.contains(&x)) {
            return x.clone();
        } else {
            "".to_string()
        }
    });

    re.filter(|x| x != "").collect::<Vec<String>>()
}

fn main() {
    dbg!(find_words(vec![
        String::from("Hello"),
        String::from("Alaska"),
        String::from("Dad"),
        String::from("Peace")
    ]));
}
