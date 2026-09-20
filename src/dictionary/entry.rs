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

#[cfg(test)]
mod tests {
    use super::*;

    fn entry_with(exchange: Option<&str>, tag: Option<&str>) -> StardictEntry {
        StardictEntry {
            id: 1,
            word: "test".into(),
            sw: "test".into(),
            phonetic: None,
            definition: None,
            translation: None,
            pos: None,
            collins: None,
            oxford: None,
            tag: tag.map(str::to_string),
            bnc: None,
            frq: None,
            exchange: exchange.map(str::to_string),
            detail: None,
            audio: None,
        }
    }

    #[test]
    fn exchange_info_parses_known_keys() {
        let entry = entry_with(Some("d:perceived/p:perceived/3:perceives/i:perceiving"), None);
        let info = entry.exchange_info().unwrap();
        assert!(info.contains("过去分词: perceived"));
        assert!(info.contains("过去式: perceived"));
        assert!(info.contains("第三人称单数: perceives"));
        assert!(info.contains("现在分词: perceiving"));
    }

    #[test]
    fn exchange_info_skips_unknown_keys() {
        let entry = entry_with(Some("x:ignored/p:went"), None);
        assert_eq!(entry.exchange_info().as_deref(), Some("过去式: went"));
    }

    #[test]
    fn exchange_info_returns_none_on_malformed_pair() {
        // `?` on missing value short-circuits the whole Option
        let entry = entry_with(Some("p:went/badpair"), None);
        assert!(entry.exchange_info().is_none());
    }

    #[test]
    fn exchange_info_returns_none_when_empty_or_absent() {
        assert!(entry_with(None, None).exchange_info().is_none());
        assert!(entry_with(Some(""), None).exchange_info().is_none());
        assert!(entry_with(Some("x:only-unknown"), None).exchange_info().is_none());
    }

    #[test]
    fn exchange_info_covers_adjective_and_noun_forms() {
        let entry = entry_with(Some("r:better/t:best/s:books/0:book/1:booked"), None);
        let info = entry.exchange_info().unwrap();
        assert!(info.contains("形容词比较级: better"));
        assert!(info.contains("形容词最高级: best"));
        assert!(info.contains("名词复数形式: books"));
        assert!(info.contains("lemma: book"));
        assert!(info.contains("lemma transform: booked"));
    }

    #[test]
    fn tag_info_maps_known_exam_tags() {
        let entry = entry_with(None, Some("zk gk cet4 cet6 ky gre toefl ielts"));
        assert_eq!(entry.tag_info().as_deref(), Some("中考/高考/CET4/CET6/考研/GRE/TOEFL/IELTS"));
    }

    #[test]
    fn tag_info_marks_unknown_tags() {
        let entry = entry_with(None, Some("zk foo"));
        assert_eq!(entry.tag_info().as_deref(), Some("中考/Unknown"));
    }

    #[test]
    fn tag_info_returns_none_when_absent_or_blank() {
        assert!(entry_with(None, None).tag_info().is_none());
        assert!(entry_with(None, Some("")).tag_info().is_none());
        assert!(entry_with(None, Some("   ")).tag_info().is_none());
    }
}
