use html_parser::{Dom, Element, Node};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, Write};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenericError {
    #[error("{0}")]
    ParserError(#[from] html_parser::Error),
    #[error("{0}")]
    IO(#[from] std::io::Error),
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum ChartType {
    Dx,
    Std,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Level {
    bas: String,
    adv: String,
    exp: String,
    mas: String,
    rem: Option<String>,
}

#[derive(Eq, Hash, PartialEq, Debug)]
enum Diff {
    Bas,
    Adv,
    Exp,
    Mas,
    Rem,
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Chart {
    song: Song,
    diff: Diff,
}

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
struct Song {
    jacket: String,
    title: String,
    chart_type: ChartType,
    level: Level,
}

fn only_child_of_dom(elem: &Dom) -> &Element {
    if let Node::Element(e) = &elem.children[0] {
        e
    } else {
        panic!();
    }
}

fn only_child(elem: &Element) -> &Element {
    assert_eq!(elem.children.len(), 1);
    if let Node::Element(e) = &elem.children[0] {
        e
    } else {
        panic!();
    }
}

fn nth_child(elem: &Element, index: usize) -> &Element {
    if let Node::Element(e) = &elem.children[index] {
        e
    } else {
        panic!();
    }
}

fn children_to_text(elem: &Element) -> String {
    assert_eq!(elem.children.len(), 1);
    if let Node::Text(s) = &elem.children[0] {
        s.to_string()
    } else {
        panic!();
    }
}

fn get_song_list() -> std::result::Result<Vec<Song>, GenericError> {
    let genres = vec!["pop", "nico", "touhou", "gv", "mai", "gc"];
    let mut songs = vec![];

    for genre in genres {
        // get html file
        // `<div class="data"><div><div><div class="songs">` ...
        let html = std::fs::read_to_string(format!("data/{}.html", genre))?;

        // parse html file - current code - 2022.03.24 (uni+ initial update)
        let json = Dom::parse(&html)?;
        let elem = only_child_of_dom(&json);
        let elem = only_child(elem);
        let elem = only_child(elem);
        let elem = only_child(elem);
        for song in &elem.children {
            // println!("{:#?}", song_elem.children.len());
            // println!("{:#?}", song_elem);

            let song_elem = if let Node::Element(e) = song {
                e
            } else {
                panic!();
            };
            let song_elem = only_child(song_elem);
            let song_elem = only_child(song_elem);
            let song_elem = only_child(song_elem);
            let song_elem = nth_child(song_elem, 1);

            // jacket
            let jacket_elem = nth_child(song_elem, 0);
            assert_eq!(jacket_elem.classes[1], "jacket");
            let jacket_elem = only_child(jacket_elem);
            assert_eq!(jacket_elem.name, "img");
            let jacket = jacket_elem.attributes["src"].as_ref().unwrap();

            // title; artist is also here if you want. I don't want it yet.
            let title_elem = nth_child(song_elem, 1);
            let title_elem = only_child(title_elem);
            assert_eq!(title_elem.classes[1], "title");
            let song_title_elem = nth_child(title_elem, 0);
            // let artist_title_elem = nth_child(title_elem, 2);
            assert_eq!(song_title_elem.classes[0], "titleText");
            let title = children_to_text(song_title_elem);

            // levels & DX/STD
            for index in &[2, 3] {
                let level_elem = nth_child(song_elem, *index as usize);
                if level_elem.classes[0] != "songs-data-box-level" {
                    continue;
                }
                let bas = children_to_text(nth_child(level_elem, 0));
                let adv = children_to_text(nth_child(level_elem, 1));
                let exp = children_to_text(nth_child(level_elem, 2));
                let mas = children_to_text(nth_child(level_elem, 3));
                let rem_elem = nth_child(level_elem, 4);
                let rem = match rem_elem.children.len() {
                    0 => None,
                    1 => Some(children_to_text(rem_elem)),
                    _ => panic!(),
                };
                let level = Level {
                    bas,
                    adv,
                    exp,
                    mas,
                    rem,
                };
                let chart_type = match level_elem.classes[1].as_str() {
                    "dx" => ChartType::Dx,
                    "std" => ChartType::Std,
                    _ => panic!("{:#?}", &level_elem.classes[1]),
                };

                songs.push(Song {
                    jacket: jacket.clone(),
                    title: title.clone(),
                    level,
                    chart_type,
                });
            }

            // println!("{:#?}", song_elem.children.len());
            // println!("{:#?}", song_elem);
        }
    }

    // println!("{:#?}", songs);
    Ok(songs)
}

use html2text::from_read;

fn main() -> std::result::Result<(), GenericError> {
    let songs = get_song_list()?;

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
    // println!("{:#?}", intl_del_songs);
    // println!("{:#?}", level_collections);

    for level in levels {
        let list = &level_collections[level];
        let mut w = File::create(format!("charts/{}.csv", level))?;
        for chart in list {
            let max_len = 200;
            assert!(chart.song.title.len() < max_len);
            let title = from_read(chart.song.title.as_bytes(), max_len)
                .trim()
                .to_string();

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
                "{}\t{}\t{}\t{}",
                title, chart_type, diff, chart.song.jacket
            )?;
        }
    }

    Ok(())
}
