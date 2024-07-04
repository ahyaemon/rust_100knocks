pub mod knocks;

mod parse {
    use std::fmt::Debug;
    use std::str::FromStr;

    ///
    /// 受け取った文字列参照 s を、sep で分割する。
    /// 分割時に型 T に変換する。
    ///
    pub fn parse_sep<T>(s: &str, sep: char) -> Vec<T>
    where
        T: FromStr,
        <T as FromStr>::Err: Debug,
    {
        s.split(sep)
            .map(|ss| {
                ss.parse::<T>()
                    .unwrap_or_else(|_| panic!("\"{}\" can not be parsed", s))
            })
            .collect()
    }
}

#[cfg(test)]
mod test_utils {
    use std::fs;

    /// resources ディレクトリ配下のリソースを読み込み、文字列で返す。
    pub fn read_resource(s: &str) -> String {
        let filepath = format!("resources/{}", s);
        fs::read_to_string(filepath).unwrap().trim().into()
    }
}
