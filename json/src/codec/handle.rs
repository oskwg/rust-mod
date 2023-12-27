pub fn codec(s: &String) -> String {
    self::side();
    format!("codec: {}",json::stringify_pretty(s,2u16))
}

fn side() {
    // 副作用
}