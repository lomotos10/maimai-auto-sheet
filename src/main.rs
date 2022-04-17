use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::fs;
use std::io::{self, BufRead, Write};
use thiserror::Error;

lazy_static! {
    static ref SONG_REPLACEMENT: HashMap<String, String> = {
        [
            ("GIGANTØMAKHIA", "GIGANTOMAKHIA"),
            // ("D✪N’T  ST✪P  R✪CKIN’", "D✪N’T ST✪P R✪CKIN’"),
        ]
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect::<HashMap<_, _>>()
    };
}

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("{0}")]
    IO(#[from] std::io::Error),
    #[error("{0}")]
    SerdeError(#[from] serde_json::Error),
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
enum ChartType {
    Dx,
    Std,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
struct Level {
    bas: String,
    adv: String,
    exp: String,
    mas: String,
    rem: Option<String>,
}

#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord, Clone)]
enum Diff {
    Bas,
    Adv,
    Exp,
    Mas,
    Rem,
}

#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord, Clone)]
struct Chart {
    diff: Diff,
    song: Song,
}

#[derive(Eq, Hash, PartialEq, Debug, PartialOrd, Ord, Clone)]
struct ChartWithPrevVerLevel {
    prev_level: String,
    chart: Chart,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone, PartialOrd, Ord)]
struct Song {
    ordering: usize,
    jacket: String,
    title: String,
    chart_type: ChartType,
    level: Level,
}

fn serdest_to_string(st: &serde_json::Value) -> String {
    if let serde_json::Value::String(s) = st {
        s.to_string()
    } else {
        panic!()
    }
}

fn get_curl(url: &str) -> String {
    let mut data = Vec::new();
    let mut handle = curl::easy::Easy::new();
    handle.url(url).unwrap();
    {
        let mut transfer = handle.transfer();
        transfer
            .write_function(|new_data| {
                data.extend_from_slice(new_data);
                Ok(new_data.len())
            })
            .unwrap();
        transfer.perform().unwrap();
    }
    let s = match std::str::from_utf8(&data) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };
    s.to_string()
}

fn get_song_list() -> std::result::Result<Vec<Song>, GenericError> {
    let jp_url = fs::read_to_string("data/jp_url.txt")?;
    let jp_url = jp_url.trim();
    let s = get_curl(jp_url);

    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();

    let songs = if let serde_json::Value::Array(s) = songs {
        s
    } else {
        panic!()
    };

    let mut songs_v = vec![];
    for song in songs {
        let song = if let serde_json::Value::Object(m) = song {
            m
        } else {
            panic!()
        };

        let title = serdest_to_string(song.get("title").unwrap());
        let title = SONG_REPLACEMENT.get(&title).unwrap_or(&title).to_string();
        let jacket = serdest_to_string(song.get("image_url").unwrap());
        let ordering = serdest_to_string(song.get("sort").unwrap())
            .parse::<usize>()
            .unwrap();
        if song.contains_key("lev_bas") {
            songs_v.push(Song {
                ordering,
                jacket: jacket.clone(),
                title: title.clone(),
                chart_type: ChartType::Std,
                level: Level {
                    bas: serdest_to_string(song.get("lev_bas").unwrap()),
                    adv: serdest_to_string(song.get("lev_adv").unwrap()),
                    exp: serdest_to_string(song.get("lev_exp").unwrap()),
                    mas: serdest_to_string(song.get("lev_mas").unwrap()),
                    rem: if song.contains_key("lev_remas") {
                        Some(serdest_to_string(song.get("lev_remas").unwrap()))
                    } else {
                        None
                    },
                },
            });
        }
        if song.contains_key("dx_lev_bas") {
            songs_v.push(Song {
                ordering,
                jacket,
                title,
                chart_type: ChartType::Dx,
                level: Level {
                    bas: serdest_to_string(song.get("dx_lev_bas").unwrap()),
                    adv: serdest_to_string(song.get("dx_lev_adv").unwrap()),
                    exp: serdest_to_string(song.get("dx_lev_exp").unwrap()),
                    mas: serdest_to_string(song.get("dx_lev_mas").unwrap()),
                    rem: if song.contains_key("dx_lev_remas") {
                        Some(serdest_to_string(song.get("dx_lev_remas").unwrap()))
                    } else {
                        None
                    },
                },
            });
        }
    }
    Ok(songs_v)
}

// Key type: title, dx, diff
fn set_intl_level_list() -> Result<HashMap<(String, ChartType, Diff), String>, GenericError> {
    let mut level_list = HashMap::new();
    let intl_url = fs::read_to_string("data/intl_url.txt")?;
    let intl_url = intl_url.trim();
    let s = get_curl(intl_url);
    // Parse the string of data into serde_json::Value.
    let songs: serde_json::Value = serde_json::from_str(&s).unwrap();

    let songs = if let serde_json::Value::Array(s) = songs {
        s
    } else {
        panic!()
    };
    for song in songs {
        let song = if let serde_json::Value::Object(m) = song {
            m
        } else {
            panic!()
        };
        let title = serdest_to_string(song.get("title").unwrap());
        if song.contains_key("lev_bas") {
            level_list.insert(
                (title.clone(), ChartType::Std, Diff::Bas),
                serdest_to_string(song.get("lev_bas").unwrap()),
            );
            level_list.insert(
                (title.clone(), ChartType::Std, Diff::Adv),
                serdest_to_string(song.get("lev_adv").unwrap()),
            );
            level_list.insert(
                (title.clone(), ChartType::Std, Diff::Exp),
                serdest_to_string(song.get("lev_exp").unwrap()),
            );
            level_list.insert(
                (title.clone(), ChartType::Std, Diff::Mas),
                serdest_to_string(song.get("lev_mas").unwrap()),
            );
            if song.contains_key("lev_remas") {
                level_list.insert(
                    (title.clone(), ChartType::Std, Diff::Rem),
                    serdest_to_string(song.get("lev_remas").unwrap()),
                );
            }
        }
        if song.contains_key("dx_lev_bas") {
            level_list.insert(
                (title.clone(), ChartType::Dx, Diff::Bas),
                serdest_to_string(song.get("dx_lev_bas").unwrap()),
            );
            level_list.insert(
                (title.clone(), ChartType::Dx, Diff::Adv),
                serdest_to_string(song.get("dx_lev_adv").unwrap()),
            );
            level_list.insert(
                (title.clone(), ChartType::Dx, Diff::Exp),
                serdest_to_string(song.get("dx_lev_exp").unwrap()),
            );
            level_list.insert(
                (title.clone(), ChartType::Dx, Diff::Mas),
                serdest_to_string(song.get("dx_lev_mas").unwrap()),
            );
            if song.contains_key("dx_lev_remas") {
                level_list.insert(
                    (title.clone(), ChartType::Dx, Diff::Rem),
                    serdest_to_string(song.get("dx_lev_remas").unwrap()),
                );
            }
        }
    }
    Ok(level_list)
}

fn main() -> std::result::Result<(), GenericError> {
    let songs = get_song_list()?;
    let intl_levels = set_intl_level_list()?;

    // deleted songs
    let mut intl_del_songs = HashSet::new();
    let file = File::open("data/intl_del.txt")?;
    let lines = io::BufReader::new(file).lines();
    for line in lines.flatten() {
        intl_del_songs.insert(line);
    }

    let levels = vec!["12+", "13", "13+", "14", "14+", "15"];
    let mut level_collections = levels
        .iter()
        .map(|s| (s.to_string(), HashSet::new()))
        .collect::<HashMap<_, _>>();

    for song in songs {
        let Level {
            bas,
            adv,
            exp,
            mas,
            rem,
        } = song.level.clone();

        if intl_del_songs.contains(&song.title) {
            continue;
        }

        if levels.contains(&bas.as_str()) {
            level_collections.get_mut(&bas).unwrap().insert(Chart {
                song: song.clone(),
                diff: Diff::Bas,
            });
        }
        if levels.contains(&adv.as_str()) {
            level_collections.get_mut(&adv).unwrap().insert(Chart {
                song: song.clone(),
                diff: Diff::Adv,
            });
        }
        if levels.contains(&exp.as_str()) {
            level_collections.get_mut(&exp).unwrap().insert(Chart {
                song: song.clone(),
                diff: Diff::Exp,
            });
        }
        if levels.contains(&mas.as_str()) {
            level_collections.get_mut(&mas).unwrap().insert(Chart {
                song: song.clone(),
                diff: Diff::Mas,
            });
        }
        if let Some(rem) = rem {
            if levels.contains(&rem.as_str()) {
                level_collections.get_mut(&rem).unwrap().insert(Chart {
                    song: song.clone(),
                    diff: Diff::Rem,
                });
            }
        }
    }

    for level in levels {
        let mut list = level_collections[level]
            .iter()
            .map(|chart| {
                let prev_level = intl_levels
                    .get(&(
                        chart.song.title.clone(),
                        chart.song.chart_type.clone(),
                        chart.diff.clone(),
                    ))
                    .cloned()
                    .unwrap_or_else(|| "N/A".to_string());

                ChartWithPrevVerLevel {
                    chart: chart.clone(),
                    prev_level,
                }
            })
            .collect::<Vec<_>>()
            .clone();
        list.sort();

        let mut w = File::create(format!("charts/{}.csv", level))?;
        for chart_with_prev_level in list {
            let prev_level = chart_with_prev_level.prev_level;
            let chart = chart_with_prev_level.chart;
            let chart_type = match chart.song.chart_type {
                ChartType::Dx => "DX",
                ChartType::Std => "STD",
            };
            let diff = match chart.diff {
                Diff::Bas => "BAS",
                Diff::Adv => "ADV",
                Diff::Exp => "EXP",
                Diff::Mas => "MAS",
                Diff::Rem => "REM",
            };
            writeln!(
                &mut w,
                "'{}\t{}\t{}\thttps://maimaidx.jp/maimai-mobile/img/Music/{}\t{}",
                chart.song.title, chart_type, diff, chart.song.jacket, prev_level
            )?;
        }
    }

    Ok(())
}
