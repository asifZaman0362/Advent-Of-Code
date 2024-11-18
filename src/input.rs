use std::io::{Read, Write};

#[derive(serde::Deserialize, Debug)]
struct Cookie {
    session: String,
}

pub fn read_input(test: bool, year: u16, day: u8) -> anyhow::Result<Vec<String>> {
    let buf = if test {
        let mut buf = String::new();
        let mut file = std::fs::File::open("test.txt")?;
        file.read_to_string(&mut buf)?;
        buf
    } else {
        match std::fs::File::open(format!("{year}_{day}_input.txt")) {
            Ok(mut file) => {
                let mut buf = String::new();
                file.read_to_string(&mut buf)?;
                buf
            }
            Err(e) => {
                if e.kind() == std::io::ErrorKind::NotFound {
                    let mut cookie = std::fs::File::open(".cookie")?;
                    let mut buf = String::new();
                    cookie.read_to_string(&mut buf)?;
                    let cookie: Cookie = serde_json::from_str(buf.trim())?;
                    let client = reqwest::blocking::Client::new();
                    let buf = client
                        .get(format!("https://adventofcode.com/{year}/day/{day}/input"))
                        .header(
                            reqwest::header::COOKIE,
                            format!("session={}", &cookie.session),
                        )
                        .send()?
                        .text()?;
                    let mut file = std::fs::File::create(format!("{year}_{day}_input.txt"))?;
                    file.write_all(buf.as_bytes())?;
                    buf
                } else {
                    panic!("{}", e)
                }
            }
        }
    };
    Ok(buf
        .trim_end()
        .split('\n')
        .map(|x| x.to_owned())
        .collect::<Vec<_>>())
}
