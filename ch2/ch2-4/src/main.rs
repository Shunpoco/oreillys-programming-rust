// traitsをスコープに取り込む
// traitsは型が実装することのできるメソッドの集合。なんのこっちゃ
use std::io::Write;
use std::str::FromStr;

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

fn main() {
    // pythonにおけるリスト、goにおけるslice
    // 値の追加を行うにはmutが必須
    let mut numbers = Vec::new();

    // std::env::args() はイテレータを返す
    // skipメソッドは指定したintegerだけイテレータをスキップしたイテレータを作る
    for arg in std::env::args().skip(1) {
        // ここでu64をpushしていることから、numbersはVec<u64>と推論される
        numbers.push(
            // u64としてパース
            // u64を直接返すのではなく、パースが成功したかどうかを示すResult値を返す。
            // Ok(v): パース成功、vは生成した値
            // Err(e): パース失敗、eは其の理由を説明するエラー値
            // expectメソッドで、パースが成功したかどうかをチェックしている。eの場合はメッセージを出力し、プログラムの実行を中断する
            u64::from_str(&arg)
            .expect("error parsing argument")
        );
    }

    if numbers.len() == 0 {
        // unwrap()はエラーメッセージの出力が成功したかどうかをチェックする(expectよりも雑な？書き方)
        writeln!(std::io::stderr(), "Usage: gcd NUMBER ...").unwrap();
        std::process::exit(1);
    }

    let mut d = numbers[0];
    // ベクタの要素を処理する際、所有権はnumbersに残っている。&演算子は要素への参照を借用している
    for m in &numbers[1..] {
        // *演算子はmの参照解決を行う。参照されている値を返す
        d = gcd(d, *m);
    }

    println!("The greatest common divisor of {:?} is {}", numbers, d)

    // ベクタはnumbersに所有されているので、main関数の最後でnumbersがスコープから外れると、ベクタは自動的に解法される。
}
