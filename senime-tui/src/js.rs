use serde::{Deserialize, Serialize, Serializer};

use crate::measurement::Measurement;

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

pub enum JSAction {
    Random,
    Daily,
    None,
}

pub struct JSContent {
    pub title: String,
    pub content: String,
}

// api:  /Api/Text/getContent
// encrypt before: {"competitionType":0,"snumflag":"1","from":"web","timestamp":1784339666,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
// encrypt after: 0hv2w3UU00zcNMoK7Ic7oMTP9yGUa1M0Ng7JcNzRli0vJv9BOa8WoM7qMYZhXVs1QsP+zpK/qO5zsQWUulXhrE5WhEugG5b6Sx3XbOoJHKU21BZIge0kE72+lOEqmTWA+tFWxEzpFH4aZVm2D66yQlhhKQn8PEgCgJ/HIgu9TvWErXUdEbDc40pXqRVcBKql
// return
// {
// 	"error": 0,
// 	"msg": {
// 		"a_name": "消费主义陷阱：理性生活，回归本真",
// 		"a_content": "当下社会...",
// 		"a_url": ""
// 	}
// }
//
// api:  /Api/Text/getRandomText
// encrypt before: {"from":"web","timestamp":1784337875,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
// return
// {
// 	"error": 0,
// 	"msg": {
// 		"name": "C03",
// 		"content": "那天班上学习..."
// 	}
// }
#[allow(unused)]
pub fn js_get_content(
    settings_path: &str,
    action: JSAction,
) -> Result<(JSSettings, JSContent), String> {
    // 1. 从`settings_path`读取`JSSettings`
    let settings_str = std::fs::read_to_string(settings_path).map_err(|e| e.to_string())?;
    let settings: JSSettings = toml::from_str(&settings_str).map_err(|e| e.to_string())?;
    // 3. 构建请求体：基础字段来自 settings，timestamp 就地获取
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as usize;

    // 2. 根据`action`选择 API 端点
    let (endpoint, body) = match action {
        JSAction::Daily => {
            let body = serde_json::json!({
                "competitionType": 0,
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
        JSAction::None => return Err("no action".to_string()),
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
        .send(&encrypted)
        .map_err(|e| e.to_string())?;

    // 6. 解析响应：a_name → title, a_content → content（Random 用 name/content）
    let mut body = response.into_body();
    let body_str = body.read_to_string().map_err(|e| e.to_string())?;
    let json: serde_json::Value = serde_json::from_str(&body_str).map_err(|e| e.to_string())?;

    if json["error"] != 0 {
        return Err("api response error code 1".to_string());
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

// api:  /Api/User/incrUserRecord
// encrypt before: {"incrDailyRecord":465,"incrTotalKeystrokes":1228,"incrTotalTime":241.4,"incrTotalWordNum":417,"from":"web","timestamp":1784339915,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
//
// api:  /Api/Rank/uploadResult
// encrypt before: {"challengeFlag":0,"textTitle":"消费主义陷阱：理性生活，回归本真","speed":103.65,"keystrokes":5.09,"maChang":2.94,"wordNum":417,"typingTime":"04:01.396","huiGai":48,"huiChe":0,"jianShu":1228,"jianZhun":"79.71%","repeatNum":0,"daCi":"78.18%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":851,"keyMethod":"+100.00%","isFirstSubmit":1,"isGroupText":0,"accuracy":79.71,"from":"web","timestamp":1784339915,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
//
// api:  /Api/Record/uploadRecord
// encrypt before: {"content":"当下社会，消费主义无处不在...","textTitle":"消费主义陷阱：理性生活，回归本真","speed":103.65,"keystrokes":5.09,"maChang":2.94,"wordNum":417,"typingTime":"04:01.396","huiGai":48,"huiChe":0,"jianShu":1228,"jianZhun":"79.71%","repeatNum":0,"daCi":"78.18%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":851,"keyMethod":"+100.00%","isSystemText":1,"from":"web","timestamp":1784339915,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
//
//
// api:  /Api/User/incrUserRecord
// encrypt before: {"incrDailyRecord":53,"incrTotalKeystrokes":180,"incrTotalTime":42.55,"incrTotalWordNum":50,"from":"web","timestamp":1784341510,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
//
// api:  /Api/Rank/uploadResult
// encrypt before: {"challengeFlag":0,"textTitle":"常用前500 第 4 天","speed":70.51,"keystrokes":4.23,"maChang":3.6,"wordNum":50,"typingTime":"00:42.549","huiGai":3,"huiChe":0,"jianShu":180,"jianZhun":"85.67%","repeatNum":0,"daCi":"4%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":121,"keyMethod":"+100.00%","isFirstSubmit":1,"isGroupText":0,"accuracy":85.67,"from":"web","timestamp":1784341510,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
//
// api:  /Api/Record/uploadRecord
// encrypt before: {"content":"解听收前比观石象微知...","textTitle":"常用前500 第 4 天","speed":70.51,"keystrokes":4.23,"maChang":3.6,"wordNum":50,"typingTime":"00:42.549","huiGai":3,"huiChe":0,"jianShu":180,"jianZhun":"85.67%","repeatNum":0,"daCi":"4%","wrongNum":0,"inputMethod":"虎码","backspace":0,"xuanChong":121,"keyMethod":"+100.00%","isSystemText":1,"from":"web","timestamp":1784341510,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
#[allow(unused)]
pub fn js_report(
    settings: &JSSettings,
    action: JSAction,
    mea: &Measurement,
    content: &JSContent,
) -> () {
    let _incr_user_record = IncrUserRecord::new(settings, mea);
    let _upload_record = UploadRecord::new(settings, mea, content);
    let _upload_result = UploadResult::new(settings, mea, content);
    todo!()
}

// {
// "incrDailyRecord":53,
// "incrTotalKeystrokes":180,
// "incrTotalTime":42.55,
// "incrTotalWordNum":50,
// "from":"web",
// "timestamp":1784341510,
// "version":"v2.1.6",
// "subversions":17108,
// "token":"7d670b541f0b8"
// }
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
    backspace: usize,
    xuan_chong: usize,
}

impl JSMeasurement {
    fn new(mea: &Measurement) -> Self {
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
            backspace: mea.bs_times,
            xuan_chong: mea.se_times,
        }
    }
}

// {
// "challengeFlag":0,
// "textTitle":"常用前500 第 4 天",
// "speed":70.51,
// "keystrokes":4.23,
// "maChang":3.6,
// "wordNum":50,
// "typingTime":"00:42.549",
// "huiGai":3,
// "huiChe":0,
// "jianShu":180,
// "jianZhun":"85.67%",
// "repeatNum":0,
// "daCi":"4%",
// "wrongNum":0,
// "inputMethod":"虎码",
// "backspace":0,
// "xuanChong":121,
// "keyMethod":"+100.00%",
// "isFirstSubmit":1,
// "isGroupText":0,
// "accuracy":85.67,
// "from":"web",
// "timestamp":1784341510,
// "version":"v2.1.6",
// "subversions":17108,
// "token":"7d670b541f0b8"
// }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UploadResult {
    challenge_flag: usize,
    text_title: String,
    measure: JSMeasurement,
    input_method: String,
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
        let measure = JSMeasurement::new(mea);
        Self {
            challenge_flag: 0,
            text_title: content.title.clone(),
            measure,
            key_method: "+100.00%".to_string(),
            input_method: settings.ime.clone(),
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
// {
// "content":"解听收前比观石象微知...",
// "textTitle":"常用前500 第 4 天",
// "speed":70.51,
// "keystrokes":4.23,
// "maChang":3.6,
// "wordNum":50,
// "typingTime":"00:42.549",
// "huiGai":3,
// "huiChe":0,
// "jianShu":180,
// "jianZhun":"85.67%",
// "repeatNum":0,
// "daCi":"4%",
// "wrongNum":0,
// "inputMethod":"虎码",
// "backspace":0,
// "xuanChong":121,
// "keyMethod":"+100.00%",
// "isSystemText":1,
// "from":"web",
// "timestamp":1784341510,
// "version":"v2.1.6",
// "subversions":17108,
// "token":"7d670b541f0b8"
// }
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct UploadRecord {
    content: String,
    text_title: String,
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
        let measure = JSMeasurement::new(mea);
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

fn serialize_f32_2<S>(v: &f32, s: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    // 四舍五入到两位
    let v = (v * 100.0).round() / 100.0;
    s.serialize_f32(v)
}

// encrypt before: {"competitionType":0,"snumflag":"1","from":"web","timestamp":1784339666,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
// encrypt after: 0hv2w3UU00zcNMoK7Ic7oMTP9yGUa1M0Ng7JcNzRli0vJv9BOa8WoM7qMYZhXVs1QsP+zpK/qO5zsQWUulXhrE5WhEugG5b6Sx3XbOoJHKU21BZIge0kE72+lOEqmTWA+tFWxEzpFH4aZVm2D66yQlhhKQn8PEgCgJ/HIgu9TvWErXUdEbDc40pXqRVcBKql
// s.a 是CryptoJS
// encrypt: function(t) {
//     console.log("encrypt before:", t);
//     var e = "c9ec834c80f77237",
//         a = "db4d6bfde3057dca",
//         r = s.a.enc.Latin1.parse(e),
//         o = s.a.enc.Latin1.parse(a),
//         n = s.a.AES.encrypt(t, r, {
//             iv: o,
//             mode: s.a.mode.CBC,
//             padding: s.a.pad.ZeroPadding
//         });
//     console.log("encrypt after:", n.toString());
//     return n.toString()
// }
fn encrypt(body: String) -> String {
    use aes::cipher::{BlockEncryptMut, KeyIvInit, block_padding::ZeroPadding};

    let key = b"c9ec834c80f77237";
    let iv = b"db4d6bfde3057dca";

    // ZeroPadding 要求缓冲区预先填充到块大小的整数倍
    let body_bytes = body.as_bytes();
    let padded_len = (body_bytes.len() + 15) / 16 * 16;
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
