use crate::parse::parse_sep;

/// # No. 40 even or odd
///
/// 整数値を入力させ、その値が偶数ならばeven、奇数ならばoddと表示するプログラムを作成せよ。
pub fn knock_40(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_40 {
    use crate::knocks::knocks_40_49::knock_40;

    #[test]
    fn it_works_odd() {
        let actual = knock_40("3");
        let expected = String::from("odd");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_none_even() {
        let actual = knock_40("4");
        let expected = String::from("even");
        assert_eq!(actual, expected);
    }
}

/// # No. 41 1桁の自然数?
///
/// 整数値を入力させ、その値が一桁の自然数かそうでないか判定するプログラムを作成せよ。
pub fn knock_41(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests_41 {
    use crate::knocks::knocks_40_49::knock_41;

    #[test]
    fn it_works() {
        let actual = knock_41("3");
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_false() {
        let actual = knock_41("10");
        let expected = false;
        assert_eq!(actual, expected);
    }
}

/// # No. 42 小さい順?
///
/// 整数値を3つ入力させ、それらの値が小さい順（等しくてもよい）に並んでいるか判定し、
/// 小さい順に並んでいる場合はOK、そうなっていない場合はNGと表示するプログラムを作成せよ。
pub fn knock_42(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests_42 {
    use crate::knocks::knocks_40_49::knock_42;

    #[test]
    fn it_works() {
        let actual = knock_42("1 1 3");
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_false() {
        let actual = knock_42("2 1 3");
        let expected = false;
        assert_eq!(actual, expected);
    }
}

/// # No. 43 2次方程式の解の判別
///
/// 2次方程式 ax^2 + bx + c = 0 （x^2はxの2乗の意味）の係数a, b, cを入力し、
/// 2次方程式の解が2つの実数解か、重解か、2つの虚数解かを判別するプログラムを作成せよ。
pub fn knock_43(s: &str) -> Solution {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Solution {
    Real,      // 実数解
    Multiple,  // 重解
    Imaginary, // 虚数解
}

#[cfg(test)]
mod tests_43 {
    use crate::knocks::knocks_40_49::{knock_43, Solution};

    #[test]
    fn it_works_real() {
        let actual = knock_43("3 2 -4");
        let expected = Solution::Real;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_multiple() {
        let actual = knock_43("1 2 1");
        let expected = Solution::Multiple;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_imaginary() {
        let actual = knock_43("4 -3 1");
        let expected = Solution::Imaginary;
        assert_eq!(actual, expected);
    }
}

/// # No. 44 通貨換算
///
/// 換算したい金額（円単位）と1ドル何円かを整数値で入力すると、
/// 入力した金額が何ドル何セントか計算して表示するプログラムを作成せよ。
/// 1セントは1/100ドルである。結果は整数値でよい（1セント未満の端数切り捨て）。
pub fn knock_44(s: &str) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests_44 {
    use crate::knocks::knocks_40_49::knock_44;

    #[test]
    fn it_works1() {
        let actual = knock_44("10000 120");
        let expected = (83, 33);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_44("15000 125");
        let expected = (120, 0);
        assert_eq!(actual, expected);
    }
}

/// # No. 45 タクシー料金
///
/// 初乗り料金が1700mまで610円、それ以降は313mごとに80円のタクシーがある。
/// このタクシーに乗った距離をm単位で入力し、料金を計算するプログラムを作成せよ。
pub fn knock_45(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_45 {
    use crate::knocks::knocks_40_49::knock_45;

    #[test]
    fn it_works1() {
        let actual = knock_45("10000");
        let expected = 2770;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_45("2013");
        let expected = 690;
        assert_eq!(actual, expected);
    }
    #[test]
    fn it_works3() {
        let actual = knock_45("2014");
        let expected = 770;
        assert_eq!(actual, expected);
    }
}

/// # No. 46 入場料
///
/// 神山美術館の入場料金は、一人600円であるが、5人以上のグループなら一人550円、20人以上の団体なら一人500円である。
/// 人数を入力し、入場料の合計を計算するプログラムを作成せよ。
pub fn knock_46(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_46 {
    use crate::knocks::knocks_40_49::knock_46;

    #[test]
    fn it_works1() {
        let actual = knock_46("3");
        let expected = 1800;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_46("7");
        let expected = 3850;
        assert_eq!(actual, expected);
    }
    #[test]
    fn it_works3() {
        let actual = knock_46("22");
        let expected = 11000;
        assert_eq!(actual, expected);
    }
}

/// # No. 47 値の入れ替え
///
/// 異なる整数値を2つ入力し、それぞれ別の変数に格納する。
/// そして、それらの変数の値を入れ替えた後、それぞれの変数の値を表示するプログラムを作成せよ。
/// 単に順序を変えて表示するだけではダメ。
/// 必ず変数の値を入れ替えること。
/// 下の実行例の場合、まず変数aに2、bに5が入力された状態から、aの値が5、bの値が2になるように入れ替える。
#[allow(dead_code)]
pub fn knock_47() {
    // やらない
}

/// # No. 48 繰り返し計算
///
/// 最初に2以上の整数値を入力し、次の規則で計算し、計算回数と計算結果を表示し、
/// 計算結果が1になるまで繰り返すプログラムを作成せよ。
/// 規則：ある値が偶数ならその値を1/2にする。奇数ならその値を3倍して1を足す。
pub fn knock_48(s: &str) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests_48 {
    use crate::knocks::knocks_40_49::knock_48;

    #[test]
    fn it_works() {
        let actual = knock_48("3");
        let expected = [10, 5, 16, 8, 4, 2, 1];
        assert_eq!(actual, expected);
    }
}

/// # No. 49 九九
///
/// 九九の表を、2重の繰り返しを使って表示するプログラムを作成せよ。
/// 2重の繰り返しを使わず単に表示するだけではダメ。
/// 値の間はタブ(\t)を使って間をあけること。
pub fn knock_49() -> String {
    todo!()
}

#[cfg(test)]
mod tests_49 {
    use crate::knocks::knocks_40_49::knock_49;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works() {
        let actual = knock_49();
        let expected = read_resource("knock_49_expected.txt");
        assert_eq!(actual, expected);
    }
}
