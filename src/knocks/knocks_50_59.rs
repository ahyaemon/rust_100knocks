use crate::parse::parse_sep;

/// # No. 50 foobar
///
/// 1から100までの値を繰り返しで表示するが、3の倍数の時はfoo、
/// 5の倍数の時はbarと数字の代わりに表示するプログラムを作成せよ。
/// なお、3と5の両方の倍数の時はfoobarと表示される。
pub fn knock_50() -> String {
    todo!()
}

#[cfg(test)]
mod tests_50 {
    use crate::knocks::knocks_50_59::knock_50;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works() {
        let actual = knock_50();
        let expected = read_resource("knock_50_expected.txt");
        assert_eq!(actual, expected);
    }
}

/// # No. 51 お支払い
///
/// 指定した金額を100円玉と10円玉と1円玉だけで、できるだけ少ない枚数で支払いたい。
/// 金額を入力するとそれぞれの枚数を計算して表示するプログラムを作成せよ。
pub fn knock_51(s: &str) -> (usize, usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests_51 {
    use crate::knocks::knocks_50_59::knock_51;

    #[test]
    fn it_works() {
        let actual = knock_51("12345");
        let expected = (123, 4, 5);
        assert_eq!(actual, expected);
    }
}

/// # No. 52 閏年
///
/// 西暦を入力したらその年が閏（うるう）年かどうかを判定するプログラムを作成せよ。
/// なお、4で割り切れる年のうち、100で割り切れないか、400で割り切れる年は閏年である。
pub fn knock_52(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests_52 {
    use crate::knocks::knocks_50_59::knock_52;

    #[test]
    fn it_works_2015() {
        let actual = knock_52("2015");
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_2016() {
        let actual = knock_52("2016");
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_2100() {
        let actual = knock_52("2100");
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_2000() {
        let actual = knock_52("2000");
        let expected = true;
        assert_eq!(actual, expected);
    }
}

/// # No. 53 素因数分解
///
/// 自然数の入力値を素因数分解して結果を表示するプログラムを作成せよ。
pub fn knock_53(s: &str) -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests_53 {
    use crate::knocks::knocks_50_59::knock_53;

    #[test]
    fn it_works() {
        let actual = knock_53("840");
        let expected = [2, 2, 2, 3, 5, 7];
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_one() {
        let actual = knock_53("24");
        let expected = [2, 2, 2, 3];
        assert_eq!(actual, expected);
    }
}

/// # No. 54 最大最小
///
/// まずデータの個数を入力させ、次にデータの個数だけ整数値を入力させる。
/// この入力データの中で最大値と最小値を求め表示するプログラムを作成せよ。
/// データの個数は100個までとする。
/// なお、データの個数とデータはファイルからリダイレクトで入力させればよいので、
/// 入力のためのメッセージは不要である（実行例を参照すること）。
pub fn knock_54(s: &str) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests_54 {
    use crate::knocks::knocks_50_59::knock_54;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works_small() {
        let txt = read_resource("knock_54_small.txt");
        let actual = knock_54(&txt);
        let expected = (128, 962);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_middle() {
        let txt = read_resource("knock_54_middle.txt");
        let actual = knock_54(&txt);
        let expected = (20, 988);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_large() {
        let txt = read_resource("knock_54_large.txt");
        let actual = knock_54(&txt);
        let expected = (5, 996);
        assert_eq!(actual, expected);
    }
}

/// # No. 55 夢想花again
///
/// 「とんで」を9回「まわって」を3回繰り返した後「まわる」と表示して改行する、を3回繰り返すプログラムを作成せよ。
/// 「とんで」「まわって」と3行文の繰り返しは必ず繰り返し構文を使うこと。
pub fn knock_55() -> String {
    todo!()
}

#[cfg(test)]
mod tests_55 {
    use crate::knocks::knocks_50_59::knock_55;

    #[test]
    fn it_works() {
        let actual = knock_55();
        let expected =
            "とんでとんでとんでとんでとんでとんでとんでとんでとんでまわってまわってまわってまわる
とんでとんでとんでとんでとんでとんでとんでとんでとんでまわってまわってまわってまわる
とんでとんでとんでとんでとんでとんでとんでとんでとんでまわってまわってまわってまわる";
        assert_eq!(actual, expected);
    }
}

/// # No. 56 2進数変換
///
/// 0〜65535の整数値を入力させ、入力値を16桁の2進数に変換して表示するプログラムを作成せよ。
pub fn knock_56(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_56 {
    use crate::knocks::knocks_50_59::knock_56;

    #[test]
    fn it_works1() {
        let actual = knock_56("127");
        let expected = String::from("0000000001111111");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_56("10000");
        let expected = String::from("0010011100010000");
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works3() {
        let actual = knock_56("65535");
        let expected = String::from("1111111111111111");
        assert_eq!(actual, expected);
    }
}

/// # No. 57 テスト集計
///
/// まず受験者数を入力させ、次に受験者数ごとに英語、数学、国語の点数をスペースで区切って入力させる
/// （具体的な入力方法は下記のscanfの使い方の説明、および入力データの中身を見よ）。
/// 入力が終了したら、英語、数学、国語の平均点、および各受験生の合計点を計算して表示するプログラムを作成せよ。
/// 受験者数は100人までとする。
/// なお、データの個数とデータはファイルからリダイレクトで入力させればよいので、
/// 入力のためのメッセージは不要である（実行例を参照すること）。
///
///
/// **以下改変**
/// - 戻り値: ((英語の平均点, 数学の平均点, 国語の平均点), 各受験生の合計点のベクター)
pub fn knock_57(s: &str) -> ((usize, usize, usize), Vec<usize>) {
    todo!()
}

#[cfg(test)]
mod tests_57 {
    use crate::knocks::knocks_50_59::knock_57;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works_small() {
        let txt = read_resource("knock_57_small.txt");
        let actual = knock_57(&txt);
        assert_eq!(actual.0, (46, 51, 55));
        assert_eq!(actual.1[0], 141);
        assert_eq!(actual.1[1], 114);
        assert_eq!(actual.1[8], 96);
        assert_eq!(actual.1[9], 188);
    }

    #[test]
    fn it_works_middle() {
        let txt = read_resource("knock_57_middle.txt");
        let actual = knock_57(&txt);
        assert_eq!(actual.0, (55, 53, 54));
        assert_eq!(actual.1[0], 136);
        assert_eq!(actual.1[1], 64);
        assert_eq!(actual.1[48], 265);
        assert_eq!(actual.1[49], 167);
    }

    #[test]
    fn it_works_large() {
        let txt = read_resource("knock_57_large.txt");
        let actual = knock_57(&txt);
        assert_eq!(actual.0, (52, 51, 51));
        assert_eq!(actual.1[0], 151);
        assert_eq!(actual.1[1], 241);
        assert_eq!(actual.1[98], 107);
        assert_eq!(actual.1[99], 178);
    }
}

/// # No. 58 棒グラフ
///
/// 0以上の整数値を5つ入力させ、それぞれの値に対して、次の形式でグラフを描くプログラムを作成せよ。
/// 形式：左端に値を表示し、適切に空白を空けて":"を書く（:で揃えるためにタブ\tを使うとよい）。
/// その後ろに値の数だけ*を描くが、5個おきに空白を１つ入れる。具体例は下記の実行例を参照すること。
pub fn knock_58(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_58 {
    use crate::knocks::knocks_50_59::knock_58;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works() {
        let actual = knock_58("7 10 14 15 21");
        let expected = read_resource("knock_58_expected.txt");
        assert_eq!(actual, expected);
    }
}

/// # No. 59 行列の和
///
/// 3x3行列の和を求めて表示するプログラムを作成せよ。
/// 行列の値は2次元配列で表現し、繰り返しを使って計算すること。
/// 3x3行列とは縦3つ、横3つの9つの要素(値)をひとまとめにして扱うものである。
/// 2つの3x3行列の和は次式のように、それぞれ同じ位置にある値を足したものとして計算できる。
///
/// 例えばa12という要素は、1行目2列目の要素という意味である。それぞれ同じ位置にある要素を足せばよい。
/// なお、入力値は1行ずつ3つの値をスペースで区切って入力するようにするとよい。
/// このためには、scanf("%d %d %d", &a[0][0], &a[0][1], &a[0][2]);のように書く(No. 57参照)。
pub fn knock_59(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_59 {
    use crate::knocks::knocks_50_59::knock_59;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works() {
        let txt = read_resource("knock_59.txt");
        let actual = knock_59(&txt);
        let expected = read_resource("knock_59_expected.txt");
        assert_eq!(actual, expected);
    }
}
