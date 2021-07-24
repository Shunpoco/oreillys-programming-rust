// extern crateはCargo.tomlに記述したcrate ironとmimeを利用するためのもの
extern crate iron;
// macro_useは、crateでexportされているマクロを使うことを宣言する
#[macro_use] extern crate mime;

// use宣言で、crateの公開されている機能を取り込む。*はすべての名前を利用できるようにしている。
// preludeはそのクレートのユーザすべてが利用するような一般的な機能をエクスポートする慣例があるのでワイルドカードを使っている
use iron::prelude::*;
use iron::status;

fn main() {
    println!("Serving on http://localhost:3000...");
    Iron::new(get_form).http("localhost:3000").unwrap();
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
            <input type="text" name="m"/>
            <button type="submit">Compute GCD</button> 
        </form>    
    "#);

    Ok(response)
}