// use std::collections::HashMap;

fn main() {
    let resp = fetch().unwrap();
    dbg!(resp);

    // TODO JSON Decoder

    //let origin = match resp.get("origin") {
    //    Some(val) => val,
    //    None => "val",
    //};

    //let not_item = match resp.get("not_item") {
    //    Some(val) => val,
    //    None => "default",
    //};
    // dbg!(origin);
    // dbg!(not_item);
    // dbg!(resp);
}
// enum Root {
//     String(String),
//     i32(i32),
//     Array(Vec<String>),
//     Null
// }

fn fetch() -> Result<String, Box<dyn std::error::Error>> {

    let resp = reqwest::blocking::get("https://scrapbox.io/api/pages/araki-web-dev/");
    let res = resp.unwrap();
    dbg!(&res);
    let json = res.text().unwrap();
    dbg!(&json);
    Ok(json)
}

#[test]
fn fetch_to_example_test() {
    let hoge = fetch().unwrap();
    dbg!(hoge);
    assert!(true)
}