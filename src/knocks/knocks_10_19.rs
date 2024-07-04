use std::iter;

/// # No. 10 絶対値
///
/// 整数値を入力させ、その値を絶対値にして表示するプログラムを作成せよ。（できれば変数の値を絶対値に変えるようにせよ）
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 絶対値
pub fn knock_10(a: &str) -> isize {
    todo!()
}

#[cfg(test)]
mod tests_10 {
    use crate::knocks::knocks_10_19::knock_10;

    #[test]
    fn it_works_positive() {
        let actual = knock_10("1");
        let expected = 1;
        assert_eq!(actual, expected);
    }
    #[test]
    fn it_works_zero() {
        let actual = knock_10("0");
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_negative() {
        let actual = knock_10("-1");
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

/// # No. 11 ごあいさつ10回
///
/// Hello World!を10回繰り返して表示するプログラムを作成せよ。
///
/// ---
///
/// - 戻り値: `"Hello World!"` を改行区切りで 10 個繋いだ文字列
pub fn knock_11() -> String {
    todo!()
}

#[cfg(test)]
mod tests_11 {
    use crate::knocks::knocks_10_19::knock_11;

    #[test]
    fn it_works() {
        let actual = knock_11();
        let expected = String::from(
            "Hello World!
Hello World!
Hello World!
Hello World!
Hello World!
Hello World!
Hello World!
Hello World!
Hello World!
Hello World!",
        );
        assert_eq!(actual, expected);
    }
}

/// # No. 12 ごあいさつ指定回
///
/// 整数値を入力させ、その値の回数だけHello World!を繰り返して表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: `"Hello World!"` を引数分改行区切りで繋いだ文字列
pub fn knock_12(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_12 {
    use crate::knocks::knocks_10_19::knock_12;

    #[test]
    fn it_works() {
        let actual = knock_12("3");
        let expected = String::from(
            "Hello World!
Hello World!
Hello World!",
        );
        assert_eq!(actual, expected);
    }
}

/// # No. 13 カウントアップ
///
/// 整数値を入力させ、0から入力値まで数を1ずつ増やして表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 0 から入力値までを改行区切りで繋いだ文字列
pub fn knock_13(i: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_13 {
    use crate::knocks::knocks_10_19::knock_13;

    #[test]
    fn it_works() {
        let actual = knock_13("3");
        let expected = String::from(
            "0
1
2
3",
        );
        assert_eq!(actual, expected);
    }
}

/// # No. 14 カウントダウン
///
/// 整数値を入力させ、入力値から0まで数を1ずつ減らして表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 入力値から 0 までを改行区切りで繋いだ文字列
pub fn knock_14(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_14 {
    use crate::knocks::knocks_10_19::knock_14;

    #[test]
    fn it_works() {
        let actual = knock_14("3");
        let expected = String::from(
            "3
2
1
0",
        );
        assert_eq!(actual, expected);
    }
}

/// # No. 15 2ずつカウントアップ
///
/// 整数値を入力させ、0から入力値を超えない値まで2ずつ増やして表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 0 から入力値を超えない値まで2ずつ増やして改行区切りで繋いだ文字列
pub fn knock_15(i: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_15 {
    use crate::knocks::knocks_10_19::knock_15;

    #[test]
    fn it_works() {
        let actual = knock_15("7");
        let expected = String::from(
            "0
2
4
6",
        );
        assert_eq!(actual, expected);
    }
}

/// # No. 16 0でおしまい
///
/// 整数値を入力させ、入力値が0でなければ再度入力させ、0であれば終了するプログラムを作成せよ。
#[allow(dead_code)]
pub fn knock_16() {
    // やらない
}

/// # No. 17 配列を初期化
///
/// 要素数10の整数型の配列を宣言し、i番目の要素の初期値をiとし、順に値を表示するプログラムを作成せよ。
///
/// ---
///
/// - 戻り値: i 番目の要素の初期値が i となる要素数 10 の配列
pub fn knock_17() -> [usize; 10] {
    todo!()
}

#[cfg(test)]
mod tests_17 {
    use crate::knocks::knocks_10_19::knock_17;

    #[test]
    fn it_works() {
        let actual = knock_17();
        let expected = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];
        assert_eq!(actual, expected);
    }
}

/// # No. 18 配列を入力値で初期化
///
/// 要素数10の整数型の配列を宣言し、整数値を入力させ、すべての配列の要素を入力値として、
/// すべての要素の値を表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値の文字列参照
/// - 戻り値: 全ての要素が入力値である要素数 10 の配列
pub fn knock_18(s: &str) -> [usize; 10] {
    todo!()
}

#[cfg(test)]
mod tests_18 {
    use crate::knocks::knocks_10_19::knock_18;

    #[test]
    fn it_works() {
        let actual = knock_18("6");
        let expected = [6, 6, 6, 6, 6, 6, 6, 6, 6, 6];
        assert_eq!(actual, expected);
    }
}

/// # No. 19 配列に入力値を格納
///
/// 要素数5の整数型の配列を宣言し、すべての配列に対して順に入力された整数値を代入し、
/// すべての要素の値を表示するプログラムを作成せよ。
///
/// ---
///
/// - 引数: 整数値 5 つをスペースで区切った文字列参照
/// - 戻り値: 入力値を順に要素に持つサイズ 5 の配列
pub fn knock_19(s: &str) -> [usize; 5] {
    todo!()
}

#[cfg(test)]
mod tests_19 {
    use crate::knocks::knocks_10_19::knock_19;

    #[test]
    fn it_works() {
        let actual = knock_19("4 6 7 3 1");
        let expected = [4, 6, 7, 3, 1];
        assert_eq!(actual, expected);
    }
}
