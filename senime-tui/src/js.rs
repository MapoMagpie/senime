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
    /// 无效的 action
    NoAction,
}

impl std::fmt::Display for JsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            JsError::Io(e) => write!(f, "JS bridge IO 错误: {e}"),
            JsError::Toml(e) => write!(f, "JS bridge 配置文件解析错误: {e}"),
            JsError::Http(e) => write!(f, "JS bridge HTTP 请求错误: {e}"),
            JsError::Json(e) => write!(f, "JS bridge JSON 解析错误: {e}"),
            JsError::Api(msg) => write!(f, "JS bridge API 错误: {msg}"),
            JsError::NoAction => write!(f, "JS bridge: 无效的 action"),
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
    #[allow(unused)]
    None,
}

impl std::str::FromStr for JSAction {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "random" => Ok(JSAction::Random),
            "daily" => Ok(JSAction::Daily),
            "dailyonce" => Ok(JSAction::DailyOnce),
            _ => Err(format!(
                "无效的 js-action: {s}，应为 random , daily, dailyonce"
            )),
        }
    }
}

pub struct JSContent {
    pub title: String,
    pub content: String,
}

pub fn js_get_content(
    settings_path: &str,
    action: JSAction,
) -> Result<(JSSettings, JSContent), JsError> {
    // 1. 从`settings_path`读取`JSSettings`
    let settings_str = std::fs::read_to_string(settings_path)?;
    let settings: JSSettings = toml::from_str(&settings_str)?;
    // 3. 构建请求体：基础字段来自 settings，timestamp 就地获取
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // 2. 根据`action`选择 API 端点
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
        JSAction::None => return Err(JsError::NoAction),
    };

    // 4. 加密请求体
    let encrypted = encrypt(body);

    // 5. 以同步 POST 请求 API
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

    // 6. 解析响应：a_name → title, a_content → content（Random 用 name/content）
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
    };

    // 7. 返回 settings 和 content
    Ok((settings, content))
}

