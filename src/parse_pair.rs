use std::str::FromStr;

/// 把字符串 `s` （形如 `"400x600"` 或者 `"1.0,0.5"`）解析成一个坐标对
/// 
/// 具体来说，`s`应该具有<left><sep><right>的格式，其中<seq>是有 `separator`
/// 参数给出的字符串，而 <left> 和 <right> 是可以被 `T::from_str` 解析的字符串。
/// `separator` 必须是 ASCII 字符
/// 如果 `s` 具有正确的格式，就返回 `Some<x, y>`; 如果无法正确解析，就返回 `None`
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => {
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
        }
    }
}

fn sieve() {
    let mut arr = [true; 3];
    eprintln!("before sieve arr: {:?}", arr);
    for i in 1..3 {
        if arr[i] {
            let mut j = i * i;
            eprintln!("i {}, j {}", i, j);
            while j < 3 {
                arr[j] = false;
                j += i;
            }
        }
    }
    eprintln!("sieve [true; 3] result: {:?}", arr);
}

#[test]
fn test_sieve() {
    sieve();
}



#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("", ','), None);
    assert_eq!(parse_pair::<i32>("10, ", ','), None);
    assert_eq!(parse_pair::<i32>("10, 20", ','), Some((10, 20)));
}


struct Anime {
    name: &'static str,
    bechdel_pass: bool
}

#[test]
fn test_anime() {
    let aria = Anime {
        name: "do",
        bechdel_pass: true
    };

    let anime_ref = &aria;
    assert_eq!(anime_ref.name, "do");
    assert_eq!(anime_ref.bechdel_pass, true);
}