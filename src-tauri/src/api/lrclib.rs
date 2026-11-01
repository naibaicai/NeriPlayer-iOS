// LRCLIB 歌词 API（开放 API，无鉴权）
use reqwest::Client;
use serde::{Deserialize, Serialize};

use crate::api::transport::{parse_json_response, FallbackHttp};
use crate::error::AppResult;

const BASE_URL: &str = "https://lrclib.net/api";

pub struct LrcLibClient {
    http: FallbackHttp,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LrcLibResult {
    pub id: u64,
    #[serde(rename = "trackName")]
    pub track_name: String,
    #[serde(rename = "artistName")]
    pub artist_name: String,
    pub duration: f64,
    #[serde(rename = "syncedLyrics")]
    pub synced_lyrics: Option<String>,
    #[serde(rename = "plainLyrics")]
    pub plain_lyrics: Option<String>,
}

impl LrcLibClient {
    pub fn with_transport(http: FallbackHttp) -> Self {
        Self { http }
    }

    pub fn new(http: &Client) -> Self {
        Self::with_transport(FallbackHttp::new(http, "lrclib"))
    }

    /// 精确匹配歌词
    pub async fn get_lyrics(
        &self,
        track: &str,
        artist: &str,
        duration_secs: u64,
    ) -> AppResult<Option<LrcLibResult>> {
        let duration_text = duration_secs.to_string();
        let resp = self
            .http
            .send(|client| {
                client
                    .get(format!("{}/get", BASE_URL))
                    .query(&[
                        ("track_name", track),
                        ("artist_name", artist),
                        ("duration", duration_text.as_str()),
                    ])
                    .header("User-Agent", "NeriPlayer/1.0.0")
            })
            .await?;

        if resp.status() == 404 {
            return Ok(None);
        }

        // 统一走传输层解析：429/5xx/HTML 错误页会给出可读原因，而不是 JSON 语法错
        let result: LrcLibResult = parse_json_response(resp, "lrclib get").await?;
        Ok(Some(result))
    }

    /// 模糊搜索歌词
    pub async fn search(&self, query: &str) -> AppResult<Vec<LrcLibResult>> {
        let resp = self
            .http
            .send(|client| {
                client
                    .get(format!("{}/search", BASE_URL))
                    .query(&[("q", query)])
                    .header("User-Agent", "NeriPlayer/1.0.0")
            })
            .await?;

        let results: Vec<LrcLibResult> = parse_json_response(resp, "lrclib search").await?;
        Ok(results)
    }
}
