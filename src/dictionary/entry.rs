/// A single row in the `stardict` table (ECDICT schema).
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct StardictEntry {
    pub id: i64,
    pub word: String,
    pub sw: String,
    pub phonetic: Option<String>,
    pub definition: Option<String>,
    pub translation: Option<String>,
    pub pos: Option<String>,
    pub collins: Option<i32>,
    pub oxford: Option<i32>,
    pub tag: Option<String>,
    pub bnc: Option<i32>,
    pub frq: Option<i32>,
    pub exchange: Option<String>,
    pub detail: Option<String>,
    pub audio: Option<String>,
}

impl StardictEntry {
    /// Parse exchange field, e.g. `d:perceived/p:perceived/3:perceives/i:perceiving`
    pub fn exchange_info(&self) -> Option<String> {
        let exchange = self.exchange.as_ref()?;
        let mut infos = Vec::new();
        for pair in exchange.split('/') {
            let mut kv = pair.splitn(2, ':');
            let key = kv.next()?;
            let value = kv.next()?;
            let s = match key {
                "p" => format!("过去式: {}", value),
                "d" => format!("过去分词: {}", value),
                "i" => format!("现在分词: {}", value),
                "3" => format!("第三人称单数: {}", value),
                "r" => format!("形容词比较级: {}", value),
                "t" => format!("形容词最高级: {}", value),
                "s" => format!("名词复数形式: {}", value),
                "0" => format!("lemma: {}", value),
                "1" => format!("lemma transform: {}", value),
                _ => continue,
            };
            infos.push(s);
        }
        if infos.is_empty() { None } else { Some(infos.join("; ")) }
    }

    /// Parse tag field (exam/level tags).
    pub fn tag_info(&self) -> Option<String> {
        let tag = self.tag.as_ref()?;
        let infos: Vec<&str> = tag
            .split_whitespace()
            .map(|t| match t {
                "zk" => "中考",
                "gk" => "高考",
                "cet4" => "CET4",
                "cet6" => "CET6",
                "ky" => "考研",
                "gre" => "GRE",
                "toefl" => "TOEFL",
                "ielts" => "IELTS",
                _ => "Unknown",
            })
            .collect();
        if infos.is_empty() { None } else { Some(infos.join("/")) }
    }
}
