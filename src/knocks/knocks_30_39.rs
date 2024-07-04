use crate::parse::parse_sep;

/// # No. 30 棒グラフ
///
/// 整数値を入力させ、その個数だけ*を表示するプログラムを作成せよ。
/// 入力値が0以下の値の場合は何も書かなくてよい。
pub fn knock_30(s: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests_30 {
    use crate::knocks::knocks_30_39::knock_30;

    #[test]
    fn it_works() {
        let actual = knock_30("3");
        let expected = Some(String::from("***"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none() {
        let actual = knock_30("-1");
        let expected = None;
        assert_eq!(actual, expected);
    }
}

/// # No. 31 棒グラフ改
///
/// 整数値を入力させ、その個数だけ*を、5個おきに空白（スペース）を入れて表示するプログラムを作成せよ。
/// 入力値が0以下の値の場合は何も書かなくてよい。
pub fn knock_31(s: &str) -> Option<String> {
    todo!()
}

#[cfg(test)]
mod tests_31 {
    use crate::knocks::knocks_30_39::knock_31;

    #[test]
    fn it_works() {
        let actual = knock_31("13");
        let expected = Some(String::from("***** ***** ***"));
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none() {
        let actual = knock_31("-1");
        let expected = None;
        assert_eq!(actual, expected);
    }
}

/// # No. 32 5の倍数でbar
///
/// 1から20まで順に表示するが、5の倍数の場合は数字の代わりにbarと表示するプログラムを作成せよ。
pub fn knock_32(s: &str) -> Vec<String> {
    todo!()
}

#[cfg(test)]
mod tests_32 {
    use crate::knocks::knocks_30_39::knock_32;

    #[test]
    fn it_works() {
        let actual = knock_32("11");
        let expected = [
            String::from("1"),
            String::from("2"),
            String::from("3"),
            String::from("4"),
            String::from("bar"),
            String::from("6"),
            String::from("7"),
            String::from("8"),
            String::from("9"),
            String::from("bar"),
            String::from("11"),
        ];
        assert_eq!(actual, expected);
    }
}

/// # No. 33 入力値抜き
///
/// 整数値を入力させ、1から9まで、入力値以外を表示するプログラムを作成せよ。
pub fn knock_33(s: &str) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests_33 {
    use crate::knocks::knocks_30_39::knock_33;

    #[test]
    fn it_works() {
        let actual = knock_33("7");
        let expected = [1, 2, 3, 4, 5, 6, 8, 9];
        assert_eq!(actual, expected);
    }
}

/// # No. 34 入力値抜き改
///
/// 整数値を入力させ、1から9まで、入力値と入力値+1以外を表示するプログラムを作成せよ。
/// 入力値が9の場合は9のみ表示しない。
pub fn knock_34(s: &str) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests_34 {
    use crate::knocks::knocks_30_39::knock_34;

    #[test]
    fn it_works() {
        let actual = knock_34("7");
        let expected = [1, 2, 3, 4, 5, 6, 9];
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_9() {
        let actual = knock_34("9");
        let expected = [1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(actual, expected);
    }
}

/// # No. 35 配列要素の参照
///
/// {3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
/// 整数値を入力させ、要素番号が入力値である配列要素の値を表示するプログラムを作成せよ。
/// 入力値が配列の要素の範囲外であるかどうかのチェックは省略してよい。
pub fn knock_35(s: &str) -> u8 {
    todo!()
}

#[cfg(test)]
mod tests_35 {
    use crate::knocks::knocks_30_39::knock_35;

    #[test]
    fn it_works() {
        let actual = knock_35("5");
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

/// # No. 36 続・配列要素の参照
///
/// {3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
/// 整数値を2つ入力させ、要素番号が入力値である2つの配列要素の値の積を計算して表示するプログラムを作成せよ。
/// 入力値が配列の要素の範囲外であるかどうかのチェックは省略してよい。
pub fn knock_36(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_36 {
    use crate::knocks::knocks_30_39::knock_36;

    #[test]
    fn it_works() {
        let actual = knock_36("3 6");
        let expected = 72;
        assert_eq!(actual, expected);
    }
}

/// # No. 37 続々・配列要素の参照
///
/// {3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
/// 整数値を入力させ、要素番号が入力値の配列要素の値を参照し、
/// さらにその参照した値を要素番号とする配列要素の値を参照して表示するプログラムを作成せよ。
/// 入力値が配列の要素の範囲外であるかどうかのチェックは省略してよい。
pub fn knock_37(s: &str) -> u8 {
    todo!()
}

#[cfg(test)]
mod tests_37 {
    use crate::knocks::knocks_30_39::knock_37;

    #[test]
    fn it_works() {
        let actual = knock_37("8");
        let expected = 1;
        assert_eq!(actual, expected);
    }
}

/// # No. 38 さらに・配列要素の参照
///
/// {3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
/// 最初は参照する要素番号を0とし、この参照する要素番号の配列要素の値を表示し、
/// 次にその配列要素の値を次の参照する要素番号とし、
/// この次の参照する要素番号の配列要素の値を表示し、
/// さらにその配列要素の値を次の参照する要素番号とし、
/// ……を10回繰り返すプログラムを作成せよ。
/// （具体的にどのような手順かは実行例を見て考えよう。）
pub fn knock_38() -> [u8; 10] {
    todo!()
}

#[cfg(test)]
mod tests_38 {
    use crate::knocks::knocks_30_39::knock_38;

    #[test]
    fn it_works() {
        let actual = knock_38();
        let expected = [3, 8, 5, 1, 7, 6, 9, 2, 0, 3];
        assert_eq!(actual, expected);
    }
}

/// # No. 39 もっと・配列要素の参照
///
/// {3, 7, 0, 8, 4, 1, 9, 6, 5, 2}で初期化される大きさ10の整数型配列を宣言し、
/// 最初は参照する要素番号を0とする。
/// この参照する要素番号の配列要素の値から次の要素番号の配列要素の値を引いた値を表示し、
/// 参照する要素番号を1増やす。この手順を9回繰り返すプログラムを作成せよ。
/// （具体的にどのような手順かは実行例を見て考えよう。）
pub fn knock_39() -> [i8; 9] {
    todo!()
}

#[cfg(test)]
mod tests_39 {
    use crate::knocks::knocks_30_39::knock_39;

    #[test]
    fn it_works() {
        let actual = knock_39();
        let expected = [-4, 7, -8, 4, 3, -8, 3, 1, 3];
        assert_eq!(actual, expected);
    }
}
