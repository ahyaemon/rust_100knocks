use crate::knocks::knocks_80_89::knock_84;
use crate::parse::parse_sep;
use rand::prelude::*;

/// # No. 90 ブラックジャックその1 ☆☆
///
/// トランプをシャッフルするプログラムはNo.84で作成した。
/// これを使って、ブラックジャックゲームを作ってみよう。
/// ブラックジャックは、最初に2枚のカードを配り、2〜10は数字通り、
/// JとQとKは10、Aは1か11として合計し21に近いほど勝ちとなるが、
/// 21を超えると負け（バストと呼ぶ）というゲームである。
/// まず最初にトランプをシャッフルし、2枚を先頭から順番に引き、
/// それらのカードの数字（マークは無視してよい）と合計値を表示するプログラムを作成せよ。
///
///
/// **以下改変**
/// - 戻り値: (1 枚目のカードの数字, 2 枚目のカードの数字, 合計値)
pub fn knock_90() -> (usize, usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests_90 {
    use crate::knocks::knocks_90_99::knock_90;

    /// カードの数字は乱数生成器に依存するため、以下を複数回チェックする
    /// - 数値が 1 ~ 13 であること
    /// - 1 枚目と 2 枚目の合計が正しいこと
    #[test]
    fn it_works() {
        for _ in 0..100 {
            let actual = knock_90();
            assert_eq!(actual.2, actual.0 + actual.1);
            assert_eq!(actual.0 >= 1, true);
            assert_eq!(actual.0 <= 13, true);
            assert_eq!(actual.1 >= 1, true);
            assert_eq!(actual.1 <= 13, true);
        }
    }
}

/// # No. 91 ブラックジャックその2 ☆☆☆☆
///
/// No.90で作ったブラックジャックゲームをさらに進化させよう。
/// ブラックジャックでは、最初に配られた2枚の合計では足りない場合、
/// 21を超えない限り何枚でもカードを追加で引くことができる。
/// 合計が16以下の場合はもう一枚カードを引き、17以上になったら止める、
/// というルールで自動的にカードを引くプログラムを作成せよ。
///
///
/// **以下改変**
/// - 戻り値: Vec<引いたカード>
pub fn knock_91() -> Vec<usize> {
    todo!()
}

#[cfg(test)]
mod tests_91 {
    use crate::knocks::knocks_90_99::knock_91;

    /// カードの数字は乱数生成器に依存するため、以下を複数回チェックする
    /// - 引いたカードの合計値が 17 以上であること
    /// - 二枚目までで 17 以上になっている場合、それ以上カードを引いていないこと
    #[test]
    fn it_works() {
        for _ in 0..100 {
            let actual = knock_91();
            assert_eq!(actual.iter().sum::<usize>() >= 17, true);
        }
    }
}

/// # No. 92 世界の人 ☆
///
/// 1から50まで順に表示していくが、3の倍数と3のつく数字のときは頭に"aho"と付けて表示するプログラムを作成せよ。
pub fn knock_92() -> String {
    todo!()
}

#[cfg(test)]
mod tests_92 {
    use crate::knocks::knocks_90_99::knock_92;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works() {
        let actual = knock_92();
        let expected = read_resource("knock_92_expected.txt");
        assert_eq!(actual, expected);
    }
}

/// # No. 93 宇宙の人 ☆☆☆
///
/// まず開始値と終了値をそれぞれ入力させる。
/// 次に、開始値から終了値まで順に、3の倍数と3のつく数字のときは頭に"aho"と付けて表示するプログラムを作成せよ。
/// 開始値と終了値の値の妥当性（例えば終了値の方が開始値よりも大きいか）チェックは省略してよい。
pub fn knock_93(s: &str) -> String {
    todo!()
}

#[cfg(test)]
mod tests_93 {
    use crate::knocks::knocks_90_99::knock_93;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works() {
        let actual = knock_93("290 310");
        let expected = read_resource("knock_93_expected.txt");
        assert_eq!(actual, expected);
    }
}

/// # No. 94 hit and blow その1 ☆☆☆
///
/// hit and blowという数当てゲームがある。
/// 出題者は0〜9999の範囲の数字を正解として選ぶ。
/// 次に、解答者は予想する数字を言う。
/// 出題者は同じ桁（位置）に同じ数字があればヒット、桁は違うが同じ数字があればブローとして数え、
/// ヒットとブローの数を答える（3桁以下の数字は頭に0が付いているものとする）。
/// 例えば、次のようになる：
///   正解:1234 予想:1234 → 4ヒット（＝クリア）
///   正解:1234 予想:5678 → 0ヒット0ブロー
///   正解:1234 予想:1892 → 1ヒット1ブロー
///   正解:0034 予想:3400 → 0ヒット4ブロー
///   正解:1222 予想:1234 → 2ヒット0ブロー（ヒットはブローより優先して判定する）
///   正解:1112 予想:1221 → 1ヒット2ブロー
///
/// このゲームを3回に分けて作ってみよう。
/// まず、コンピュータは正解となる4つの数字をランダムに選ぶ（同じ数字を何回使ってもよいし、0で始まってもよい）。
/// 次に、プレイヤーに4桁の数字を入力させる。そして、ヒットの数を数え、表示するプログラムを作成せよ。
///
///
/// **以下改変**
/// - 引数は "{答え} {プレイヤーの入力}" とする
/// - 戻り値はヒット数
pub fn knock_94(s: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests_94 {
    use crate::knocks::knocks_90_99::knock_94;

    #[test]
    fn it_works1() {
        let actual = knock_94("7107 1234");
        let expected = 0;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_94("9834 1234");
        let expected = 2;
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works3() {
        let actual = knock_94("0048 0000");
        let expected = 2;
        assert_eq!(actual, expected);
    }
}

/// # No. 95 hit and blow その2 ☆☆☆☆☆
///
/// No.94に続いて、ブローの数も数えて表示するプログラムを作成せよ。
/// （No.94のプログラムをコピーして、それに追加するとよい。）
/// ヒント：ヒットはブローよりも優先されるので、まずヒットのチェックを行う。
/// この時、ヒットした数字は再びブローでチェックしてはいけないことに注意。
/// また、ブローをチェックする際にも、一度ブローと判定した数字はチェックしてはならない（正解、予想値ともに）。
///
///
/// **以下改変**
/// ※ 引数は "{答え} {プレイヤーの入力}" とする
/// ※ 戻り値は (ヒット数, ブロー数)
pub fn knock_95(s: &str) -> (usize, usize) {
    todo!()
}

#[cfg(test)]
mod tests_95 {
    use crate::knocks::knocks_90_99::knock_95;

    #[test]
    fn it_works1() {
        let actual = knock_95("3295 1234");
        let expected = (1, 1);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works2() {
        let actual = knock_95("3318 1234");
        let expected = (0, 2);
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works3() {
        let actual = knock_95("0210 0012");
        let expected = (2, 2);
        assert_eq!(actual, expected);
    }
}

/// # No. 96 hit and blow その3 ☆☆☆☆☆
///
/// No.95に続いて、正解と一致するまで繰り返して予想させるようにしてゲームを完成させよう。
///
///
/// **以下改変**
/// - 引数は正解値のスペースの後、スペース区切りのプレイヤーの入力
pub fn knock_96(s: &str) -> Vec<Knock96Result> {
    todo!()
}

#[derive(Debug, PartialEq)]
pub enum Knock96Result {
    /// 正解した場合
    Collect,
    /// 不正解の場合、ヒット数とブロー数を持たせる
    Wrong { hit: usize, blow: usize },
}

#[cfg(test)]
mod tests_96 {
    use crate::knocks::knocks_90_99::{knock_96, Knock96Result};

    #[test]
    fn it_works() {
        let actual = knock_96("3169 1234 4529 3169");
        let expected = vec![
            Knock96Result::Wrong { hit: 0, blow: 2 },
            Knock96Result::Wrong { hit: 1, blow: 0 },
            Knock96Result::Collect,
        ];
        assert_eq!(actual, expected);
    }
}

/// # ビンゴその1 ☆☆☆
///
/// ビンゴゲームのカードを作るプログラムを作ろう。
/// 縦5マス、横5マス、計25マスのそれぞれに1〜75までの数字をランダムに配置する。
/// 同じ番号は2回以上配置しない。
/// 作成したカードは、実行例のようにタブ(\t)を開けて表示すること。
///
/// (一般的なビンゴカードは、縦の5列それぞれは1〜15、16〜30、31〜45、46〜60、
/// 61〜75のそれぞれの数字のうち5つずつを配置し、中央のマスはフリーとしているが、
/// ここでは簡単のためにこのようにしている。
/// 余裕があれば一般的なカードを作ってるとよいだろう)
///
///
/// **以下改変**
/// - 戻り値は String ではなく、`Vec<Row>` とする
pub fn knock_97() -> Vec<Knock97Row> {
    todo!()
}

/// 列に含まれれる数字の Vector
type Knock97Row = Vec<usize>;

#[cfg(test)]
mod tests_97 {
    use crate::knocks::knocks_90_99::knock_97;

    /// 具体的な数値は乱数生成器に依存する為、以下をテストする
    /// - 同じ数値が含まれないこと
    /// - 5 * 5 の行列になっていること
    #[test]
    fn it_works() {
        let actual = knock_97();

        // 行が 5 つ
        assert_eq!(actual.len(), 5);

        // 各行に対して列が 5 つ
        for row in &actual {
            assert_eq!(row.len(), 5);
        }

        // 全ての数値が異なる
        // -> 二重配列をフラットにして、各要素が 1 つしか存在しないことを確認
        // FIXME もっと良い確認方法がありそう
        let flatten: Vec<usize> = actual.iter().flatten().map(|u| *u).collect();
        for i in &flatten {
            let amount = flatten
                .iter()
                .filter(|a| **a == *i)
                .collect::<Vec<&usize>>()
                .len();
            assert_eq!(amount, 1);
        }
    }
}

/// No. 98 ビンゴその2 ☆☆☆☆☆
///
/// No.97で作ったビンゴカードで、ビンゴの抽選をするプログラムを作ろう。
/// 1〜75の数字をランダムな順番で1つずつ選んでいく。
/// 選んだ数字がカードにあれば、その数字のマス目を開ける。
/// 空いたマス目が縦横斜めのいずれかに5つ並べばビンゴとなり終了する。
/// 抽選の経過がわかるように表示を工夫しよう。
///
///
/// **以下改変**
/// - ビンゴカードの数値の並びは固定とする
/// - 引数: ビンゴカードの後に、抽選する数字を改行区切りにした文字列
/// - 戻り値: (ビンゴしたかどうかの boolean, ビンゴカード)
pub fn knock_98(s: &str) -> (bool, Vec<Knock98Column>) {
    todo!()
}

/// ビンゴの列を表す
/// 穴が空いていない場合は Some(数字), 穴が開いた場合は None になる
type Knock98Column = Vec<Option<usize>>;

#[cfg(test)]
mod tests_98 {
    use crate::knocks::knocks_90_99::knock_98;
    use crate::test_utils::read_resource;

    #[test]
    fn it_works_bingo_vertical() {
        let txt = read_resource("knock_98_bingo_vertical.txt");
        let actual = knock_98(&txt);
        let expected = (
            true,
            vec![
                vec![Some(37), None, Some(29), Some(20), None],
                vec![Some(16), None, None, Some(2), Some(74)],
                vec![Some(38), None, Some(35), Some(43), Some(8)],
                vec![Some(27), None, Some(30), Some(6), Some(69)],
                vec![Some(25), None, Some(52), Some(63), Some(70)],
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_bingo_horizontal() {
        let txt = read_resource("knock_98_bingo_horizontal.txt");
        let actual = knock_98(&txt);
        let expected = (
            true,
            vec![
                vec![Some(37), Some(66), Some(29), Some(20), Some(33)],
                vec![Some(16), None, Some(21), None, Some(74)],
                vec![Some(38), Some(59), Some(35), Some(43), Some(8)],
                vec![Some(27), Some(73), Some(30), Some(6), Some(69)],
                vec![None, None, None, None, None],
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_bingo_diagonal_up() {
        let txt = read_resource("knock_98_bingo_diagonal_up.txt");
        let actual = knock_98(&txt);
        let expected = (
            true,
            vec![
                vec![Some(37), Some(66), Some(29), Some(20), None],
                vec![Some(16), Some(4), Some(21), None, Some(74)],
                vec![Some(38), Some(59), None, Some(43), Some(8)],
                vec![Some(27), None, Some(30), Some(6), Some(69)],
                vec![None, Some(13), Some(52), Some(63), Some(70)],
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_bingo_diagonal_down() {
        let txt = read_resource("knock_98_bingo_diagonal_down.txt");
        let actual = knock_98(&txt);
        let expected = (
            true,
            vec![
                vec![None, None, Some(29), Some(20), Some(33)],
                vec![Some(16), None, Some(21), None, Some(74)],
                vec![Some(38), Some(59), None, Some(43), Some(8)],
                vec![Some(27), Some(73), None, None, Some(69)],
                vec![Some(25), None, Some(52), Some(63), None],
            ],
        );
        assert_eq!(actual, expected);
    }

    #[test]
    fn it_works_not_bingo() {
        let txt = read_resource("knock_98_not_bingo.txt");
        let actual = knock_98(&txt);
        let expected = (
            false,
            vec![
                vec![Some(37), None, Some(29), Some(20), Some(33)],
                vec![None, Some(4), None, Some(2), Some(74)],
                vec![None, None, Some(35), None, None],
                vec![Some(27), None, None, Some(6), Some(69)],
                vec![Some(25), Some(13), None, None, Some(70)],
            ],
        );
        assert_eq!(actual, expected);
    }
}
