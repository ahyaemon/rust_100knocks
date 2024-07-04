/// # No. 00 ごあいさつ
///
/// 実行するとHello World!と表示するプログラムを作成せよ。
///
/// ---
///
/// - 戻り値: `"Hello, World!"`
pub fn knock_00() -> String {
    todo!()
}

#[cfg(test)]
mod tests_00 {
    use crate::knocks::knocks_00_09::knock_00;

    #[test]
    fn it_works() {
        let actual = knock_00();
        let expected = String::from("Hello, World!");
        assert_eq!(actual, expected);
    }
}

/// # No. 01 足し算
///
/// 12345+23456を計算して結果を表示するプログラムを作成せよ。
///
/// ---
///
/// - 戻り値: `12345 + 23456` の計算結果
pub fn knock_01() -> usize {
    todo!()
}

#[cfg(test)]
mod tests_01 {
    use crate::knocks::knocks_00_09::knock_01;

    #[test]
    fn it_works() {
        let actual = knock_01();
        let expected = 35801;
        assert_eq!(actual, expected);
    }
}

/// # No. 02 余り
///
/// 12345を7で割った余りを表示するプログラムを作成せよ。
///
/// ---
///
/// - 戻り値: 12345 を 7 で割った余り
pub fn knock_02() -> usize {
    todo!()
}

#[cfg(test)]
mod tests_02 {
    use crate::knocks::knocks_00_09::knock_02;

    #[test]
    fn it_works() {
        let actual = knock_02();
        let expected = 4;
        assert_eq!(actual, expected);
    }
}

/// # No. 03 入力
///
/// 整数値を入力させ、その入力値を表示するプログラムを作成せよ。
///
/// ---
///
/// 表示内容を返すように改変
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 整数値
pub fn knock_03(a: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests_03 {
    use crate::knocks::knocks_00_09::knock_03;

    #[test]
    fn it_works() {
        let actual = knock_03("123");
        let expected = 123;
        assert_eq!(actual, expected);
    }
}

/// # No. 04 入力と計算
///
/// 整数値を入力させ、その入力値を3倍した計算結果を表示するプログラムを作成せよ。
///
/// ---
///
/// 表示内容を返すように改変
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 整数値を 3 枚した結果
pub fn knock_04(a: &str) -> i32 {
    todo!()
}

#[cfg(test)]
mod tests_04 {
    use crate::knocks::knocks_00_09::knock_04;

    #[test]
    fn it_works() {
        let actual = knock_04("123");
        let expected = 369;
        assert_eq!(actual, expected);
    }
}

/// # No. 05 四則演算
///
/// 整数値を2つ入力させ、それらの値の和、差、積、商と余りを求めるプログラムを作成せよ。
/// なお、差と商は1つ目の値から2つ目の値を引いた、あるいは割った結果とする。
/// 余りは無い場合も0と表示するのでよい。
///
/// ---
///
/// - 引数: 二つの整数値をスペース区切りにした文字列参照
/// - 戻り値: 長さ 5 の配列 `[和, 差, 積, 商, 余り]`
pub fn knock_05(a: &str, b: &str) -> [i32; 5] {
    todo!()
}

#[cfg(test)]
mod tests_05 {
    use crate::knocks::knocks_00_09::knock_05;

    #[test]
    fn it_works_123_7() {
        let actual = knock_05("123", "7");
        let expected = [130, 116, 861, 17, 4];
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_123_3() {
        let actual = knock_05("123", "3");
        let expected = [126, 120, 369, 41, 0];
        assert_eq!(actual, expected);
    }
}

/// # No. 06 0?
///
/// 整数値を入力させ、値が0ならzeroと表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `Option<String>`
///   - 値が 0 => `Some(0)`
///   - それ以外 => `None`
pub fn knock_06(a: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests_06 {
    use crate::knocks::knocks_00_09::knock_06;

    #[test]
    fn it_works_some() {
        let actual = knock_06("0");
        let expected = Some(String::from("zero"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none() {
        let actual = knock_06("1");
        let expected = None;
        assert_eq!(actual, expected);
    }
}

/// # No. 07 0 or not 0
///
/// 整数値を入力させ、値が0ならzero、0でなければnot zeroと表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 入力値が 0 なら `"zero"`, 0 でなければ `"not zero"`
pub fn knock_07(a: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_07 {
    use crate::knocks::knocks_00_09::knock_07;

    #[test]
    fn it_works_zero() {
        let actual = knock_07("0");
        let expected = String::from("zero");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_not_zero() {
        let actual = knock_07("1");
        let expected = String::from("not zero");
        assert_eq!(actual, expected);
    }
}

/// # No. 08 正の整数?
///
/// 整数値を入力させ、値が正であればpositiveと表示するプログラムを作成せよ。ただし0は正には含まない。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `Option<String>`
///   - 正の値の場合は `Some`
///   - それ以外の場合は `None`
pub fn knock_08(a: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests_08 {
    use crate::knocks::knocks_00_09::knock_08;

    #[test]
    fn it_works_some() {
        let actual = knock_08("0");
        let expected = Some(String::from("positive"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none() {
        let actual = crate::knocks::knocks_00_09::knock_08("1");
        let expected = None;
        assert_eq!(actual, expected);
    }
}

/// # No. 09 正? 負? 0?
///
/// 整数値を入力させ、値が正であればpositive、負であればnegative、0であればzeroと表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値:
///   - 正の値: `"positive"`
///   - 0: `"zero"`
///   - 負の値: `"negative"`
pub fn knock_09(a: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_09 {
    use crate::knocks::knocks_00_09::knock_09;

    #[test]
    fn it_works_positice() {
        let actual = knock_09("1");
        let expected = String::from("positive");
        assert_eq!(actual, expected);
    }
    #[test]

    fn it_works_zero() {
        let actual = knock_09("0");
        let expected = String::from("zero");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_negative() {
        let actual = knock_09("-1");
        let expected = String::from("negative");
        assert_eq!(actual, expected);
    }
}
