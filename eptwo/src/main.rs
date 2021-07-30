#![allow(dead_code)]

fn main() {
    let word: String;
    let x = adila();
    let y: bool = totalcheck(&x);
    word = if y {
        " ".to_string()
    } else {
        " not ".to_string()
    };
    println!("{} is{}the person.", x.name, word);
}

struct Person {
    green_eyes: bool,
    red_hair: bool,
    have_glasses: bool,
    name: String,
}

fn adila() -> Person {
    Person {
        green_eyes: true,
        red_hair: false,
        have_glasses: false,
        name: "adila".to_string(),
    }
}

fn rhcheck(person: &Person) -> bool {
    let name: Vec<char> = person.name.chars().collect();
    let mut x1 = 0;
    let mut y1 = 1;
    let mut check: bool = false;
    if person.red_hair {
        for _ in 0..(name.len() - 1) {
            if name[x1] == name[y1] {
                check = true;
            } else {
                x1 += 1;
                y1 += 1;
            }
        }
    } else {
        check = true
    }
    check
}

fn glasscheck(person: &Person) -> bool {
    let name: Vec<char> = person.name.chars().collect();
    let limit: u8;
    let mut count: u8 = 0;
    let check: bool;
    limit = if person.have_glasses { 2 } else { 3 };
    for i in name {
        count += match i {
            'a' | 'e' | 'i' | 'o' | 'u' | 'y' => 1,
            _ => 0,
        };
    }
    check = count == limit;
    check
}

fn totalcheck(person: &Person) -> bool {
    let check: bool;
    let geyes: bool = person.green_eyes;
    let rhcheck: bool = rhcheck(&person);
    let glasscheck: bool = glasscheck(&person);
    if geyes && rhcheck && glasscheck {
        check = true
    } else {
        check = false
    }
    check
}

fn samplesuccess() -> Person {
    Person {
        green_eyes: true,
        red_hair: true,
        have_glasses: true,
        name: "Roon".to_string(),
    }
}

fn samplesuccesses() -> Person {
    Person {
        green_eyes: true,
        red_hair: true,
        have_glasses: false,
        name: "rouovv".to_string(),
    }
}

fn samplefail() -> Person {
    Person {
        green_eyes: true,
        red_hair: true,
        have_glasses: false,
        name: "lolipop".to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn success_1() {
        assert!(totalcheck(&samplesuccess()));
    }

    #[test]
    fn success_2() {
        assert!(totalcheck(&samplesuccesses()));
    }

    #[test]
    fn fail() {
        assert!(!totalcheck(&samplefail()));
    }
}
