use std::{
    fmt::Display,
    time::{Duration, Instant},
};

#[derive(Debug)]
pub struct Record {
    pub len: i32,
    pub origin: Vec<char>,
    pub has_selection: bool,
    pub start: Instant,
    pub end: Instant,
}

pub struct Measurement {
    // 总时长
    pub duration: Duration,
    // 暂停时间
    pub pause_duration: Duration,
    // 字符数
    pub text_wc: usize,
    // 原始输入字符数
    pub code_cc: usize,
    // 预设文本数量
    pub preset_wc: Option<usize>,
    pub preset_avg_len: Option<f32>,
    // 每秒键数，根据code_cc计算
    pub kps: f32,
    // 每分字数，根据text_wc计算
    pub wpm: f32,
    // 平均码长
    pub avg_len: f32,
    // context中的records始终为追加，也不会修改已有的record，因此记录已经计算过的records，下次计算时从此处开始
    // 一个例外是，records变为空的话，说明context清空了所有输入的数据，因此Measurement也重置
    pub counted: usize,
    // 回退次数
    pub bs_times: usize,
    // 空格次数
    pub sp_times: usize,
    // 候选次数
    pub se_times: usize,
    // 键准，根据`code_cc`与`bs_times`计算
    pub accuracy: f32,
    // 打单次数
    pub si_times: usize,
    // 打词率，根据`text_wc`与`si_times`计算
    pub wg_freq: f32,
    pub records: Vec<Record>,
}

impl Measurement {
    pub fn new() -> Self {
        Measurement {
            duration: Duration::from_secs(0),
            pause_duration: Duration::from_secs(0),
            text_wc: 0,
            code_cc: 0,
            preset_wc: None,
            preset_avg_len: None,
            kps: 0.0,
            wpm: 0.0,
            avg_len: 0.0,
            counted: 0,
            bs_times: 0,
            sp_times: 0,
            se_times: 0,
            accuracy: 100.0,
            si_times: 0,
            wg_freq: 0.0,
            records: Default::default(),
        }
    }

    pub fn push_record(
        &mut self,
        text_len: i32,
        origin: Vec<char>,
        input_start: Instant,
        has_selection: bool,
    ) {
        let record = Record {
            len: text_len,
            origin,
            has_selection,
            start: input_start,
            end: Instant::now(),
        };
        self.records.push(record);
    }

    pub fn clear(&mut self) {
        let preset_wc = self.preset_wc.take();
        let preset_avg_len = self.preset_avg_len.take();
        let mut new_measurement = Measurement::new();
        new_measurement.preset_wc = preset_wc;
        new_measurement.preset_avg_len = preset_avg_len;
        *self = new_measurement;
    }

    /// 计量速度.
    /// 需要的信息:
    ///   开始时间-结束时间
    ///   总字数
    ///   总输入
    ///   码长
    ///   顶字次数?
    ///   空格次数?
    ///   回退次数?
    ///   候选次数?
    ///   暂停时间?
    pub fn calc(&mut self, text_wc: usize) {
        if self.records.is_empty() {
            return;
        }
        if self.counted == self.records.len() {
            return;
        }
        // 暂停判断，5秒
        let pause_assert = Duration::from_secs(5);

        let start = self.records[0].start;
        let mut end = if self.counted == 0 {
            self.records[0].end
        } else {
            self.records[self.counted - 1].end
        };
        for rec in &self.records[self.counted..] {
            if rec.len < 0 {
                self.bs_times += 1;
            }
            self.code_cc += rec.origin.len();
            if end < rec.start {
                let dur = rec.start - end;
                if dur > pause_assert {
                    self.pause_duration += dur;
                }
            }
            if rec.origin.last() == Some(&' ') {
                self.sp_times += 1;
            }
            if rec.has_selection {
                self.se_times += 1;
            }
            if rec.len == 1 {
                self.si_times += 1;
            }
            end = rec.end;
        }
        self.counted = self.records.len();
        self.text_wc = text_wc;
        self.accuracy = if self.code_cc == 0 {
            100.0
        } else {
            (self.code_cc - self.bs_times) as f32 / self.code_cc as f32 * 100.0
        };

        self.wg_freq = if self.text_wc == 0 {
            0.0
        } else {
            (self.text_wc - self.si_times) as f32 / self.text_wc as f32 * 100.0
        };

        self.duration = end.duration_since(start) - self.pause_duration;
        self.wpm = self.text_wc as f32 / (self.duration.as_secs_f32() / 60.0);
        self.kps = self.code_cc as f32 / self.duration.as_secs_f32();
        self.avg_len = self.code_cc as f32 / text_wc as f32;
    }

    pub fn spans(&self) -> Vec<String> {
        let (dur_min, dur_sec) = duration_to_min_and_sec(self.duration);
        let (pause_min, pause_sec) = duration_to_min_and_sec(self.pause_duration);
        let pal = self
            .preset_avg_len
            .map_or("".to_string(), |pal| format!("/{pal:.2}"));
        let pwc = self.preset_wc.map_or("".to_string(), |pw| format!("/{pw}"));

        let span_wpm = format!("速度:[{:.2}]", self.wpm);
        let span_kps = format!("击键:[{:.2}]", self.kps);
        let span_avg_len = format!("码长:[{:.2}{}]", self.avg_len, pal);
        let span_dur = format!("耗时:[{dur_min:02}:{dur_sec:02}]");
        let span_pause = format!("暂停:[{pause_min:02}:{pause_sec:02}]");
        let span_text_wc = format!("字数:[{}{}]", self.text_wc, pwc);
        let span_code_cc = format!("键数:[{}]", self.code_cc);
        let span_bs_times = format!("回退:[{}]", self.bs_times);
        let span_sp_times = format!("空格:[{}]", self.sp_times);
        let span_se_times = format!("候选:[{}]", self.se_times);
        let span_accuracy = format!("键准:[{:.2}]", self.accuracy);
        let span_wg_freq = format!("打词:[{:.2}]", self.wg_freq);
        vec![
            span_wpm,
            span_kps,
            span_avg_len,
            span_dur,
            span_pause,
            span_text_wc,
            span_code_cc,
            span_bs_times,
            span_sp_times,
            span_se_times,
            span_accuracy,
            span_wg_freq,
        ]
    }
}

impl Display for Measurement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.spans().join(" "))
    }
}

fn duration_to_min_and_sec(duration: Duration) -> (u64, u64) {
    let time = duration.as_secs();
    let minutes = time / 60;
    let seconds = time % 60;
    (minutes, seconds)
}
