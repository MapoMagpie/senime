use std::path::PathBuf;

use serde::{Deserialize, Serialize, Serializer};

use crate::measurement::Measurement;

#[derive(Debug)]
pub enum JsError {
    /// IO 错误（读取配置文件失败等）
    Io(std::io::Error),
    /// TOML 解析错误
    Toml(toml::de::Error),
    /// HTTP 请求错误
    Http(ureq::Error),
    /// JSON 解析错误
    Json(serde_json::Error),
    /// API 返回了错误码
    Api(String),
}

impl std::fmt::Display for JsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsError::Io(e) => write!(f, "JS bridge IO 错误: {e}"),
            JsError::Toml(e) => write!(f, "JS bridge 配置文件解析错误: {e}"),
            JsError::Http(e) => write!(f, "JS bridge HTTP 请求错误: {e}"),
            JsError::Json(e) => write!(f, "JS bridge JSON 解析错误: {e}"),
            JsError::Api(msg) => write!(f, "JS bridge API 错误: {msg}"),
        }
    }
}

impl std::error::Error for JsError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            JsError::Io(e) => Some(e),
            JsError::Toml(e) => Some(e),
            JsError::Http(e) => Some(e),
            JsError::Json(e) => Some(e),
            _ => None,
        }
    }
}

impl From<std::io::Error> for JsError {
    fn from(e: std::io::Error) -> Self {
        JsError::Io(e)
    }
}

impl From<toml::de::Error> for JsError {
    fn from(e: toml::de::Error) -> Self {
        JsError::Toml(e)
    }
}

impl From<ureq::Error> for JsError {
    fn from(e: ureq::Error) -> Self {
        JsError::Http(e)
    }
}

impl From<serde_json::Error> for JsError {
    fn from(e: serde_json::Error) -> Self {
        JsError::Json(e)
    }
}

#[derive(Deserialize)]
#[serde(default)]
pub struct JSSettings {
    ime: String,
    token: String,
    subversions: usize,
    version: String,
    from: String,
}

