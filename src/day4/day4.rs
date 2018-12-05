use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;
extern crate chrono;
use chrono::{NaiveDateTime, NaiveDate, NaiveTime, Duration, Timelike};



#[derive(Debug)]
enum LogMessageKind {
    ShiftChange(u32),
    Sleep(NaiveTime),
    Wake(NaiveTime),
}

#[derive(Debug)]
struct LogMessage {
    kind: LogMessageKind,
    timestamp: NaiveDateTime
}

#[derive(Debug)]
struct TimestampedMessage {
    timestamp: NaiveDateTime,
    message: String
}

fn parse_log_messages(messages: &Vec<String>) -> Vec<LogMessage> {
    let mut log_messages: Vec<LogMessage> = Vec::new();

    for message in messages{
        let timestamp_ = &message[1..17];
        let timestamp_date = match NaiveDateTime::parse_from_str(&timestamp_, "%Y-%m-%d %H:%M"){
            Err(why) => panic!("couldn't parse timestamp_date {}  ", why.description()),
            Ok(timestamp_date) => timestamp_date,
        };

        let timestamp_time = match NaiveTime::parse_from_str(&timestamp_, "%Y-%m-%d %H:%M"){
            Err(why) => panic!("couldn't parse timestamp {}  ", why.description()),
            Ok(timestamp_time) => timestamp_time,
        };

        let message = &message[19..];
        if message.starts_with("Guard") {
            let message = &message[7..];
            let space_offset = message.find(' ').unwrap_or(0);
            let guard_id: u32 = message[..space_offset]
                .parse().expect("Expected u32");
            log_messages.push(
                LogMessage{kind: LogMessageKind::ShiftChange(guard_id),
                    timestamp: timestamp_date});
        } else if message.starts_with("falls") {
            log_messages.push(LogMessage{kind: LogMessageKind::Sleep(timestamp_time), timestamp:timestamp_date});
        } else if message.starts_with("wakes"){
            log_messages.push(LogMessage{kind: LogMessageKind::Wake(timestamp_time), timestamp:timestamp_date});
            // log_messages.push(LogMessage::Wake(timestamp));
        }
    }
    log_messages.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
    return log_messages;
}



fn main() {
    // let ss : NaiveTime = NaiveTime::parse_from_str("00:32", "%H:%M").expect("bla");
    // let END_TIME : NaiveTime = NaiveTime::parse_from_str("01:00", "%H:%M").expect("bla");
    // let dt: NaiveDateTime = NaiveDate::from_ymd(2015, 9, 8).and_hms_milli(12, 34, 56, 789);

    // let t = NaiveTime::from_hms(23, 56, 4);
    // println!("{:?}", ss.minute());
    // ss.num_seconds_from_midnight();

    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Expected filename");
        std::process::exit(1);
    }

    let path = Path::new(&args[1]);
    let display = path.display();
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", display,
                                                   why.description()),
        Ok(file) => file,
    };

    let mut lines: Vec<String> = Vec::new();
    for line in BufReader::new(file).lines() {
        lines.push(line.unwrap());
    }

    let log_messages = parse_log_messages(&lines);
    let mut guard_sleep_time: HashMap<u32, i64>
        = HashMap::new();

    let mut temp: HashMap<u32, [u32;60]>
        = HashMap::new();


    let mut guard_id: u32 = 0;
    let mut last_sleep_time: u32 = 0;

    let zero_arr: [u32;60] = [0; 60];
    for message in &log_messages{
        if let LogMessageKind::ShiftChange(id) = message.kind{
            guard_id = id;
        } else if let LogMessageKind::Sleep(ts) = message.kind {
            last_sleep_time = ts.minute();
        }
        else if let LogMessageKind::Wake(ts) = message.kind {
            let timestamp = message.timestamp;
            let wake_time = ts.minute();

            let counts = temp.entry(guard_id)
                .or_insert(zero_arr);

            for i in last_sleep_time..wake_time{
                let xx: usize = i as usize;
                counts[xx] += 1;
            }
        }

    }
    let mut guard_id_max: u32 = 0;
    let mut guard_sleep_max: u32 = 0;
    let mut guard_id_freq_max : u32 = 0;
    let mut guard_id_freq_max_value : u32 = 0;

    for (guard_id, counts) in &temp {
        let sum = counts.iter().fold(0,|a, &b| a + b);
        if sum > guard_sleep_max{
            guard_sleep_max = sum;
            guard_id_max = *guard_id;
        }

        if let Some(freq) = counts.iter().max(){
            if *freq > guard_id_freq_max_value{
                guard_id_freq_max_value = *freq;
                guard_id_freq_max = *guard_id;
            }
        }


    }

    let max_counts = temp.get(&guard_id_max).expect("bla");
    let mut tmp_val = 0;
    let mut tmp_idx: u32 = 0;
    for (i, c) in max_counts.iter().enumerate() {
        if *c > tmp_val{
            tmp_val = *c;
            tmp_idx = i as u32;
        }
    }

    println!("idx {}, id {}", tmp_idx, guard_id_max);
    println!("result {}", tmp_idx*guard_id_max);

    let max_freqs = temp.get(&guard_id_freq_max).expect("bla");
    let mut tmp_val = 0;
    let mut tmp_idx: u32 = 0;
    for (i, c) in max_freqs.iter().enumerate() {
        if *c > tmp_val{
            tmp_val = *c;
            tmp_idx = i as u32;
        }
    }

    println!("idx {}, id {}", tmp_idx, guard_id_freq_max);
    println!("result {}", tmp_idx*guard_id_freq_max);

}


