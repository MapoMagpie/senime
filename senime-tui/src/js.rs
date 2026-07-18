use serde::{Serialize, Serializer};

use crate::measurement::Measurement;

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
    title: String,
    content: String,
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
// 		"content": "那天班上学习《人民日报》社论《领导干部带头学好》的文章，班主任主持，班长顾养民念报纸。孙少平一句也没听，低着头悄悄在桌子下面看小说。他根本没有发现跛女子给班主任老师示意他的不规行为。直等到老师走到他面前，把书从他手里一把夺过之后，他才猛地惊呆了。全班顿时哄堂大笑。顾养民不念报了，他看来似乎是一副局外人的样子，但孙少平觉得班长分明抱着一种幸灾乐祸的态度，看老师怎样处置他呀。班主任把没收的书放在讲桌上，先没说什么，让顾养民接着往下念。学习完了以后，老师把他叫到宿舍，意外地把书又还给了他，并且说：“《红岩》是一本好书，但以后你不要在课堂上看了。去吧”孙少平怀着感激的心情退出了老师的房子。他从老师的眼睛里没有看出一丝的谴责，反而满含着一种亲切和热情。这一件小小的事，使他对书更加珍爱了。是的，他除过一天几个黑高粱面馍以外，再有什么呢？只有这些书，才使他觉得活着还是十分有意义的，他的精神也才能得到一些安慰，并且唤起对自己未来生活的某种美好的向往----没有这一点，他就无法熬过眼前这艰难而痛苦的每一个日子。"
// 	}
// }
pub fn js_bridge(settings_path: &str, action: JSAction) -> Option<(JSSettings, JSContent)> {
    // 1. 从`setting_path`读取到`JSSettings`
    // 2. 根据`action`请求不同的`api`，
    // 3. 以`/Api/Text/getContent`为例，构建请求体，其中`from, version, subversions, token`从`JSSettings`中来，`timestamp`就地获取
    //    请求体 {"competitionType":0,"snumflag":"1","from":"web","timestamp":1784339666,"version":"v2.1.6","subversions":17108,"token":"7d670b541f0b8"}
    // 4. 将请求体加密
    //    加密后: 0hv2w3UU00zcNMoK7Ic7oMTP9yGUa1M0Ng7JcNzRli0vJv9BOa8WoM7qMYZhXVs1QsP+zpK/qO5zsQWUulXhrE5WhEugG5b6Sx3XbOoJHKU21BZIge0kE72+lOEqmTWA+tFWxEzpFH4aZVm2D66yQlhhKQn8PEgCgJ/HIgu9TvWErXUdEbDc40pXqRVcBKql
    // 5. 请求`api`，以下是在`javascript`中进行请求的示例，转成`rust`版本的
    //    await fetch("https://www.jsxiaoshi.com/index.php/Api/Text/getContent", {
    //        "credentials": "omit",
    //        "headers": {
    //            "User-Agent": "Mozilla/5.0 (X11; Linux x86_64; rv:152.0) Gecko/20100101 Firefox/152.0",
    //            "Accept": "application/json, text/plain, */*",
    //            "Accept-Language": "en-US,en;q=0.9,zh-CN;q=0.8",
    //            "Content-Type": "application/x-www-form-urlencoded",
    //            "Sec-Fetch-Dest": "empty",
    //            "Sec-Fetch-Mode": "cors",
    //            "Sec-Fetch-Site": "cross-site",
    //            "Priority": "u=0",
    //            "Pragma": "no-cache",
    //            "Cache-Control": "no-cache"
    //        },
    //        "referrer": "https://www.52dazi.cn/",
    //        "body": "0hv2w3UU00zcNMoK7Ic7oMTP9yGUa1M0Ng7JcNzRli0vJv9BOa8WoM7qMYZhXVs1QsP+zpK/qO5zsQWUulXhrE5WhEugG5b6Sx3XbOoJHKU21BZIge0kE72+lOEqmTWA+tFWxEzpFH4aZVm2D66yQlhhKQn8PEgCgJ/HIgu9TvWErXUdEbDc40pXqRVcBKql",
    //        "method": "POST",
    //        "mode": "cors"
    //    });
    // 6. 得到请求结果，将其中的`a_name`作为`JSContent`的`title`，`a_content`作为`JSContent`的`content`
    // {
    // 	"error": 0,
    // 	"msg": {
    // 		"a_name": "消费主义陷阱：理性生活，回归本真",
    // 		"a_content": "当下社会...",
    // 		"a_url": ""
    // 	}
    // }
    // 7. 返回`JSSettings`和`JSContent`
    todo!()
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
    todo!()
}
