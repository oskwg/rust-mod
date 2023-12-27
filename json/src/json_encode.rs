
mod inner_mod {
    use crate::codec::handle::codec;

    pub fn encode(s: &String) -> String {
        let cc: String = codec(&s);
        println!("private: {}", cc);
        cc
    }
}

pub fn encode(s: &String) -> String {
    inner_mod::encode(s)
}


