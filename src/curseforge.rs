use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_with::serde_as;

use bson::serde_helpers::datetime::FromChrono04DateTime;

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Category {
    #[serde(alias = "_id")]
    pub id: i32,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub name: String,
    pub slug: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde_as(as = "FromChrono04DateTime")]
    #[serde(rename = "dateModified")]
    pub date_modified: DateTime<Utc>,
    #[serde(rename = "isClass")]
    pub is_class: Option<bool>,
    #[serde(rename = "classId")]
    pub class_id: Option<i32>,
    #[serde(rename = "parentCategoryId")]
    pub parent_category_id: Option<i32>,
    #[serde(rename = "displayIndex")]
    pub display_index: i32,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct CategoryInfo {
    pub id: Option<i32>,
    #[serde(rename = "gameId")]
    pub game_id: Option<i32>,
    pub name: Option<String>,
    pub slug: Option<String>,
    pub url: Option<String>,
    #[serde(rename = "iconUrl")]
    pub icon_url: Option<String>,
    #[serde(rename = "dateModified")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "isClass")]
    pub is_class: Option<bool>,
    #[serde(rename = "classId")]
    pub class_id: Option<i32>,
    #[serde(rename = "parentCategoryId")]
    pub parent_category_id: Option<i32>,
    #[serde(rename = "displayIndex")]
    pub display_index: Option<i32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct File {
    #[serde(alias = "_id")]
    pub id: i32,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "modId")]
    pub mod_id: i32,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<bool>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "releaseType")]
    pub release_type: Option<i32>,
    #[serde(rename = "fileStatus")]
    pub file_status: Option<i32>,
    pub hashes: Option<Vec<Hash>>,
    #[serde(rename = "fileDate")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub file_date: Option<DateTime<Utc>>,
    #[serde(rename = "fileLength")]
    pub file_length: Option<i64>,
    #[serde(rename = "downloadCount")]
    pub download_count: Option<i64>,
    #[serde(rename = "fileSizeOnDisk")]
    pub file_size_on_disk: Option<i64>,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "gameVersions")]
    pub game_versions: Option<Vec<String>>,
    #[serde(rename = "sortableGameVersions")]
    pub sortable_game_versions: Option<Vec<FileSortableGameVersions>>,
    pub dependencies: Option<Vec<FileDependencies>>,
    #[serde(rename = "exposeAsAlternative")]
    pub expose_as_alternative: Option<bool>,
    #[serde(rename = "parentProjectFileId")]
    pub parent_project_file_id: Option<i32>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: Option<i32>,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: Option<bool>,
    #[serde(rename = "serverPackFileId")]
    pub server_pack_file_id: Option<i32>,
    #[serde(rename = "isEarlyAccessContent")]
    pub is_early_access_content: Option<bool>,
    #[serde(rename = "earlyAccessEndDate")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub early_access_end_date: Option<DateTime<Utc>>,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: Option<i64>,
    pub modules: Option<Vec<Module>>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileInfo {
    pub id: i32,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "modId")]
    pub mod_id: i32,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<bool>,
    #[serde(rename = "displayName")]
    pub display_name: Option<String>,
    #[serde(rename = "fileName")]
    pub file_name: Option<String>,
    #[serde(rename = "releaseType")]
    pub release_type: Option<i32>,
    #[serde(rename = "fileStatus")]
    pub file_status: Option<i32>,
    pub hashes: Option<Vec<Hash>>,
    #[serde(rename = "fileDate")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub file_date: Option<DateTime<Utc>>,
    #[serde(rename = "fileLength")]
    pub file_length: Option<i64>,
    #[serde(rename = "downloadCount")]
    pub download_count: Option<i64>,
    #[serde(rename = "fileSizeOnDisk")]
    pub file_size_on_disk: Option<i64>,
    #[serde(rename = "downloadUrl")]
    pub download_url: Option<String>,
    #[serde(rename = "gameVersions")]
    pub game_versions: Option<Vec<String>>,
    #[serde(rename = "sortableGameVersions")]
    pub sortable_game_versions: Option<Vec<FileSortableGameVersions>>,
    pub dependencies: Option<Vec<FileDependencies>>,
    #[serde(rename = "exposeAsAlternative")]
    pub expose_as_alternative: Option<bool>,
    #[serde(rename = "parentProjectFileId")]
    pub parent_project_file_id: Option<i32>,
    #[serde(rename = "alternateFileId")]
    pub alternate_file_id: Option<i32>,
    #[serde(rename = "isServerPack")]
    pub is_server_pack: Option<bool>,
    #[serde(rename = "serverPackFileId")]
    pub server_pack_file_id: Option<i32>,
    #[serde(rename = "isEarlyAccessContent")]
    pub is_early_access_content: Option<bool>,
    #[serde(rename = "earlyAccessEndDate")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub early_access_end_date: Option<DateTime<Utc>>,
    #[serde(rename = "fileFingerprint")]
    pub file_fingerprint: Option<i64>,
    pub modules: Option<Vec<Module>>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Mod {
    #[serde(alias = "_id")]
    pub id: i32,
    #[serde(rename = "gameId")]
    pub game_id: Option<i32>,
    pub name: Option<String>,
    pub slug: String,
    pub links: Option<Links>,
    pub summary: Option<String>,
    pub status: Option<i32>,
    #[serde(rename = "downloadCount")]
    pub download_count: Option<i64>,
    #[serde(rename = "isFeatured")]
    pub is_featured: Option<bool>,
    #[serde(rename = "primaryCategoryId")]
    pub primary_category_id: Option<i32>,
    pub categories: Option<Vec<CategoryInfo>>,
    #[serde(rename = "classId")]
    pub class_id: Option<i32>,
    pub authors: Option<Vec<Author>>,
    pub logo: Option<Logo>,
    pub screenshots: Option<Vec<ScreenShot>>,
    #[serde(rename = "mainFileId")]
    pub main_file_id: Option<i32>,
    #[serde(rename = "latestFiles")]
    pub latest_files: Option<Vec<FileInfo>>,
    #[serde(rename = "latestFilesIndexes")]
    pub latest_files_indexes: Option<Vec<FileIndex>>,
    #[serde(rename = "dateCreated")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub date_created: Option<DateTime<Utc>>,
    #[serde(rename = "dateModified")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub date_modified: Option<DateTime<Utc>>,
    #[serde(rename = "dateReleased")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub date_released: Option<DateTime<Utc>>,
    #[serde(rename = "allowModDistribution")]
    pub allow_mod_distribution: Option<bool>,
    #[serde(rename = "gamePopularityRank")]
    pub game_popularity_rank: Option<i32>,
    #[serde(rename = "isAvailable")]
    pub is_available: Option<bool>,
    #[serde(rename = "thumbsUpCount")]
    pub thumbs_up_count: Option<i32>,
    pub rating: Option<i32>,

    #[serde_as(as = "FromChrono04DateTime")]
    pub sync_at: DateTime<Utc>,
}

