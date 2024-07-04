use crate::parse::parse_sep;

/// # No. 20 割って掛ける
///
/// 整数値を2つ入力させ、1つめの値を2つめの値で割った結果を表示し、
/// 続けてその結果に2つめの値を掛けた結果を表示するプログラムを作成せよ。
/// 計算はすべて整数型で行うこと（割り切れない場合は自動的に小数点以下が切り捨てられる）。
///
/// ---
///
/// - 引数: 整数値 2 つをスペースで区切った文字列参照
/// - 戻り値: (1つめの値を2つめの値で割った結果, その結果に2つめの値を掛けた結果)
pub fn knock_20(s: &str) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests_20 {
    use crate::knocks::knocks_20_29::knock_20;

    #[test]
    fn it_works() {
        let actual = knock_20("10 3");
        let expected = (3, 9);
        assert_eq!(actual, expected);
    }
}

/// # No. 21 5より大きく20より小さい
///
/// 整数値を入力させ、その値が5よりも大きく、かつ、20よりも小さければOKと表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `Option<String>`
///   - 入力値が 5 より大きく 20 より小さければ `Some(String)`
///   - それ以外は None
pub fn knock_21(s: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests_21 {
    use crate::knocks::knocks_20_29::knock_21;

    #[test]
    fn it_works_some_6() {
        let actual = knock_21("6");
        let expected = Some(String::from("OK"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_some_19() {
        let actual = knock_21("19");
        let expected = Some(String::from("OK"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none_5() {
        let actual = knock_21("5");
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none_20() {
        let actual = knock_21("20");
        let expected = None;
        assert_eq!(actual, expected);
    }
}

/// # No. 22 -10以下または10以上
///
/// 整数値を入力させ、その値が-10以下、または、10以上であればOKと表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `Option<String>`
///   - 入力値が -10　以下または 10 以上であれば `Some(String)`
///   - それ以外は None
pub fn knock_22(s: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests_22 {
    use crate::knocks::knocks_20_29::knock_22;

    #[test]
    fn it_works_some_minus_10() {
        let actual = knock_22("-10");
        let expected = Some(String::from("OK"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_some_10() {
        let actual = knock_22("10");
        let expected = Some(String::from("OK"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none_minus_9() {
        let actual = knock_22("-9");
        let expected = None;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none_9() {
        let actual = knock_22("9");
        let expected = None;
        assert_eq!(actual, expected);
    }
}

/// # No. 23 -5以上10未満
///
/// 整数値を入力させ、その値が-5以上10未満であればOK、そうでなければNGと表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `String`
///   - 入力値が -5 以上 10 未満であれば `"OK"`
///   - それ以外は `"NG"`
pub fn knock_23(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_23 {
    use crate::knocks::knocks_20_29::knock_23;

    #[test]
    fn it_works_ok_minus_5() {
        let actual = knock_23("-5");
        let expected = String::from("OK");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ok_9() {
        let actual = knock_23("9");
        let expected = String::from("OK");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ng_minus_6() {
        let actual = knock_23("-6");
        let expected = String::from("NG");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ng_10() {
        let actual = knock_23("10");
        let expected = String::from("NG");
        assert_eq!(actual, expected);
    }
}

/// # No. 24 -10以上0未満、または、10以上
///
/// 整数値を入力させ、その値が-10以上0未満、または、10以上であればOK、
/// そうでなければNGと表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `String`
///   - 入力値が -10 以上 0 未満、または、10 以上であれば `"OK"`
///   - それ以外は `"NG"`
pub fn knock_24(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_24 {
    use crate::knocks::knocks_20_29::knock_24;

    #[test]
    fn it_works_ok_minus_10() {
        let actual = knock_24("-10");
        let expected = String::from("OK");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ok_minus_1() {
        let actual = knock_24("-1");
        let expected = String::from("OK");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ok_10() {
        let actual = knock_24("10");
        let expected = String::from("OK");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ng_minus_11() {
        let actual = knock_24("-11");
        let expected = String::from("NG");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ng_0() {
        let actual = knock_24("0");
        let expected = String::from("NG");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_ng_9() {
        let actual = knock_24("9");
        let expected = String::from("NG");
        assert_eq!(actual, expected);
    }
}

/// # No. 25 -10未満?、-10以上0未満?、0以上?
///
/// 整数値を入力させ、その値が-10未満ならrange 1、
/// -10以上0未満であればrange 2、
/// 0以上であればrange 3、と表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `String`
///   - 入力値が -10 未満なら `"range 1"`
///   - -10 以上 0 未満であれば `"range 2"`
///   - 0 以上であれば `"range 3"`
pub fn knock_25(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_25 {
    use crate::knocks::knocks_20_29::knock_25;

    #[test]
    fn it_works_range1() {
        let actual = knock_25("-11");
        let expected = String::from("range1");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_range2_min() {
        let actual = knock_25("-10");
        let expected = String::from("range2");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_range2_max() {
        let actual = knock_25("-1");
        let expected = String::from("range2");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_range3() {
        let actual = knock_25("0");
        let expected = String::from("range3");
        assert_eq!(actual, expected);
    }
}

/// # No. 26 switch-case
///
/// 整数値を入力させ、その値が1ならone、2ならtwo、3ならthree、
/// それ以外ならothersと表示するプログラムをswitch-case文を使って作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `String`
///   - 入力値が -10 未満なら `"range 1"`
///   - -10 以上 0 未満であれば `"range 2"`
///   - 0 以上であれば `"range 3"`
pub fn knock_26(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_26 {
    use crate::knocks::knocks_20_29::knock_26;

    #[test]
    fn it_works_one() {
        let actual = knock_26("1");
        let expected = String::from("one");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_work_two() {
        let actual = knock_26("2");
        let expected = String::from("two");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_three() {
        let actual = knock_26("3");
        let expected = String::from("three");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_others() {
        let actual = knock_26("4");
        let expected = String::from("others");
        assert_eq!(actual, expected);
    }
}

/// # No. 27 1からnまでの和
///
/// 整数値を入力させ、1からその値までの総和を計算して表示するプログラムを作成せよ。
/// ただし、0以下の値を入力した場合は0と表示する。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 1 から入力値までの総和
pub fn knock_27(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_27 {
    use crate::knocks::knocks_20_29::knock_27;

    #[test]
    fn it_works() {
        let actual = knock_27("10");
        let expected = 55;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_work_minus() {
        let actual = knock_27("-1");
        let expected = 0;
        assert_eq!(actual, expected);
    }
}

/// # No. 28 nの階乗
///
/// 整数値を入力させ、その値の階乗を表示するプログラムを作成せよ。
/// ただし、0以下の値を入力した場合は1と表示する。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 入力値の階乗
pub fn knock_28(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_28 {
    use crate::knocks::knocks_20_29::knock_28;

    #[test]
    fn it_works() {
        let actual = knock_28("10");
        let expected = 3628800;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_work_minus() {
        let actual = knock_28("-2");
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

/// # No. 29 5つの合計
///
/// 整数値を5回入力させ、それらの値の合計を表示するプログラムを繰り返しを使って作成せよ。
///
/// ---
///
/// - 引数: 整数値 5 つをスペースで区切った文字列参照
/// - 戻り値: 入力値の合計
pub fn knock_29(s: &str) -> isize {
    todo!()
}

#[cfg(test)]
mod tests_29 {
    use crate::knocks::knocks_20_29::knock_29;

    #[test]
    fn it_works() {
        let actual = knock_29("11 22 33 44 55");
        let expected = 165;
        assert_eq!(actual, expected);
    }
}