impl Default for JSSettings {
    fn default() -> Self {
        Self {
            ime: "五笔字形".to_string(),
            token: Default::default(),
            subversions: 17108,
            version: "v2.1.6".to_string(),
            from: "web".to_string(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum JSAction {
    Random,
    Daily,
    DailyOnce,
    None,
}

impl std::str::FromStr for JSAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "random" => Ok(JSAction::Random),
            "daily" => Ok(JSAction::Daily),
            "dailyonce" => Ok(JSAction::DailyOnce),
            "none" => Ok(JSAction::None),
            _ => Err(format!(
                "无效的 js-action: {s}，应为 random, daily, dailyonce, none"
            )),
        }
    }
}

pub struct JSContent {
    pub title: String,
    pub content: String,
    pub is_local: bool,
}

/// 根据路径获取JSSettings
/// 若`path.is_some`，则从其中加载`JSSettings`，若文件不存在或为非法`TOML`内容，则抛出异常
/// 若`path.is_none`，则从尝试从默认路径加载`JSSettings`，若默认路径不存在`js-settings.toml`，则返回`None`
/// 若存在默认路径，但是非法`TOML`内容，则抛出异常
pub fn js_get_settings<P: Into<PathBuf>>(path: Option<P>) -> Result<Option<JSSettings>, JsError> {
    match path {
        Some(p) => {
            let path = p.into();
            let content = std::fs::read_to_string(&path)?;
            let settings: JSSettings = toml::from_str(&content)?;
            Ok(Some(settings))
        }
        None => {
            let path = match dirs::config_dir().map(|d| d.join("senime").join("js-settings.toml")) {
                Some(p) if p.is_file() => p,
                _ => return Ok(None),
            };
            let content = std::fs::read_to_string(&path)?;
            let settings: JSSettings = toml::from_str(&content)?;
            Ok(Some(settings))
        }
    }
}

pub fn js_get_content(settings: &JSSettings, action: JSAction) -> Result<JSContent, JsError> {
    // 构建请求体：基础字段来自 settings，timestamp 就地获取
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // 根据`action`选择 API 端点
    let (endpoint, body) = match action {
        JSAction::Daily | JSAction::DailyOnce => {
            let competition_type = if JSAction::DailyOnce == action { 2 } else { 0 };
            let body = serde_json::json!({
                "competitionType": competition_type,
                "snumflag": "1",
                "from": settings.from,
                "timestamp": timestamp,
                "version": settings.version,
                "subversions": settings.subversions,
                "token": settings.token,
            })
            .to_string();
            ("/Api/Text/getContent", body)
        }
        JSAction::Random => {
            let body = serde_json::json!({
                "from": settings.from,
                "timestamp": timestamp,
                "version": settings.version,
                "subversions": settings.subversions,
                "token": settings.token,
            })
            .to_string();
            ("/Api/Text/getRandomText", body)
        }
        JSAction::None => unreachable!(),
    };

    // 加密请求体
    let encrypted = encrypt(body);

    // 以同步 POST 请求 API
    let url = format!("https://www.jsxiaoshi.com/index.php{endpoint}");
    let response = ureq::post(&url)
        .header(
            "User-Agent",
            "Mozilla/5.0 (X11; Linux x86_64; rv:152.0) Gecko/20100101 Firefox/152.0",
        )
        .header("Accept", "application/json, text/plain, */*")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Referer", "https://www.52dazi.cn/")
        .send(&encrypted)?;

    // 解析响应：a_name → title, a_content → content（Random 用 name/content）
    let mut body = response.into_body();
    let body_str = body.read_to_string()?;
    let json: serde_json::Value = serde_json::from_str(&body_str)?;

    if json["error"] != 0 {
        if let Some(msg) = json["msg"].as_str() {
            return Err(JsError::Api(msg.to_string()));
        } else {
            return Err(JsError::Api("API 返回非零错误码".to_string()));
        }
    }

    let msg = &json["msg"];
    let content = JSContent {
        title: msg["a_name"]
            .as_str()
            .or_else(|| msg["name"].as_str())
            .unwrap_or_default()
            .to_string(),
        content: msg["a_content"]
            .as_str()
            .or_else(|| msg["content"].as_str())
            .unwrap_or_default()
            .to_string(),
        is_local: false,
    };

    // 7. 返回 settings 和 content
    Ok(content)
}

/// 请求上报 API 并返回 (返回信息, 异常信息)
///
/// 任何错误（HTTP 状态码异常如 404、网络错误、响应体读取失败、JSON 解析失败、
/// API 返回非零错误码）都会被捕获到“异常信息”中返回，不会向上抛出。
fn report_api(url: &str, body: String) -> (String, String) {
    const UA: &'static str =
        "Mozilla/5.0 (X11; Linux x86_64; rv:153.0) Gecko/20100101 Firefox/153.0";
    const REFERER: &'static str = "https://www.52dazi.cn/";
    const CT: &'static str = "application/x-www-form-urlencoded";
    let resp = match ureq::post(url)
        .header("User-Agent", UA)
        .header("Accept", "application/json, text/plain, */*")
        .header("Content-Type", CT)
        .header("Referer", REFERER)
        .send(body)
    {
        Ok(resp) => resp,
        Err(e) => return (String::new(), format!("{e}")),
    };
    let body_str = match resp.into_body().read_to_string() {
        Ok(s) => s,
        Err(e) => return (String::new(), format!("{e}")),
    };
    let json: serde_json::Value = match serde_json::from_str(&body_str) {
        Ok(json) => json,
        Err(e) => return (body_str, format!("{e}")),
    };
    // API 返回非零错误码时，将错误信息记入“异常信息”
    if json["error"].as_i64().is_some_and(|e| e != 0) {
        let msg = json["msg"]
            .as_str()
            .unwrap_or("API 返回非零错误码")
            .to_string();
        return (String::new(), msg);
    }
    let msg = json["msg"].as_str().unwrap_or_default().to_string();
    (msg, String::new())
}

pub fn js_report(settings: &JSSettings, mea: &Measurement, content: &JSContent) -> String {
    let mut lines = Vec::new();
    {
        let incr_user_record = IncrUserRecord::new(settings, mea);
        let body = encrypt(serde_json::to_string(&incr_user_record).unwrap_or_default());
        let api = "https://www.jsxiaoshi.com/index.php/Api/User/incrUserRecord";
        let (ret, err) = report_api(api, body);
        lines.push(format!("{:<14} : {ret}{err}", "incrUserRecord"));
    }

    if !content.is_local {
        let upload_result = UploadResult::new(settings, mea, content);
        let body = encrypt(serde_json::to_string(&upload_result).unwrap_or_default());
        let api = "https://www.jsxiaoshi.com/index.php/Api/Rank/uploadResult";
        let (ret, err) = report_api(api, body);
        lines.push(format!("{:<14} : {ret}{err}", "uploadResult"));
    };

    {
        let upload_record = UploadRecord::new(settings, mea, content);
        let body = encrypt(serde_json::to_string(&upload_record).unwrap_or_default());
        let api = "https://www.jsxiaoshi.com/index.php/Api/Record/uploadRecord";
        let (ret, err) = report_api(api, body);
        lines.push(format!("{:<14} : {ret}{err}", "uploadRecord"));
    }

    lines.join("\n")
}

// {"incrDailyRecord":300,"incrTotalKeystrokes":805,"incrTotalTime":162.89,"incrTotalWordNum":280,"from":"web","timestamp":1784354377,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IncrUserRecord {
    incr_daily_record: usize,
    incr_total_keystrokes: usize,
    #[serde(serialize_with = "serialize_f32_2")]
    incr_total_time: f32,
    incr_total_word_num: usize,
    from: String,
    timestamp: usize,
    version: String,
    subversions: usize,
    token: String,
}