// #[serde_as]
// #[derive(Debug, Deserialize, Clone)]
// pub struct Fingerprint {
//     #[serde(alias = "_id")]
//     pub id: i64,
//     pub file: FileInfo,
//     #[serde(rename = "latestFiles")]
//     pub latest_files: Vec<FileInfo>,

//     #[serde_as(as = "FromChrono04DateTime")]
//     pub sync_at: DateTime<Utc>,
// }

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileDependencies {
    #[serde(rename = "modId")]
    pub mod_id: i32,
    #[serde(rename = "relationType")]
    pub relation_type: Option<i32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileSortableGameVersions {
    #[serde(rename = "gameVersionName")]
    pub game_version_name: Option<String>,
    #[serde(rename = "gameVersionPadded")]
    pub game_version_padded: Option<String>,
    #[serde(rename = "gameVersion")]
    pub game_version: Option<String>,
    #[serde(rename = "gameVersionReleaseDate")]
    #[serde_as(as = "Option<FromChrono04DateTime>")]
    pub game_version_release_date: Option<DateTime<Utc>>,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: Option<i32>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Hash {
    pub value: String,
    pub algo: i32,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub url: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Logo {
    pub id: i32,
    #[serde(rename = "modId")]
    pub mod_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    pub url: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Links {
    #[serde(rename = "websiteUrl")]
    pub website_url: Option<String>,
    #[serde(rename = "wikiUrl")]
    pub wiki_url: Option<String>,
    #[serde(rename = "issuesUrl")]
    pub issues_url: Option<String>,
    #[serde(rename = "sourceUrl")]
    pub source_url: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct ScreenShot {
    pub id: i32,
    #[serde(rename = "modId")]
    pub mod_id: i32,
    pub title: Option<String>,
    pub description: Option<String>,
    #[serde(rename = "thumbnailUrl")]
    pub thumbnail_url: Option<String>,
    pub url: Option<String>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Module {
    pub name: Option<String>,
    pub fingerprint: Option<i64>,
}

#[serde_as]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FileIndex {
    #[serde(rename = "gameVersion")]
    pub game_version: Option<String>,
    #[serde(rename = "fileId")]
    pub file_id: i32,
    pub filename: Option<String>,
    #[serde(rename = "releaseType")]
    pub release_type: Option<i32>,
    #[serde(rename = "gameVersionTypeId")]
    pub game_version_type_id: Option<i32>,
    #[serde(rename = "modLoader")]
    pub mod_loader: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json;

    #[test]
    fn test_curseforge_mod() {
        let json = r###"
        {
            "_id": 594678,
            "allowModDistribution": true,
            "authors": [
                {
                    "id": 100247822,
                    "name": "purplik",
                    "url": "https://www.curseforge.com/members/purplik"
                }
            ],
            "categories": [
                {
                    "id": 434,
                    "gameId": 432,
                    "name": "Armor, Tools, and Weapons",
                    "slug": "armor-weapons-tools",
                    "url": "https://www.curseforge.com/minecraft/mc-mods/armor-weapons-tools",
                    "iconUrl": "https://media.forgecdn.net/avatars/6/47/635351498790409758.png",
                    "dateModified": {
                        "$date": "2014-05-08T17:44:39.057Z"
                    },
                    "isClass": false,
                    "classId": 6,
                    "parentCategoryId": 6,
                    "displayIndex": null
                },
                {
                    "id": 424,
                    "gameId": 432,
                    "name": "Cosmetic",
                    "slug": "cosmetic",
                    "url": "https://www.curseforge.com/minecraft/mc-mods/cosmetic",
                    "iconUrl": "https://media.forgecdn.net/avatars/6/39/635351497555976928.png",
                    "dateModified": {
                        "$date": "2014-05-08T17:42:35.597Z"
                    },
                    "isClass": false,
                    "classId": 6,
                    "parentCategoryId": 6,
                    "displayIndex": null
                }
            ],
            "classId": 6,
            "dateCreated": {
                "$date": "2022-03-17T17:57:05.42Z"
            },
            "dateModified": {
                "$date": "2023-01-01T16:28:17.23Z"
            },
            "dateReleased": {
                "$date": "2023-01-01T16:19:51.07Z"
            },
            "downloadCount": 190029,
            "gameId": 432,
            "gamePopularityRank": 10790,
            "isAvailable": false,
            "isFeatured": false,
            "latestFiles": [
                {
                    "id": 3872689,
                    "gameId": 432,
                    "modId": 594678,
                    "isAvailable": true,
                    "displayName": "hats-and-cosmetics-1.2.1-1.19",
                    "fileName": "hats-and-cosmetics-1.2.1-1.19.jar",
                    "releaseType": 2,
                    "fileStatus": 4,
                    "hashes": [
                        {
                            "value": "8451c8829278f37a811ae70f21624e43dc26eb7a",
                            "algo": 1
                        },
                        {
                            "value": "5e6ed5e447c60dce70952386f3fbf017",
                            "algo": 2
                        }
                    ],
                    "fileDate": {
                        "$date": "2022-07-12T21:24:02.393Z"
                    },
                    "fileLength": 120306,
                    "downloadCount": 3591,
                    "fileSizeOnDisk": null,
                    "downloadUrl": "https://edge.forgecdn.net/files/3872/689/hats-and-cosmetics-1.2.1-1.19.jar",
                    "gameVersions": [
                        "Forge",
                        "1.19"
                    ],
                    "sortableGameVersions": [
                        {
                            "gameVersionName": "Forge",
                            "gameVersionPadded": "0",
                            "gameVersion": "",
                            "gameVersionReleaseDate": {
                                "$date": "2022-10-01T00:00:00Z"
                            },
                            "gameVersionTypeId": 68441
                        },
                        {
                            "gameVersionName": "1.19",
                            "gameVersionPadded": "0000000001.0000000019",
                            "gameVersion": "1.19",
                            "gameVersionReleaseDate": {
                                "$date": "2022-06-07T15:38:07.377Z"
                            },
                            "gameVersionTypeId": 73407
                        }
                    ],
                    "dependencies": [
                        {
                            "modId": 309927,
                            "relationType": 3
                        }
                    ],
                    "exposeAsAlternative": null,
                    "parentProjectFileId": null,
                    "alternateFileId": 0,
                    "isServerPack": false,
                    "serverPackFileId": null,
                    "isEarlyAccessContent": null,
                    "earlyAccessEndDate": null,
                    "fileFingerprint": 2194437269,
                    "modules": [
                        {
                            "name": "META-INF",
                            "fingerprint": 3874194274
                        },
                        {
                            "name": "com",
                            "fingerprint": 4269535854
                        },
                        {
                            "name": "assets",
                            "fingerprint": 3455112628
                        },
                        {
                            "name": "data",
                            "fingerprint": 3000469934
                        },
                        {
                            "name": "hat.png",
                            "fingerprint": 1651265609
                        },
                        {
                            "name": "pack.mcmeta",
                            "fingerprint": 2592535084
                        }
                    ]
                },
                {
                    "id": 4285139,
                    "gameId": 432,
                    "modId": 594678,
                    "isAvailable": true,
                    "displayName": "hats-and-cosmetics-1.4-1.18.2",
                    "fileName": "hats-and-cosmetics-1.4-1.18.2.jar",
                    "releaseType": 1,
                    "fileStatus": 4,
                    "hashes": [
                        {
                            "value": "6832509692a57ce11c77b361e4105e4006a55b6a",
                            "algo": 1
                        },
                        {
                            "value": "9506fb14511b080cad9446279e68a3c8",
                            "algo": 2
                        }
                    ],
                    "fileDate": {
                        "$date": "2023-01-01T16:19:51.07Z"
                    },
                    "fileLength": 122261,
                    "downloadCount": 22181,
                    "fileSizeOnDisk": null,
                    "downloadUrl": "https://edge.forgecdn.net/files/4285/139/hats-and-cosmetics-1.4-1.18.2.jar",
                    "gameVersions": [
                        "1.18.2",
                        "Forge"
                    ],
                    "sortableGameVersions": [
                        {
                            "gameVersionName": "1.18.2",
                            "gameVersionPadded": "0000000001.0000000018.0000000002",
                            "gameVersion": "1.18.2",
                            "gameVersionReleaseDate": {
                                "$date": "2022-02-28T14:23:37.723Z"
                            },
                            "gameVersionTypeId": 73250
                        },
                        {
                            "gameVersionName": "Forge",
                            "gameVersionPadded": "0",
                            "gameVersion": "",
                            "gameVersionReleaseDate": {
                                "$date": "2022-10-01T00:00:00Z"
                            },
                            "gameVersionTypeId": 68441
                        }
                    ],
                    "dependencies": [
                        {
                            "modId": 309927,
                            "relationType": 3
                        }
                    ],
                    "exposeAsAlternative": null,
                    "parentProjectFileId": null,
                    "alternateFileId": 0,
                    "isServerPack": false,
                    "serverPackFileId": null,
                    "isEarlyAccessContent": null,
                    "earlyAccessEndDate": null,
                    "fileFingerprint": 4195791776,
                    "modules": [
                        {
                            "name": "META-INF",
                            "fingerprint": 2631189269
                        },
                        {
                            "name": "com",
                            "fingerprint": 1521715468
                        },
                        {
                            "name": "assets",
                            "fingerprint": 459797262
                        },
                        {
                            "name": "data",
                            "fingerprint": 3789743788
                        },
                        {
                            "name": "hat.png",
                            "fingerprint": 1651265609
                        },
                        {
                            "name": "pack.mcmeta",
                            "fingerprint": 2592535084
                        }
                    ]
                }
            ],
            "latestFilesIndexes": [
                {
                    "gameVersion": "1.18.2",
                    "fileId": 4285139,
                    "filename": "hats-and-cosmetics-1.4-1.18.2.jar",
                    "releaseType": 1,
                    "gameVersionTypeId": 73250,
                    "modLoader": 1
                },
                {
                    "gameVersion": "1.19.2",
                    "fileId": 4019148,
                    "filename": "hats-and-cosmetics-1.4-1.19.2.jar",
                    "releaseType": 1,
                    "gameVersionTypeId": 73407,
                    "modLoader": 1
                },
                {
                    "gameVersion": "1.19.1",
                    "fileId": 3913840,
                    "filename": "hats-and-cosmetics-1.2.2-1.19.1.jar",
                    "releaseType": 1,
                    "gameVersionTypeId": 73407,
                    "modLoader": 1
                },
                {
                    "gameVersion": "1.19",
                    "fileId": 3872689,
                    "filename": "hats-and-cosmetics-1.2.1-1.19.jar",
                    "releaseType": 2,
                    "gameVersionTypeId": 73407,
                    "modLoader": 1
                },
                {
                    "gameVersion": "1.18.1",
                    "fileId": 3696688,
                    "filename": "hats-and-cosmetics-0.1.jar",
                    "releaseType": 2,
                    "gameVersionTypeId": 73250,
                    "modLoader": 1
                }
            ],
            "links": {
                "websiteUrl": "https://www.curseforge.com/minecraft/mc-mods/project-594678",
                "wikiUrl": "",
                "issuesUrl": null,
                "sourceUrl": "https://github.com/PurplikDev/Hats-and-Cosmetics"
            },
            "logo": {
                "id": 564613,
                "modId": 594678,
                "title": "637921174757480798.gif",
                "description": "",
                "thumbnailUrl": "https://media.forgecdn.net/avatars/thumbnails/564/613/256/256/637921174757480798_animated.gif",
                "url": "https://media.forgecdn.net/avatars/564/613/637921174757480798.gif"
            },
            "mainFileId": 4285139,
            "name": "project-594678",
            "primaryCategoryId": 424,
            "rating": null,
            "screenshots": [
                {
                    "id": 585790,
                    "modId": 594678,
                    "title": "The 1.4 Update - Rat Update",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/585/790/310/172/1.png",
                    "url": "https://media.forgecdn.net/attachments/585/790/1.png"
                },
                {
                    "id": 585787,
                    "modId": 594678,
                    "title": "The 1.3 Update - Accessory Update",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/585/787/310/172/1.png",
                    "url": "https://media.forgecdn.net/attachments/585/787/1.png"
                },
                {
                    "id": 585786,
                    "modId": 594678,
                    "title": "The 1.2 Update - Rainy Days",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/585/786/310/172/rainy-days-update.png",
                    "url": "https://media.forgecdn.net/attachments/585/786/rainy-days-update.png"
                },
                {
                    "id": 585785,
                    "modId": 594678,
                    "title": "The 1.1 Update",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/585/785/310/172/1.png",
                    "url": "https://media.forgecdn.net/attachments/585/785/1.png"
                },
                {
                    "id": 585784,
                    "modId": 594678,
                    "title": "The 1.0 Update",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/585/784/310/172/the-1.png",
                    "url": "https://media.forgecdn.net/attachments/585/784/the-1.png"
                },
                {
                    "id": 456802,
                    "modId": 594678,
                    "title": "Lab Goggles",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/456/802/310/172/lab-goggles-showcase.png",
                    "url": "https://media.forgecdn.net/attachments/456/802/lab-goggles-showcase.png"
                },
                {
                    "id": 456800,
                    "modId": 594678,
                    "title": "Tophat",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/456/800/310/172/tophat-showcase.png",
                    "url": "https://media.forgecdn.net/attachments/456/800/tophat-showcase.png"
                },
                {
                    "id": 456799,
                    "modId": 594678,
                    "title": "Ushanka",
                    "description": "",
                    "thumbnailUrl": "https://media.forgecdn.net/attachments/thumbnails/456/799/310/172/ushanka-showcase.png",
                    "url": "https://media.forgecdn.net/attachments/456/799/ushanka-showcase.png"
                }
            ],
            "slug": "project-594678",
            "status": 9,
            "summary": "Wearable and fashionable Cosmetics!",
            "sync_at": {
                "$date": "2025-06-17T10:46:29.504Z"
            },
            "thumbsUpCount": 0,
            "translated_summary": null
        }
        "###;

        let m: Mod = serde_json::from_str(json).expect("deserialize mod from json");
        assert_eq!(m.id, 594678);
    }

    #[test]
    fn test_curseforge_file_model() {
        let json = r###"
    {
        "_id": 3913840,
        "alternateFileId": 0,
        "dependencies": [
            {
                "modId": 309927,
                "relationType": 3
            }
        ],
        "displayName": "hats-and-cosmetics-1.2.2-1.19.1",
        "downloadCount": 734,
        "downloadUrl": "https://edge.forgecdn.net/files/3913/840/hats-and-cosmetics-1.2.2-1.19.1.jar",
        "earlyAccessEndDate": null,
        "exposeAsAlternative": null,
        "fileDate": {
            "$date": "2022-08-03T06:26:20Z"
        },
        "fileFingerprint": 1221617322,
        "fileLength": 120660,
        "fileName": "hats-and-cosmetics-1.2.2-1.19.1.jar",
        "fileSizeOnDisk": null,
        "fileStatus": 4,
        "file_cdn_cached": false,
        "found": true,
        "gameId": 432,
        "gameVersions": [
            "Forge",
            "1.19.1"
        ],
        "hashes": [
            {
                "value": "f046f176352dbcfac8fe61bdfa3f8ea5d32f778f",
                "algo": 1
            },
            {
                "value": "40d80421bbac8442b7a88db8add73003",
                "algo": 2
            }
        ],
        "isAvailable": true,
        "isEarlyAccessContent": null,
        "isServerPack": false,
        "modId": 594678,
        "modules": [
            {
                "name": "META-INF",
                "fingerprint": 3791664935
            },
            {
                "name": "com",
                "fingerprint": 2813995546
            },
            {
                "name": "assets",
                "fingerprint": 3455112628
            },
            {
                "name": "data",
                "fingerprint": 3000469934
            },
            {
                "name": "hat.png",
                "fingerprint": 1651265609
            },
            {
                "name": "pack.mcmeta",
                "fingerprint": 2592535084
            }
        ],
        "need_to_cache": true,
        "parentProjectFileId": null,
        "releaseType": 1,
        "serverPackFileId": null,
        "sortableGameVersions": [
            {
                "gameVersionName": "Forge",
                "gameVersionPadded": "0",
                "gameVersion": "",
                "gameVersionReleaseDate": {
                    "$date": "2022-10-01T00:00:00Z"
                },
                "gameVersionTypeId": 68441
            },
            {
                "gameVersionName": "1.19.1",
                "gameVersionPadded": "0000000001.0000000019.0000000001",
                "gameVersion": "1.19.1",
                "gameVersionReleaseDate": {
                    "$date": "2022-10-01T00:00:00Z"
                },
                "gameVersionTypeId": 73407
            }
        ],
        "sync_at": {
            "$date": "2024-10-26T12:33:55Z"
        },
        "md5": "40d80421bbac8442b7a88db8add73003",
        "sha1": "f046f176352dbcfac8fe61bdfa3f8ea5d32f778f"
    }
        "###;

        let f: File = serde_json::from_str(json).expect("deserialize file from json");
        assert_eq!(f.id, 3913840);
        assert_eq!(f.mod_id, 594678);
    }

    #[test]
    fn test_curseforge_category_model() {
        let json = r###"
            {
        "_id": 6946,
        "classId": 6945,
        "dateModified": {
            "$date": "2024-02-06T12:41:02.673Z"
        },
        "displayIndex": 0,
        "gameId": 432,
        "iconUrl": "https://media.forgecdn.net/avatars/944/327/638428200626712952.png",
        "isClass": null,
        "name": "Mod Support",
        "parentCategoryId": 6945,
        "slug": "mod-support",
        "sync_at": {
            "$date": "2025-06-18T16:00:01.248Z"
        },
        "url": "https://www.curseforge.com/minecraft/data-packs/mod-support"
    }
        "###;

        let c: Category = serde_json::from_str(json).expect("deserialize category from json");
        assert_eq!(c.id, 6946);
        assert_eq!(c.name, "Mod Support");
    }
}
