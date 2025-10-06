use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde_with::serde_as;

use bson::serde_helpers::datetime::FromChrono04DateTime;

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct ModrinthTranslation {
    #[serde(alias = "_id")]
    pub project_id: String,
    pub translated: Option<String>,
    pub original: Option<String>,

    pub need_to_update: bool,

    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub translated_at: Option<DateTime<Utc>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Clone)]
pub struct CurseForgeTranslation {
    #[serde(rename = "modId", alias = "_id")]
    pub mod_id: i32,
    pub translated: Option<String>,
    pub original: Option<String>,

    pub need_to_update: bool,

    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub translated_at: Option<DateTime<Utc>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_curseforge_translation() {
        let json = r###"
        {
            "_id": 238222,
            "original": "View Items and Recipes",
            "translated": "查看物品和配方",
            "translated_at": {
                "$date": "2025-02-02T10:01:52.805Z"
            },
            "need_to_update": false
        }
        "###;

        let t: CurseForgeTranslation =
            serde_json::from_str(json).expect("deserialize translation from json");
        assert_eq!(t.mod_id, 238222);
    }

    #[test]
    fn test_modrinth_translation() {
        let json = r###"
            {
        "_id": "Wnxd13zP",
        "original": "Clumps XP orbs together to reduce lag",
        "translated": "将经验球聚集在一起以减少游戏卡顿。",
        "translated_at": {
            "$date": "2025-02-02T08:53:28.684Z"
        },
        "need_to_update": false
    }
    "###;

    let l: ModrinthTranslation =
        serde_json::from_str(json).expect("deserialize translation from json");
    
    assert_eq!(l.project_id, "Wnxd13zP");
    }
}