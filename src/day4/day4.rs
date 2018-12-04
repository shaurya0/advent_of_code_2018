use std::env;
use std::io::{BufReader,BufRead};
use std::fs::File;
use std::path::Path;
use std::error::Error;
use std::collections::HashMap;
extern crate chrono;
use chrono::{NaiveDateTime, NaiveTime, Duration};


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
    let START_TIME : NaiveTime = NaiveTime::parse_from_str("00:00", "%H:%M").expect("bla");
    let END_TIME : NaiveTime = NaiveTime::parse_from_str("00:59", "%H:%M").expect("bla");

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

    let mut temp: HashMap<u32, [i64;59]>
        = HashMap::new();


    let mut guard_id: u32 = 0;
    let mut last_sleep_time: NaiveDateTime = log_messages[0].timestamp.clone();
    for message in &log_messages{
        if let LogMessageKind::ShiftChange(id) = message.kind{
            guard_id = id;
        } else if let LogMessageKind::Sleep(ts) = message.kind {
            last_sleep_time = message.timestamp;
        }
        else if let LogMessageKind::Wake(ts) = message.kind {
            let timestamp = message.timestamp;
            let dur = timestamp.signed_duration_since(last_sleep_time)
                .num_minutes();

            let count = guard_sleep_time.entry(guard_id)
            .or_insert(0);
            *count += dur;
        }
        // match message.kind  {
        //     LogMessageKind::ShiftChange(id) => println!("guard {} start {:?}", id, message.timestamp),
        //     LogMessageKind::Sleep => println!("sleep {:?}", message.timestamp),
        //     LogMessageKind::Wake => println!("wake {:?}", message.timestamp),
        // }
    }
    // let mut guard_id_max: u32;
    // let mut guard_sleep_max:
    // for (guard_id, count) in &guard_sleep_time {
    //     println!("guard {}, slept for {}", guard_id, count);
    // }


    let sample = String::from("[1518-11-01 00:32] Guard #100 begins shift");
    // let sss = &sample[19..];
    // let sss = &sss[7..];
    // let space_offset = sss.find(' ').unwrap_or(0);
    // let sss = &sss[..space_offset];
    // println!("{:?}", sss);

    let time_slice = &sample[1..17];
    // println!("{:?}", time_slice);
    let time = match NaiveTime::parse_from_str(&time_slice, "%Y-%m-%d %H:%M"){
        Err(why) => panic!("couldn't parse time {}  ", why.description()),
        Ok(time) => println!("{:?}",time)
    };

    // let sample = String::from("[1518-11-03 00:02] Guard #10 begins shift");
    // let time_slice = &sample[1..17];
    // let time2 = match NaiveDateTime::parse_from_str(&time_slice, "%Y-%m-%d %H:%M"){
    //     Err(why) => panic!("couldn't parse time {}  ", why.description()),
    //     Ok(time2) => time2,
    // };

    // let dur = time.signed_duration_since(time2);

    // println!("{:?}", dur.num_minutes());
}


