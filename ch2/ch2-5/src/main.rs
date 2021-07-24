// extern crateはCargo.tomlに記述したcrate ironとmimeを利用するためのもの
extern crate iron;
extern crate router;
extern crate urlencoded;
// macro_useは、crateでexportされているマクロを使うことを宣言する
#[macro_use] extern crate mime;

// use宣言で、crateの公開されている機能を取り込む。*はすべての名前を利用できるようにしている。
// preludeはそのクレートのユーザすべてが利用するような一般的な機能をエクスポートする慣例があるのでワイルドカードを使っている
use iron::prelude::*;
use iron::status;
use router::Router;
use std::str::FromStr;
use urlencoded::UrlEncodedBody;

fn main() {
    let mut router = Router::new();

    router.get("/", get_form, "root");
    router.post("/gcd", post_gcd, "gcd");

    println!("Serving on http://localhost:3000...");
    Iron::new(router).http("localhost:3000").unwrap();
}

// &mutは可変参照を示す
// コンパイラが警告を出さないように、仮引数の名前を_で始めている（この変数を使用しないことをコンパイラに教えている）
fn get_form(_request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    // raw-string構文。 r + 0個以上のハッシュマーク(#), ダブルクオートで始まる文字列 + 同じ個数のハッシュ
    // 任意の文字をエスケープ無しで扱うことが可能（ダブルクオートやハッシュマークも）
    response.set_mut(r#"
        <title>GCD Calculator</title>
        <form action="/gcd" method="post">
            <input type="text" name="n"/>
            <input type="text" name="n"/>
            <button type="submit">Compute GCD</button> 
        </form>    
    "#);

    Ok(response)
}

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

fn post_gcd(request: &mut Request) -> IronResult<Response> {
    let mut response = Response::new();

    // metch式は、ResultのOk or Errどちらの値を取っているかチェックした上で、内部の値を取り出すことができる
    // 順序として OK/Errのチェック -> 値の取り出し、となる
    // Rustでは、Resultのようなそれぞれ値を保持する複数の異型を持つ型をユーザが定義することができる（列挙型、enum）
    let form_data = match request.get_ref::<UrlEncodedBody>() {
        Err(e) => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("Error parsing form data: {:?}\n", e));
            return Ok(response);
        }
        Ok(map) => map
    };

    // nは単変数ではなく、文字列のベクタになっている
    let unparsed_numbers = match form_data.get("n") {
        None => {
            response.set_mut(status::BadRequest);
            response.set_mut(format!("form data has no 'n' parameter\n"));
            return Ok(response);
        }
        Some(nums) => nums
    };

    let mut numbers = Vec::new();
    for unparsed in unparsed_numbers {
        // 文字列をu64に変換する。ひとつでもエラーが発生したら、HTTPレスポンスとしてエラーを返す
        match u64::from_str(&unparsed) {
            Err(_) => {
                response.set_mut(status::BadRequest);
                response.set_mut(
                    format!("Value for 'n' paramter not a number: {:?}\n",
                            unparsed),
                );
                return Ok(response);
            }
            Ok(n) => { numbers.push(n); }
        }
    }

    // 前章で実装したgcd
    let mut d = numbers[0];
    for m in &numbers[1..] {
        d = gcd(d, *m);
    }

    response.set_mut(status::Ok);
    response.set_mut(mime!(Text/Html; Charset=Utf8));
    response.set_mut(
        format!("The greatest common divisor of the numbers {:?} is <b>{}</b>\n", numbers, d),
    );

    Ok(response)
}