impl IncrUserRecord {
    fn new(settings: &JSSettings, mea: &Measurement) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        Self {
            incr_daily_record: mea.text_wc + mea.bs_times,
            incr_total_keystrokes: mea.code_cc,
            incr_total_time: mea.duration.as_secs_f32(),
            incr_total_word_num: mea.text_wc,
            from: settings.from.clone(),
            timestamp,
            version: settings.version.clone(),
            subversions: settings.subversions,
            token: settings.token.clone(),
        }
    }
}

// {"challengeFlag":0,"textTitle":"晚安","speed":103.14,"keystrokes":4.94,"maChang":2.88,"wordNum":280,"typingTime":"02:42.890","huiGai":20,"huiChe":0,"jianShu":805,"jianZhun":"85.39%","repeatNum":0,"daCi":"47.86%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":538,"keyMethod":"+100.00%","isFirstSubmit":1,"isGroupText":0,"accuracy":85.39,"from":"web","timestamp":1784354377,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UploadResult {
    challenge_flag: usize,
    text_title: String,
    #[serde(flatten)]
    measure: JSMeasurement,
    key_method: String,
    is_first_submit: usize,
    is_group_text: usize,
    #[serde(serialize_with = "serialize_f32_2")]
    accuracy: f32,
    from: String,
    timestamp: usize,
    version: String,
    subversions: usize,
    token: String,
}

