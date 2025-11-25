use crate::config::Log;
use flexi_logger::{Age, Cleanup, Criterion, Duplicate, FileSpec, Logger, Naming, WriteMode};

fn jieto_detailed_format(
    w: &mut dyn std::io::Write,
    now: &mut flexi_logger::DeferredNow,
    record: &log::Record,
) -> Result<(), std::io::Error> {
    write!(
        w,
        "[{}] {} [{}] {}",
        now.format("%Y-%m-%d %H:%M:%S"), // ← 必须有这一行！
        record.level(),
        record.target(),
        &record.args()
    )
}

fn parse_age(age_str: &str) -> Option<Age> {
    match age_str.to_lowercase().as_str() {
        "second" => Some(Age::Second),
        "minute" => Some(Age::Minute),
        "hour" => Some(Age::Hour),
        "day" => Some(Age::Day),
        _ => None,
    }
}

pub fn init_logger(config: &Log, app_name: &str) -> anyhow::Result<()> {
    let mut filespec = FileSpec::default().suffix("log");

    // 设置目录
    if let Some(dir) = &config.directory {
        filespec = filespec.directory(dir);
    } else {
        filespec = filespec.directory("logs");
    }

    // 设置 basename（即文件名前缀）
    let basename = config.filename_prefix.as_deref().unwrap_or(app_name);
    filespec = filespec.basename(basename);

    // 构建滚动策略
    let criterion = match (&config.age, config.max_size_mb) {
        (Some(age_str), Some(size_mb)) => {
            if let Some(age) = parse_age(age_str) {
                Criterion::AgeOrSize(age, size_mb * 1024 * 1024)
            } else {
                eprintln!("无效的 age 值: '{}', 默认使用 Size 滚动", age_str);
                Criterion::Size(size_mb * 1024 * 1024)
            }
        }
        (Some(age_str), None) => {
            if let Some(age) = parse_age(age_str) {
                Criterion::Age(age)
            } else {
                eprintln!("无效的 age 值: '{}', 默认不滚动", age_str);

                Criterion::Size(100 * 1024 * 1024) // 100 MB
            }
        }
        (None, Some(size_mb)) => Criterion::Size(size_mb * 1024 * 1024),
        (None, None) => {
            Criterion::Size(10 * 1024 * 1024 * 1024) // 10 GB
        }
    };

    let default_level = String::from("info");
    let level = config.level.as_ref().unwrap_or(&default_level);
    let _ = Logger::try_with_str(level)?
        .log_to_file(filespec)
        .rotate(
            criterion,
            Naming::TimestampsDirect,
            Cleanup::KeepLogFiles(config.keep_files),
        )
        .write_mode(WriteMode::BufferAndFlush)
        .duplicate_to_stderr(Duplicate::All)
        .format_for_files(jieto_detailed_format)
        .start()?;

    Ok(())
}
