use serde::{Deserialize, Serialize};

/// Which site a result is from and site-specific information.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "site", content = "site_info")]
pub enum SiteInfo {
    FurAffinity(FurAffinityFile),
    #[serde(rename = "e621")]
    E621(E621File),
    Twitter,
    Weasyl,
}

/// Information about a file from `FurAffinity`.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FurAffinityFile {
    /// The ID of the file on `FurAffinity` as seen in the image URL.
    /// This is not the same as the submission ID.
    pub file_id: i32,
}

/// Information about a file from e621.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct E621File {
    /// A list of sources from e621.
    pub sources: Option<Vec<String>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Rating {
    General,
    Mature,
    Adult,
}

/// Information about a matching image.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct File {
    /// The site-specific ID.
    pub site_id: i64,
    /// Direct link to the submission image.
    pub url: String,
    /// Filename of the submission.
    pub filename: String,
    /// Optional list of artists who may have created the image.
    pub artists: Option<Vec<String>>,
    /// Optional rating of the submission.
    pub rating: Option<Rating>,
    /// Optional date that the submission was posted.
    pub posted_at: Option<chrono::DateTime<chrono::Utc>>,
    /// Tags associated with the post, if applicable.
    pub tags: Option<Vec<String>>,
    /// SHA256 hash of the associated image, if known.
    #[serde(with = "opt_hex_u8")]
    pub sha256: Option<Vec<u8>>,
    /// Hash of the image. Only returned in some endpoints.
    pub hash: Option<i64>,
    /// Distance of the image compared to the input. Only returned in some endpoints.
    pub distance: Option<u64>,
    /// Site specific information.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(flatten)]
    pub site_info: Option<SiteInfo>,
    /// The hash that retreived this result. Only returned in some endpoints.
    pub searched_hash: Option<i64>,
}

mod opt_hex_u8 {
    use serde::Deserialize;

    #[allow(clippy::ref_option)]
    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match bytes {
            Some(bytes) => serializer.serialize_str(&hex::encode(bytes)),
            None => serializer.serialize_none(),
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let val = <Option<String>>::deserialize(deserializer)?
            .map(hex::decode)
            .transpose()
            .map_err(serde::de::Error::custom)?;

        Ok(val)
    }
}

impl File {
    /// Get the human readable name of the site.
    pub fn site_name(&self) -> &'static str {
        match &self.site_info {
            Some(SiteInfo::Twitter) => "Twitter",
            Some(SiteInfo::FurAffinity(_)) => "FurAffinity",
            Some(SiteInfo::E621(_)) => "e621",
            Some(SiteInfo::Weasyl) => "Weasyl",
            _ => unreachable!("Search result was missing SiteInfo"),
        }
    }

    /// Get a link to the image's source page.
    pub fn url(&self) -> String {
        match &self.site_info {
            Some(SiteInfo::Twitter) => format!(
                "https://twitter.com/{}/status/{}",
                self.artists
                    .as_ref()
                    .expect("exists")
                    .iter()
                    .next()
                    .expect("exists"),
                self.site_id
            ),
            Some(SiteInfo::FurAffinity(_)) => {
                format!("https://www.furaffinity.net/view/{}/", self.site_id)
            }
            Some(SiteInfo::E621(_)) => format!("https://e621.net/posts/{}", self.site_id),
            Some(SiteInfo::Weasyl) => format!("https://www.weasyl.com/view/{}/", self.site_id),
            _ => unreachable!("Search result was missing SiteInfo"),
        }
    }

    /// Generate a unique ID for the submission.
    pub fn id(&self) -> String {
        format!("{}-{}", self.site_name(), self.site_id)
    }
}

/// Information about a matching `FurAffinity` file.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct FurAffinityFileDetail {
    pub artist: Option<String>,
    pub deleted: bool,
    pub file_id: Option<i32>,
    pub file_size: Option<i32>,
    pub filename: Option<String>,
    pub hash: Option<i64>,
    pub hash_str: Option<String>,
    pub id: i32,
    pub posted_at: Option<chrono::DateTime<chrono::Utc>>,
    pub rating: Option<Rating>,
    #[serde(with = "opt_hex_u8")]
    pub sha256: Option<Vec<u8>>,
    pub tags: Vec<String>,
    pub updated_at: Option<chrono::DateTime<chrono::Utc>>,
    pub url: Option<String>,
}

/// Container for multiple matches. Includes the hash of the image sent.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Matches {
    /// Hash of the sent image.
    pub hash: i64,
    /// A list of potential matches.
    pub matches: Vec<File>,
}
