use crate::knocks::knocks_50_59::knock_53;
use crate::parse::parse_sep;
use rand::prelude::*;

/// # No. 80 互いに素 ☆
///
/// 2つの正の整数値を入力させ、互いに素であるか判定するプログラムを作成せよ。
/// なお、2つの正の整数が互いに素とは、1以外に共通公約数を持たない関係のことである。
pub fn knock_80(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests_80 {
    use crate::knocks::knocks_80_89::knock_80;

    #[test]
    fn it_works_true() {
        let actual = knock_80("23 24");
        let expected = true;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_false() {
        let actual = knock_80("69 23");
        let expected = false;
        assert_eq!(actual, expected);
    }
}

/// # No. 81 中間値 ☆
///
/// ３つの整数値を入力させ、3つの値のうち2番目に大きい値を表示するプログラムを作成せよ。
pub fn knock_81(s: &str) -> isize {
    todo!()
}

#[cfg(test)]
mod tests_81 {
    use crate::knocks::knocks_80_89::knock_81;

    #[test]
    fn it_works1() {
        let actual = knock_81("23 24 25");
        let expected = 24;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_81("16 8 21");
        let expected = 16;
        assert_eq!(actual, expected);
    }
}

/// # No. 82 パスカルの三角形 ☆
///
/// パスカルの三角形とは、1段目は1のみ、2段目からは段の数だけ数字が並び、両端は1、その間は左上と右上の値を足した値、
/// として作っていった数字の並びである（Wikipedia「パスカルの三角形」参照）。
/// このパスカルの三角形を15段まで計算して表示するプログラムを作成せよ。
/// ただし表示は左詰で値はスペースで区切って表示するのでよい（三角形に並べなくてもよい）。
pub fn knock_82() -> String {
    todo!()
}

#[cfg(test)]
mod tests_82 {
    use crate::knocks::knocks_80_89::knock_82;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works() {
        let actual = knock_82();
        let expected = read_resource("knock_82_expected.txt");
        assert_eq!(actual, expected);
    }
}

/// # No. 83 じゃんけん5回勝負 ☆
///
/// 次の仕様のじゃんけんプログラムを作成せよ。
/// ・人間は、グー、チョキ、パーをそれぞれ0、1、2の数字で入力する。
/// ・コンピュータは乱数を使って出す手を選ぶ。乱数の使い方は演習資料の高度なテクニック集を見よ。
/// ・5回勝負として、人間とコンピュータの勝った回数を数え、勝敗がつくたびに1回ずつ表示する。
///   あいこは決着がつくまで再勝負。途中でどちらかが3勝しても、5回最後まで勝負を続ける。
/// ・指定された範囲以外の値を入力したら負けにする
///
///
/// **以下改変**
///
/// - 乱数を使うとテストが難しくなるため、コンピュータはグーチョキパー(0 -> 1 -> 2)を順々に出すこととする。
/// - 戻り値は (勝ち数, 負け数) とする。
pub fn knock_83(s: &str) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests_83 {
    use crate::knocks::knocks_80_89::knock_83;

    #[test]
    fn it_works() {
        let actual = knock_83("0 2 1 3 1 0 2");
        let expected = (2, 3);
        assert_eq!(actual, expected);
    }
}

/// # No. 84 トランプを切る ☆☆
///
/// トランプをランダムに順番を変えて表示するプログラムを作成せよ。
/// トランプは4つのスート（マーク）、1〜13までの数字の52枚である。
/// トランプを表現する配列を作り、適当に順序を入れ替えていけばよい。
/// 適当に順序を入れ替えるには、例えば2つの入れ替えるカードを乱数を使って選び、
/// それらを入れ替える操作を何回も繰り返せばよい。
pub fn knock_84() -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests_84 {
    use crate::knocks::knocks_80_89::knock_84;

    /// 配列の順序は乱数生成器に依存するため、以下を複数回チェックする
    /// - 配列の長さが 52 であること
    /// - 配列に同じ要素が含まれていないこと
    #[test]
    fn it_works() {
        for _ in 0..100 {
            let actual = knock_84();
            let expected = 52;
            assert_eq!(actual.len(), expected);

            let contains = actual.iter().all(|a| actual.contains(a));
            assert_eq!(contains, true);
        }
    }
}

/// # No. 85 石取りゲーム ☆☆
///
/// 最初に石の個数を入力し（10個以上とする）、二人のプレイヤーが交互に1〜3個ずつ石を取り、
/// 最後の1個を取った方が負けとなるゲームがある。
/// このゲームプログラムを作成せよ。具体的には実行例を参照のこと。
/// ※ 4 以上の数値を入力された場合、再入力
///
///
/// **以下改変**
/// - プレイヤー1 が勝ったら true, 負けたら false を返すようにする
pub fn knock_85(s: &str) -> bool {
    todo!()
}

#[cfg(test)]
mod tests_85 {
    use crate::knocks::knocks_80_89::knock_85;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works_lose() {
        let txt = read_resource("knock_85_1.txt");
        let actual = knock_85(&txt);
        let expected = false;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_win() {
        let txt = read_resource("knock_85_2.txt");
        let actual = knock_85(&txt);
        let expected = true;
        assert_eq!(actual, expected);
    }
}

/// # No. 86 コンピュータ必勝石取りゲーム ☆☆☆
///
/// No. 85の石取りゲームを人間とコンピュータが対戦する。
/// どちらから石を取り始めるかはコンピュータが決めてよいとしたとき、
/// コンピュータが必ず勝つプログラムを作成せよ。
///
///
/// **以下改変**
/// - 石の初期数とプレイヤー 1 の入力を引数として受け取り、コンピュータの入力を戻り値として返す
pub fn knock_86(s: &str) -> Vec<usize> {
    todo!()
}

/// FIXME 仮にコンピューターが勝利していなくてもテストを通せてしまう。微妙なテスト。
#[cfg(test)]
mod tests_86 {
    use crate::knocks::knocks_80_89::knock_86;

    #[test]
    fn it_works1() {
        let actual = knock_86("20 3 4 0 1 2 3");
        let expected = [3, 1, 3, 2, 1];
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_86("21 3 1 1 2 1");
        let expected = [1, 3, 3, 2, 3];
        assert_eq!(actual, expected);
    }
}

/// # No. 87 運命数 ☆☆
///
/// カバラ数秘術という簡単な占いがある。誕生日を年（西暦）・月・日で表し、それぞれの数字を足し合わせる。
/// 合計した数字が10以上であれば、再びすべての桁の数字を足し合わせる。
/// これを1桁の数字になるまで繰り返し、得られた数字を運命数とする。
/// ただし、計算途中で11、22、33のようにゾロ目の数字になった場合は、それを運命数とする。
/// 例：2015年12月23日→2+0+1+5+1+2+2+3=16→1+6=7
/// 運命数を計算するプログラムを作成せよ。
pub fn knock_87(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_87 {
    use crate::knocks::knocks_80_89::knock_87;

    #[test]
    fn it_works_7() {
        let actual = knock_87("20151223");
        let expected = 7;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_11() {
        let actual = knock_87("20151011");
        let expected = 11;
        assert_eq!(actual, expected);
    }
}

/// # No. 88 big or small ☆
///
/// コンピュータは1から99の数字をランダムに選ぶ（正解値と呼ぶ）。
/// プレイヤは値を入力し、正解値と一致すればクリアとなり値を入力した回数を表示する。
/// 一致しなければ正解値が入力値より大きいか小さいかを表示する。この数当てゲームプログラムを作成せよ。
///
///
/// **以下改変**
/// - 正解値は 14 に固定する
/// - プレイヤーの入力を引数とし、`Vec<Knock88Result>` を戻り値とする
pub fn knock_88(s: &str) -> Vec<Knock88Result> {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Knock88Result {
    Smaller,
    Bigger,
    /// Collect の場合は正解までにかかった手数を持たせる
    Collect(usize),
}

#[cfg(test)]
mod tests_88 {
    use crate::knocks::knocks_80_89::{knock_88, Knock88Result};

    #[test]
    fn it_works() {
        let actual = knock_88("50 25 13 19 16 14");
        let expected = [
            Knock88Result::Smaller,
            Knock88Result::Smaller,
            Knock88Result::Bigger,
            Knock88Result::Smaller,
            Knock88Result::Smaller,
            Knock88Result::Collect(6),
        ];
        assert_eq!(actual, expected);
    }
}

/// # No. 89 逆 big or small ☆☆☆
///
/// プレイヤは1から99の数字をランダムに選ぶ（正解値と呼ぶ）。
/// コンピュータは値を推測して表示する（推測値と呼ぶ）。
/// プレイヤは、推測値が正解値と一致していれば0、推測値よりも正解値が大きければ正の値、
/// 推測値よりも正解値が小さければ負の値を入力する。
/// 推測値が正解値と一致すればクリアとなり回数を表示して終了する。
/// プレイヤがズルをしない（正解値を変えたりしないなど）場合、
/// 多くとも7回までに必ずクリアできるプログラムを作成せよ。
///
///
/// **以下改変**
/// - コンピュータの推測回数を戻り値とする
pub fn knock_89(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_89 {
    use crate::knocks::knocks_80_89::knock_89;

    #[test]
    fn it_works1() {
        let actual = knock_89("52");
        assert_eq!(actual <= 7, true);
    }

    #[test]
    fn it_works2() {
        let actual = knock_89("10");
        assert_eq!(actual <= 7, true);
    }
}
