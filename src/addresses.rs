use json::*;

pub struct Index {
    pub title: String,
    pub m3u8: String,
    pub tsa: Vec<String>,
}

pub fn new(inp: &str) -> Result<Index> {
    let ind = json::parse(inp).unwrap();

    let mut tsa = Vec::new();
    let tsain = ind["tsa"].members().as_slice().to_vec();

    for i in 0..tsain.len() {
        tsa.push(tsain[i].as_str().unwrap().to_owned());
    }

    let out = Index { 
        title: ind["title"].as_str().unwrap().to_owned(),
        m3u8: ind["m3u8"].as_str().unwrap().to_owned(),
        tsa: tsa,
    };

    Ok(out)
}