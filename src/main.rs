use gobble::*;
use std::io::Read;

pub enum Convert {
    AsIs(StrPos),
    Str(String),
}

fn uc(c: char) -> bool {
    c >= 'A' && c <= 'Z'
}

fn lc_(c: char) -> bool {
    (c >= 'a' && c <= 'z') || c == '_'
}

fn main() {
    let mut stdin = std::io::stdin();
    let mut in_str = String::new();
    stdin.read_to_string(&mut in_str).unwrap();

    let mut lc = LCChars::str(&in_str);

    let cv = or3(
        str_pos(skip_repeat(
            or(common_str.ig(), Any.except((uc, lc_)).skip_min(1)),
            1,
        ))
        .map(|v| Convert::AsIs(v)),
        (
            maybe(uc.one()),
            repeat(
                or(
                    uc.one().map(|c| format!("_{}", c.to_lowercase())),
                    lc_.min_n(1),
                ),
                1,
            ),
        )
            .map(|(op, ar)| match op {
                Some(c) => {
                    let mut r = c.to_lowercase().to_string();
                    r.extend(ar);
                    Convert::Str(r)
                }
                None => Convert::Str(ar.into_iter().collect()),
            }),
        str_pos(uc.one()).map(|c| Convert::AsIs(c)),
    );

    while let Ok((nit, r)) = cv.parse(&lc) {
        lc = nit;
        match r {
            Convert::AsIs(sp) => print!("{}", sp.on_str(&in_str)),
            Convert::Str(s) => print!("{}", s),
        }
    }
}
