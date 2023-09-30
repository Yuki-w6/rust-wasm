use proconio::input;

fn main() {
    input!{
        deg: i32,
        dis: i32
    }

    let huko: &str;
    if 112 < deg && deg <= 337 {
        huko = "NNE";
    } else if 337 < deg && deg <= 562 {
        huko = "NE";
    } else if 562 < deg && deg <= 787 {
        huko = "ENE";
    } else if 787 < deg && deg <= 1012 {
        huko = "E";
    } else if 1012 < deg && deg <= 1237 {
        huko = "ESE";
    } else if 1237 < deg && deg <= 1462 {
        huko = "SE";
    } else if deg < 1462 && deg <= 1687 {
        huko = "SSE";
    } else if 1687 < deg && deg <= 1912 {
        huko = "S";
    } else if 1912 < deg && deg <= 2137 {
        huko = "SSW";
    } else if 2137 < deg && deg <= 2362 {
        huko = "SW";
    } else if 2362 < deg && deg <= 2587 {
        huko = "WSW";
    } else if 2587 < deg && deg <= 2812 {
        huko = "W";
    } else if 2182 < deg && deg <= 3037 {
        huko = "WNW";
    } else if 3037 < deg && deg <= 3262 {
        huko = "NW";
    } else if 3262 < deg && deg <= 3487 {
        huko = "NNW";
    } else {
        huko = "N";
    }

     // 桁数を指定して四捨五入
     let num: f64 = 123.44;
     let base_number = 10.0;
     let rounded = (num * base_number).round() / base_number;
     println!("{}", rounded);
     // 123.4
 }