impl UploadResult {
    fn new(settings: &JSSettings, mea: &Measurement, content: &JSContent) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let measure = JSMeasurement::new(mea, &settings.ime);
        Self {
            challenge_flag: 0,
            text_title: content.title.clone(),
            measure,
            key_method: "+100.00%".to_string(),
            is_first_submit: 1,
            is_group_text: 0,
            accuracy: mea.accuracy,
            from: settings.from.clone(),
            timestamp,
            version: settings.version.clone(),
            subversions: settings.subversions,
            token: settings.token.clone(),
        }
    }
}
// {"content":"我说大概我真的累坏了","textTitle":"晚安","speed":103.14,"keystrokes":4.94,"maChang":2.88,"wordNum":280,"typingTime":"02:42.890","huiGai":20,"huiChe":0,"jianShu":805,"jianZhun":"85.39%","repeatNum":0,"daCi":"47.86%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":538,"keyMethod":"+100.00%","isSystemText":1,"from":"web","timestamp":1784354377,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UploadRecord {
    content: String,
    text_title: String,
    #[serde(flatten)]
    measure: JSMeasurement,
    key_method: String,
    is_system_text: usize,
    from: String,
    timestamp: usize,
    version: String,
    subversions: usize,
    token: String,
}

impl UploadRecord {
    fn new(settings: &JSSettings, mea: &Measurement, content: &JSContent) -> Self {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs() as usize;
        let measure = JSMeasurement::new(mea, &settings.ime);
        let is_system_text = if content.is_local { 0 } else { 1 };
        Self {
            content: content.content.clone(),
            text_title: content.title.clone(),
            measure,
            key_method: "+100.00%".to_string(),
            is_system_text,
            from: settings.from.clone(),
            timestamp,
            version: settings.version.clone(),
            subversions: settings.subversions,
            token: settings.token.clone(),
        }
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct JSMeasurement {
    #[serde(serialize_with = "serialize_f32_2")]
    speed: f32,
    #[serde(serialize_with = "serialize_f32_2")]
    keystrokes: f32,
    #[serde(serialize_with = "serialize_f32_2")]
    ma_chang: f32,
    word_num: usize,
    typing_time: String,
    hui_gai: usize,
    hui_che: usize,
    jian_shu: usize,
    jian_zhun: String,
    repeat_num: usize,
    da_ci: String,
    wrong_num: usize,
    input_method: String,
    backspace: usize,
    xuan_chong: usize,
}

impl JSMeasurement {
    fn new(mea: &Measurement, ime: &str) -> Self {
        let typing_time = format!(
            "{:02}:{:02}.{:03}",
            mea.duration.as_secs() / 60,
            mea.duration.as_secs() % 60,
            mea.duration.subsec_millis()
        );
        Self {
            speed: mea.wpm,
            keystrokes: mea.kps,
            ma_chang: mea.avg_len,
            word_num: mea.text_wc,
            typing_time,
            hui_gai: mea.bs_times,
            hui_che: 0,
            jian_shu: mea.code_cc,
            jian_zhun: format!("{:.2}%", mea.accuracy),
            repeat_num: 0,
            da_ci: format!("{:.2}%", mea.wg_freq),
            wrong_num: 0,
            input_method: ime.to_string(),
            backspace: mea.bs_times,
            xuan_chong: mea.se_times,
        }
    }
}

fn serialize_f32_2<S>(v: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 四舍五入到两位
    let v = (v * 100.0).round() / 100.0;
    s.serialize_f32(v)
}

fn encrypt(body: String) -> String {
    use aes::cipher::{BlockEncryptMut, KeyIvInit, block_padding::ZeroPadding};

    let key = b"c9ec834c80f77237";
    let iv = b"db4d6bfde3057dca";

    // ZeroPadding 要求缓冲区预先填充到块大小的整数倍
    let body_bytes = body.as_bytes();
    let padded_len = body_bytes.len().div_ceil(16) * 16;
    let mut buf = vec![0u8; padded_len];
    buf[..body_bytes.len()].copy_from_slice(body_bytes);

    let ciphertext = cbc::Encryptor::<aes::Aes128>::new(key.into(), iv.into())
        .encrypt_padded_mut::<ZeroPadding>(&mut buf, body_bytes.len())
        .expect("AES-128-CBC 加密应成功");

    use base64::Engine;
    base64::engine::general_purpose::STANDARD.encode(ciphertext)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_encrypt() {
        let body = r#"{"competitionType":0,"snumflag":"1","from":"web","timestamp":1784350730,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}"#.to_string();
        let expected = "0hv2w3UU00zcNMoK7Ic7oMTP9yGUa1M0Ng7JcNzRli0vJv9BOa8WoM7qMYZhXVs1QsP+zpK/qO5zsQWUulXhrJ6F5AOQcbT/8zcEXRduunS2/PgY6vOFjT/Z7GRJEtrvwLRo8kV6ij8l8U5Uda+0x8/XI2kBUCWyo1oqxPJVGJRVLMVSopKJt5Q/gIxXK65a";
        let result = encrypt(body);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_incr_user_record_serialization() {
        let record = IncrUserRecord {
            incr_daily_record: 300,
            incr_total_keystrokes: 805,
            incr_total_time: 162.89,
            incr_total_word_num: 280,
            from: "web".to_string(),
            timestamp: 1784354377,
            version: "v2.1.6".to_string(),
            subversions: 17108,
            token: "7d670b541f0b8".to_string(),
        };
        let json = serde_json::to_string(&record).unwrap();
        let expected = r#"{"incrDailyRecord":300,"incrTotalKeystrokes":805,"incrTotalTime":162.89,"incrTotalWordNum":280,"from":"web","timestamp":1784354377,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_upload_result_serialization() {
        let measure = JSMeasurement {
            speed: 103.14,
            keystrokes: 4.94,
            ma_chang: 2.88,
            word_num: 280,
            typing_time: "02:42.890".to_string(),
            hui_gai: 20,
            hui_che: 0,
            jian_shu: 805,
            jian_zhun: "85.39%".to_string(),
            repeat_num: 0,
            da_ci: "47.86%".to_string(),
            wrong_num: 0,
            input_method: "虎码".to_string(),
            backspace: 0,
            xuan_chong: 538,
        };
        let result = UploadResult {
            challenge_flag: 0,
            text_title: "晚安".to_string(),
            measure,
            key_method: "+100.00%".to_string(),
            is_first_submit: 1,
            is_group_text: 0,
            accuracy: 85.39,
            from: "web".to_string(),
            timestamp: 1784354377,
            version: "v2.1.6".to_string(),
            subversions: 17108,
            token: "7d670b541f0b8".to_string(),
        };
        let json = serde_json::to_string(&result).unwrap();
        let expected = r#"{"challengeFlag":0,"textTitle":"晚安","speed":103.14,"keystrokes":4.94,"maChang":2.88,"wordNum":280,"typingTime":"02:42.890","huiGai":20,"huiChe":0,"jianShu":805,"jianZhun":"85.39%","repeatNum":0,"daCi":"47.86%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":538,"keyMethod":"+100.00%","isFirstSubmit":1,"isGroupText":0,"accuracy":85.39,"from":"web","timestamp":1784354377,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}"#;
        assert_eq!(json, expected);
    }

    #[test]
    fn test_upload_record_serialization() {
        let measure = JSMeasurement {
            speed: 103.14,
            keystrokes: 4.94,
            ma_chang: 2.88,
            word_num: 280,
            typing_time: "02:42.890".to_string(),
            hui_gai: 20,
            hui_che: 0,
            jian_shu: 805,
            jian_zhun: "85.39%".to_string(),
            repeat_num: 0,
            da_ci: "47.86%".to_string(),
            wrong_num: 0,
            input_method: "虎码".to_string(),
            backspace: 0,
            xuan_chong: 538,
        };
        let record = UploadRecord {
            content: "我说大概我真的累坏了".to_string(),
            text_title: "晚安".to_string(),
            measure,
            key_method: "+100.00%".to_string(),
            is_system_text: 1,
            from: "web".to_string(),
            timestamp: 1784354377,
            version: "v2.1.6".to_string(),
            subversions: 17108,
            token: "7d670b541f0b8".to_string(),
        };
        let json = serde_json::to_string(&record).unwrap();
        let expected = r#"{"content":"我说大概我真的累坏了","textTitle":"晚安","speed":103.14,"keystrokes":4.94,"maChang":2.88,"wordNum":280,"typingTime":"02:42.890","huiGai":20,"huiChe":0,"jianShu":805,"jianZhun":"85.39%","repeatNum":0,"daCi":"47.86%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":538,"keyMethod":"+100.00%","isSystemText":1,"from":"web","timestamp":1784354377,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}"#;
        assert_eq!(json, expected);
    }

    // fn gen_random(low: f32, hight: f32) -> f32 {
    //     rand::random_range(low..=hight)
    // }

    // #[test]
    // fn test_js_report() {
    //     let text_wc = gen_random(9000., 10000.) as usize;
    //     // 随机1-3 作为文章的后缀
    //     let article_no = gen_random(1., 3.) as usize;
    //     // 生成9000-11000 字符数量的文本
    //     let content = gen_article_by_file(text_wc, &format!("article_{article_no}.txt"));
    //     // 随机码长 2.9-3.2
    //     let avg_len = gen_random(2.0, 2.8);
    //     // 随机回退次数
    //     let bs_times = (gen_random(3., 3.9) / 100. * text_wc as f32) as usize;
    //     // 随机空格次数
    //     let sp_times = (gen_random(4., 4.9) / 100. * text_wc as f32) as usize;
    //     // 随机候选次数
    //     let se_times = (gen_random(2., 2.9) / 100. * text_wc as f32) as usize;
    //     // 根据码长和字符数量计算对应的键数，并且增加2。0%-2.9%的数量
    //     let code_cc = (text_wc as f32 * avg_len) as usize + bs_times + sp_times;
    //     // 随机每秒击键数 3.9 - 4.6
    //     let kps = gen_random(4.2, 4.8);
    //     // 根据键数和每秒击键数计算需要花费的秒数
    //     let duration_secs = (code_cc as f32 / kps) as u64;
    //     // 根据花费的秒数和字符数量计算每分钟的字符数
    //     let wpm = (text_wc as f32 / duration_secs as f32) * 60.0;
    //     // 随机打词率
    //     let wg_freq = gen_random(66., 75.);
    //     // 根据打词率计算打单次数
    //     let si_times = ((wg_freq / 100.) * text_wc as f32) as usize;
    //     // 根据回退次数和总键数计算键准
    //     let accuracy = ((code_cc - bs_times) as f32 / code_cc as f32) * 100.0;

    //     let settings = js_get_settings::<String>(None).unwrap().unwrap();
    //     let mea = Measurement {
    //         duration: Duration::from_secs(duration_secs),
    //         pause_duration: Duration::from_secs(0),
    //         text_wc,
    //         code_cc,
    //         preset_wc: None,
    //         preset_avg_len: None,
    //         kps,
    //         wpm,
    //         avg_len,
    //         counted: 0,
    //         bs_times,
    //         sp_times,
    //         se_times,
    //         accuracy,
    //         si_times,
    //         wg_freq: wg_freq,
    //         wrong_count: bs_times,
    //         records: vec![],
    //         diff_slab: vec![],
    //     };
    //     let content = JSContent {
    //         title: "自由发文".to_string(),
    //         content,
    //         is_local: true,
    //     };
    //     // let incr_user_record = IncrUserRecord::new(&settings, &mea);
    //     // let incr_user_record_body = serde_json::to_string(&incr_user_record).unwrap();
    //     // println!("{incr_user_record_body}");
    //     // let upload_record = UploadRecord::new(&settings, &mea, &content);
    //     // let upload_record_body = serde_json::to_string(&upload_record).unwrap();
    //     // println!("{upload_record_body}");
    //     println!("{}", js_report(&settings, &mea, &content));
    // }
}