pub fn js_report(
    settings: &JSSettings,
    _action: JSAction,
    mea: &Measurement,
    content: &JSContent,
) -> Result<String, JsError> {
    let ua = "Mozilla/5.0 (X11; Linux x86_64; rv:152.0) Gecko/20100101 Firefox/152.0";
    let referer = "https://www.52dazi.cn/";
    let ct = "application/x-www-form-urlencoded";

    // api: /Api/User/incrUserRecord
    let incr_user_record = IncrUserRecord::new(settings, mea);
    let body = serde_json::to_string(&incr_user_record)?;
    let encrypted = encrypt(body);
    ureq::post("https://www.jsxiaoshi.com/index.php/Api/User/incrUserRecord")
        .header("User-Agent", ua)
        .header("Accept", "application/json, text/plain, */*")
        .header("Content-Type", ct)
        .header("Referer", referer)
        .send(&encrypted)?;

    // api: /Api/Rank/uploadResult
    let upload_result = UploadResult::new(settings, mea, content);
    let body = serde_json::to_string(&upload_result)?;
    let encrypted = encrypt(body);
    let resp = ureq::post("https://www.jsxiaoshi.com/index.php/Api/Rank/uploadResult")
        .header("User-Agent", ua)
        .header("Accept", "application/json, text/plain, */*")
        .header("Content-Type", ct)
        .header("Referer", referer)
        .send(&encrypted)?;
    let body_str = resp.into_body().read_to_string()?;
    let json: serde_json::Value = serde_json::from_str(&body_str)?;

    let message = json["msg"].to_string();

    // api: /Api/Record/uploadRecord
    let upload_record = UploadRecord::new(settings, mea, content);
    let body = serde_json::to_string(&upload_record)?;
    let encrypted = encrypt(body);
    ureq::post("https://www.jsxiaoshi.com/index.php/Api/Record/uploadRecord")
        .header("User-Agent", ua)
        .header("Accept", "application/json, text/plain, */*")
        .header("Content-Type", ct)
        .header("Referer", referer)
        .send(&encrypted)?;

    Ok(message)
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
        Self {
            content: content.content.clone(),
            text_title: content.title.clone(),
            measure,
            key_method: "+100.00%".to_string(),
            is_system_text: 1,
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

    #[test]
    fn test_js_get_content() {
        let ret = js_get_content("../test/js-settings.toml", JSAction::Daily);
        match ret {
            Ok((settings, content)) => {
                println!("ime: {}, token: {}", settings.ime, settings.token);
                println!("{}\n{}", content.title, content.content);
            }
            Err(err) => {
                eprintln!("{err}");
            }
        }
    }
}
// 锦标赛
// api:  /Api/Text/getContent
// {"competitionType":2,"snumflag":"1","from":"web","timestamp":1784375069,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
// {
// 	"error": 0,
// 	"msg": {
// 		"a_name": "锦标赛第3255期",
// 		"a_content": "作为一个社恐患者，我总习惯独处，害怕拥挤的人群，害怕无意义的社交，就连去超市买东西，都要提前在心里想好要说的话，反复练习，生怕和别人产生过多的交集。周末的午后，我总喜欢窝在小小的出租屋里，泡一杯淡淡的绿茶，看一本喜欢的书，听着舒缓的轻音乐，享受属于自己的安静时光，这对我来说，是最放松、最自在的时刻。身边总有人说我孤僻，说我不合群，劝我多出去走走，多认识一些朋友，可只有我自己知道，独处从不是孤独，而是与自己对话的最好方式，是沉淀自己的最佳时机。独处时，我可以静下心来思考人生，梳理自己的情绪，不必迎合别人的喜好，不必伪装自己的模样，不必强迫自己融入不适合的圈子，做最真实的自己。我会在独处时学着做饭，从一开始的手忙脚乱，到后来能做出几样可口的家常菜，看着食材在锅里慢慢变成美味的菜肴，心里满是成就感；我会在独处时打理室内绿植，给绿萝浇水、修剪枝叶，看着绿萝抽出新的枝叶，慢慢爬满窗台，感受生命的力量和美好；我会在独处时写下自己的心情，把那些不敢说的话、藏在心底的情绪，都藏在文字里，让文字成为自己的情绪出口。慢慢的我发现，社恐并不可怕，不必强迫自己去迎合别人，不必勉强自己去做不喜欢的事情。真正的成熟，是学会接纳自己的不完美，学会享受独处的时光，在独处中沉淀自己，在安静中找到内心的力量。那些独处的时光，没有喧嚣，没有纷扰，只有自己和自己的对话，终会让我们成为更从容、更强大、更通透的自己，也会让我们更懂得珍惜那些真正值得的人和事。",
// 		"a_url": ""
// 	}
// }
//
// api:  /Api/User/incrUserRecord
// {"incrDailyRecord":689,"incrTotalKeystrokes":1823,"incrTotalTime":346.04,"incrTotalWordNum":617,"from":"web","timestamp":1784375418,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
//
// api:  /Api/Rank/uploadResult
// {"challengeFlag":0,"textTitle":"锦标赛第3255期","speed":106.98,"keystrokes":5.27,"maChang":2.95,"wordNum":617,"typingTime":"05:46.039","huiGai":72,"huiChe":0,"jianShu":1823,"jianZhun":"80.34%","repeatNum":0,"daCi":"71.15%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":1223,"keyMethod":"+100.00%","isFirstSubmit":1,"isGroupText":0,"accuracy":80.34,"from":"web","timestamp":1784375418,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
//
// api:  /Api/Record/uploadRecord
// {"content":"作为一个社恐患者","textTitle":"锦标赛第3255期","speed":106.98,"keystrokes":5.27,"maChang":2.95,"wordNum":617,"typingTime":"05:46.039","huiGai":72,"huiChe":0,"jianShu":1823,"jianZhun":"80.34%","repeatNum":0,"daCi":"71.15%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":1223,"keyMethod":"+100.00%","isSystemText":1,"from":"web","timestamp":1784375418,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
