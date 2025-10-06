use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use bson::serde_helpers::datetime::FromChrono04DateTime;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DonationUrl {
    pub id: Option<String>,
    pub platform: Option<String>,
    pub url: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct License {
    pub id: Option<String>,
    pub name: Option<String>,
    pub url: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GalleryItem {
    pub url: String,
    pub featured: bool,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub created: DateTime<Utc>,
    pub ordering: Option<i64>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Project {
    #[serde(alias = "_id")]
    pub id: String,
    pub slug: String,
    pub title: Option<String>,
    pub description: Option<String>,
    pub categories: Option<Vec<String>>,
    pub client_side: Option<String>,
    pub server_side: Option<String>,
    pub body: Option<String>,
    pub status: Option<String>,
    pub requested_status: Option<String>,
    pub additional_categories: Option<Vec<String>>,
    pub issues_url: Option<String>,
    pub source_url: Option<String>,
    pub wiki_url: Option<String>,
    pub discord_url: Option<String>,
    pub donation_urls: Option<Vec<DonationUrl>>,
    pub project_type: Option<String>,
    pub downloads: Option<i64>,
    pub icon_url: Option<String>,
    pub color: Option<u32>,
    pub thread_id: Option<String>,
    pub monetization_status: Option<String>,
    pub team: String,
    pub body_url: Option<String>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub published: DateTime<Utc>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub updated: DateTime<Utc>,
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub approved: Option<DateTime<Utc>>,
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub queued: Option<DateTime<Utc>>,
    pub followers: u32,
    pub license: Option<License>,
    pub versions: Option<Vec<String>>,
    pub game_versions: Option<Vec<String>>,
    pub loaders: Option<Vec<String>>,
    pub gallery: Option<Vec<GalleryItem>>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dependencies {
    pub version_id: Option<String>,
    pub project_id: Option<String>,
    pub file_name: Option<String>,
    pub dependency_type: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hashes {
    pub sha512: String,
    pub sha1: String,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    #[serde(alias = "_id")]
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    pub file_type: Option<String>,
    pub version_id: String,
    pub project_id: String,
    pub file_cdn_cached: Option<bool>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileInfo {
    pub hashes: Hashes,
    pub url: String,
    pub filename: String,
    pub primary: bool,
    pub size: i64,
    pub file_type: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Version {
    #[serde(alias = "_id")]
    pub id: String,
    pub project_id: String,
    pub name: Option<String>,
    pub version_number: Option<String>,
    pub changelog: Option<String>,
    pub dependencies: Option<Vec<Dependencies>>,
    pub game_versions: Option<Vec<String>>,
    pub version_type: Option<String>,
    pub loaders: Option<Vec<String>>,
    pub featured: Option<bool>,
    pub status: Option<String>,
    pub requested_status: Option<String>,
    pub author_id: String,
    #[serde_as(as = "FromChrono04DateTime")]
    pub date_published: DateTime<Utc>,
    pub downloads: i64,
    pub changelog_url: Option<String>,
    pub files: Vec<FileInfo>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    pub icon: String,
    pub name: String,
    pub project_type: Option<String>,
    pub header: String,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Loader {
    pub icon: String,
    pub name: String,
    pub supported_project_types: Vec<String>,
    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct GameVersion {
    pub version: String,
    pub version_type: String,
    #[serde_as(as = "FromChrono04DateTime")]
    pub date: DateTime<Utc>,
    pub major: bool,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_modrinth_project_model() {
        let json = r###"
        {
            "_id": "Ua7DFN59",
            "additional_categories": [],
            "approved": {
                "$date": "2022-11-28T18:42:13.412Z"
            },
            "body": "<p><img src=\"https://i.imgur.com/1SgmaLx.png\" alt=\"\" width=\"1344\" height=\"416\" /><br /><br />\n<a title=\"Join the YUNG GANG on Discord!\" href=\"https://discord.gg/rns3beq\" target=\"_blank\" rel=\"noopener\"><img src=\"https://tinyurl.com/yungsdiscordbadge2\" alt=\"Join the YUNG GANG on Discord!\" width=\"136\" height=\"28\" /></a>&nbsp;<a title=\"Follow me on Twitter!\" href=\"https://twitter.com/yungnickyoung\" target=\"_blank\" rel=\"noopener\"><img src=\"https://tinyurl.com/yungstwitterbadge3\" alt=\"Follow me on Twitter!\" width=\"76\" height=\"27\" /></a>&nbsp;<a title=\"Subscribe to my YouTube!\" href=\"https://www.youtube.com/yungnickyoung\" target=\"_blank\" rel=\"noopener\"><img src=\"https://tinyurl.com/yungsyoutubebadge\" alt=\"Subscribe to my YouTube!\" width=\"80\" height=\"30\" /></a>&nbsp;<a title=\"Support me on Patreon!\" href=\"https://patreon.com/yungnickyoung\" target=\"_blank\" rel=\"noopener\"><img src=\"https://tinyurl.com/yungspatreonbadge\" alt=\"Support me on Patreon!\" width=\"138\" height=\"28\" /></a></p>\n<p>This is a library mod for YUNG's mods.</p>\n<p><b>For all my mod devs out there - </b>This provides a lot of useful stuff, especially if you're a worldgen modder!</p>\n<p>The API includes the following:</p>\n<ul>\n<li>AutoRegistration system (1.18+ only). Register any field with only a simple annotation, regardless of mod loader!</li>\n<li>Custom reimplementation of Jigsaw Manager with improved performance and custom pool element types with various new properties. Check out the <a href=\"https://github.com/yungnickyoung/YUNGs-Better-Dungeons\" target=\"_blank\" rel=\"noopener noreferrer\">Better Dungeons code</a> to see it in action.</li>\n<li>New criteria trigger for safely locating any structure. If the given structure doesn't exist, the trigger simply fails rather than instantly passing (unlike vanilla).</li>\n<li>Interfaces for JSON serialization &amp; deserialization&nbsp;with built-in type adapters.</li>\n<li>Simple, lightweight math utilities for vectors and column positions</li>\n<li><em>BlockStateRandomizer</em> and <em>ItemRandomizer</em>, data abstractions that make adding block and item randomization to your structures incredibly simple. I use these for all of my mods!</li>\n</ul>\n<p>If you're curious, you can check the code for my mods (especially the newer ones) to see how things work. Feel free to ask me any questions on Discord!</p>\n<p><img src=\"https://i.imgur.com/a49IMQJ.png\" /><br />\n<a title=\"Get 25% off your server with code YUNGNICKYOUNG at Bisect Hosting!\" href=\"http://sbisecthosting.com/yung\" target=\"_blank\" rel=\"noopener noreferrer\"><img src=\"https://i.imgur.com/nQRs7ZP.png\" alt=\"Use code YUNGNICKYOUNG with Bisect Hosting for 25% off!\" width=\"1344\" height=\"400\" /></a></p>",
            "body_url": null,
            "categories": [
                "library",
                "worldgen"
            ],
            "client_side": "required",
            "color": 5970187,
            "description": "Library mod for YUNG's mods.",
            "discord_url": "https://discord.gg/rns3beq",
            "donation_urls": [
                {
                    "id": "patreon",
                    "platform": "Patreon",
                    "url": "https://www.patreon.com/yungnickyoung"
                }
            ],
            "downloads": 9979426,
            "followers": 1773,
            "gallery": [],
            "game_versions": [
                "1.18.2",
                "1.19.2",
                "1.19.3",
                "1.19.4",
                "1.20",
                "1.20.1",
                "1.20.4",
                "1.21",
                "1.21.1",
                "1.21.4"
            ],
            "icon_url": "https://cdn.modrinth.com/data/Ua7DFN59/0fab1c351bf00926a8e1c91dc64b7c88832c3e1f_96.webp",
            "issues_url": "https://github.com/YUNG-GANG/YUNGs-API/issues",
            "license": {
                "id": "LGPL-3.0-only",
                "name": "GNU Lesser General Public License v3.0 only",
                "url": null
            },
            "loaders": [
                "fabric",
                "forge",
                "neoforge"
            ],
            "moderator_message": null,
            "monetization_status": "monetized",
            "project_type": "mod",
            "published": {
                "$date": "2022-11-28T17:17:12.676Z"
            },
            "queued": null,
            "requested_status": null,
            "server_side": "required",
            "slug": "yungs-api",
            "source_url": "https://github.com/YUNG-GANG/YUNGs-API",
            "status": "approved",
            "sync_at": {
                "$date": "2025-06-17T12:44:55.245Z"
            },
            "team": "3TidTIHz",
            "thread_id": "Ua7DFN59",
            "title": "YUNG's API",
            "translated_description": null,
            "updated": {
                "$date": "2025-05-02T22:27:47.152Z"
            },
            "versions": [
                "xET3UZBe",
                "UNVzqGkX",
                "82XBGKbQ",
                "xvoWCwex",
                "LYoQlbQt",
                "YwHWUw19",
                "LEuKu3qt",
                "Em3G31xp",
                "dpSzBMP6",
                "IOIGqCVr",
                "i0Z1vSK9",
                "YZE1pnbT",
                "GNNfW5IV",
                "IxuGYnWF",
                "yIFytswN",
                "h32n7OPC",
                "4Ek11kQV",
                "NmrTF2A5",
                "TT8tnzlH",
                "HIRzLg0r",
                "pxmQWPn7",
                "L5GqhLVE",
                "QnR5jGmc",
                "hyQxutx9",
                "rbgh8n1F",
                "5Zb55w2q",
                "k1OTLc33",
                "sE5QMX20",
                "RXxBbRs7",
                "wddoDji1",
                "dpTBMhjf",
                "jLW564iU",
                "a7qxhSOZ",
                "aMs83SRk",
                "zPT7QfIk",
                "Nx7XHO30",
                "mBbkZrZ1",
                "fFD2YR4D",
                "PJOYAmAs",
                "lscV1N5k",
                "LkDReYww",
                "tumhJgug",
                "MIGLewpu",
                "PpGXywDf",
                "ex8YYvxI",
                "LMXPKbZf",
                "DeaIlZ9A",
                "97xRZcgc",
                "MoMQNZ94",
                "fVwzTPig",
                "x20IZIXE",
                "kWoI0jki",
                "U4m2SXEP",
                "FVhw2zf8",
                "CbIBPwz2",
                "gRJY0EjN",
                "r6h5nMGq"
            ],
            "wiki_url": null
        }
        "###;

        let p: Project = serde_json::from_str(json).expect("deserialize project from json");
        assert_eq!(p.id, "Ua7DFN59");
        assert_eq!(p.slug, "yungs-api");
    }

    #[test]
    fn test_modrinth_version_model() {
        let json = r###"
        {
            "_id": "1ZHtT6Xo",
            "author_id": "l45nT5ov",
            "changelog": "- [Port to 1.21.4](https://github.com/jaredlll08/Clumps/commit/55ffbef1bc11ab191c9ccf41c554a7e1de1f865d) - Jared \n- [re-add forge to the webhook](https://github.com/jaredlll08/Clumps/commit/085fcd4dd13377a498ce02e2a70c0b90dbc00a53) - Jared \n- [Port to 1.21.3](https://github.com/jaredlll08/Clumps/commit/e21f0bbf23baa2e08638e524d93ce71a6026e16a) - Jared \n- [Port to 1.21.2](https://github.com/jaredlll08/Clumps/commit/f301704636752d3a0b693aff5ff48178f6730e6e) - Jared \n- [Port to 1.21.1. Fix Forge version not working. Close #147](https://github.com/jaredlll08/Clumps/commit/9dc57a1d3bd3cab65a34394718bc5813c46c9915) - Jared \n- [Restore Forge support (#143)](https://github.com/jaredlll08/Clumps/commit/515f24247c72ae77b5b1c043389e9b4a86679328) - Paint_Ninja \n- [Port to 1.21](https://github.com/jaredlll08/Clumps/commit/327ca88f573f0a7f46b1c1701aa9e4a5a3b096f5) - Jared \n- [Port to 1.20.6. Close #139](https://github.com/jaredlll08/Clumps/commit/0bc727c4345894322540c709889af89ba2492c69) - Jared \n- [port to 1.20.5](https://github.com/jaredlll08/Clumps/commit/959e3961e6b077e286a845c9ca6051959f192f58) - Jared \n- [Update mod version](https://github.com/jaredlll08/Clumps/commit/6f56ba314adf695e0ef34f1a4d4084b9c43c1ac6) - Jared",
            "changelog_url": null,
            "date_published": {
                "$date": "2024-12-22T23:43:33Z"
            },
            "dependencies": [
                {
                    "version_id": null,
                    "project_id": "P7dR8mSH",
                    "file_name": null,
                    "dependency_type": "required"
                }
            ],
            "downloads": 12260,
            "featured": false,
            "files": [
                {
                    "hashes": {
                        "sha512": "86909659af2f4b481ae9b230996e86658e622424e28b808d069144bf116bf47191df74cfec8b88bbc37ec9ad8cf5a4a24a0f21b39d6c456132331881c8575aeb",
                        "sha1": "d1f522452cfa1286349525ccace065a8ec7eb940"
                    },
                    "url": "https://cdn.modrinth.com/data/Wnxd13zP/versions/1ZHtT6Xo/Clumps-fabric-1.21.4-22.0.0.1.jar",
                    "filename": "Clumps-fabric-1.21.4-22.0.0.1.jar",
                    "primary": true,
                    "size": 20721,
                    "file_type": null
                }
            ],
            "found": true,
            "game_versions": [
                "1.21.4"
            ],
            "loaders": [
                "fabric"
            ],
            "name": "Fabric-1.21.4-22.0.0.1",
            "project_id": "Wnxd13zP",
            "requested_status": null,
            "slug": "clumps",
            "status": "listed",
            "sync_at": {
                "$date": "2024-12-31T15:01:35Z"
            },
            "version_number": "22.0.0.1",
            "version_type": "release"
        }
        "###;

        let v: Version = serde_json::from_str(json).expect("deserialize version from json");
        assert_eq!(v.id, "1ZHtT6Xo");
        assert_eq!(v.project_id, "Wnxd13zP");
    }

    #[test]
    fn test_modrinth_file_model() {
        let json = r###"
        {
            "_id": {
                "sha512": "86909659af2f4b481ae9b230996e86658e622424e28b808d069144bf116bf47191df74cfec8b88bbc37ec9ad8cf5a4a24a0f21b39d6c456132331881c8575aeb",
                "sha1": "d1f522452cfa1286349525ccace065a8ec7eb940"
            },
            "file_cdn_cached": false,
            "file_type": null,
            "filename": "Clumps-fabric-1.21.4-22.0.0.1.jar",
            "found": true,
            "primary": true,
            "project_id": "Wnxd13zP",
            "size": 20721,
            "sync_at": {
                "$date": "2024-12-31T15:01:35Z"
            },
            "url": "https://cdn.modrinth.com/data/Wnxd13zP/versions/1ZHtT6Xo/Clumps-fabric-1.21.4-22.0.0.1.jar",
            "version_id": "1ZHtT6Xo"
        }
        "###;
        let f: File = serde_json::from_str(json).expect("deserialize file from json");
        assert_eq!(f.hashes.sha1, "d1f522452cfa1286349525ccace065a8ec7eb940");
        assert_eq!(f.version_id, "1ZHtT6Xo");
        assert_eq!(f.project_id, "Wnxd13zP");
    }

    #[test]
    fn test_modrinth_category_model() {
        let json = r###"
            {
                "_id": {
                    "$oid": "67960188a71e867c27236cb4"
                },
                "header": "categories",
                "icon": "<svg fill=\"none\" viewBox=\"0 0 24 24\" stroke=\"currentColor\" stroke-width=\"2\"><path stroke-linecap=\"round\" stroke-linejoin=\"round\" d=\"M3.055 11H5a2 2 0 012 2v1a2 2 0 002 2 2 2 0 012 2v2.945M8 3.935V5.5A2.5 2.5 0 0010.5 8h.5a2 2 0 012 2 2 2 0 104 0 2 2 0 012-2h1.064M15 20.488V18a2 2 0 012-2h3.064M21 12a9 9 0 11-18 0 9 9 0 0118 0z\" /></svg>",
                "name": "worldgen",
                "project_type": "mod",
                "sync_at": {
                    "$date": "2025-01-26T09:34:00.994Z"
                }
            }
        "###;
        let c: Category = serde_json::from_str(json).expect("deserialize category from json");
        assert_eq!(c.name, "worldgen");
    }

    #[test]
    fn test_modrinth_loader_model() {
        let json = r###"
            {
                "_id": {
                    "$oid": "67960189a71e867c27236cca"
                },
                "icon": "<svg viewBox=\"0 0 24 24\" fill=\"none\" stroke=\"currentColor\" stroke-width=\"2\" stroke-linecap=\"round\" stroke-linejoin=\"round\"><path d=\"M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z\"/></svg>",
                "name": "waterfall",
                "supported_project_types": [
                    "plugin",
                    "project",
                    "mod"
                ],
                "sync_at": {
                    "$date": "2025-01-26T09:34:01.552Z"
                }
            }
        "###;
        let l: Loader = serde_json::from_str(json).expect("deserialize loader from json");
        assert_eq!(l.name, "waterfall");
    }

    #[test]
    fn test_modrinth_game_version_model() {
        let json = r###"
            {
                "_id": {
                    "$oid": "6795fc3ce84ef657877b7a20"
                },
                "date": {
                    "$date": "2025-01-22T13:14:44Z"
                },
                "major": false,
                "sync_at": {
                    "$date": "2025-01-26T09:11:24.542Z"
                },
                "version": "25w04a",
                "version_type": "snapshot"
            }
        "###;
        let gv: GameVersion = serde_json::from_str(json).expect("deserialize game version from json");
        assert_eq!(gv.version, "25w04a");
    }
}
