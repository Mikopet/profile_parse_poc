#![allow(dead_code)]
use convert_case::{Case, Casing};
use glob::{glob_with, MatchOptions};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use serde_repr::Deserialize_repr;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct Person {
    name: String,
}

#[derive(Debug, Deserialize)]
struct Tag {
    name: String,
    color: Color,
}

type Color = (u8, u8, u8);

#[derive(Debug, Deserialize_repr)]
#[repr(u8)]
enum AuthMethod {
    NoAuth = 0,
    Password = 1,
}

#[derive(Debug, Deserialize)]
struct ProfileAttribute {
    name: String,
    tag_ids: Vec<Uuid>,
    auth_method: AuthMethod,
}

struct Profile {
    id: Uuid,
    version: u32,
    person_id: Uuid,
    created_at: u32,
    save_interval: u8,
    save_keep_threshold: u8,
    data: Option<String>,
}

impl Profile {
    fn new(path: &Path) -> String {
        path.display().to_string()
    }
}

fn main() -> Result<(), ProfileError> {
    println!(
        "{:#?}",
        (
            read_meta::<Person>(),
            read_meta::<Tag>(),
            read_meta::<ProfileAttribute>(),
        )
    );

    // let profiles = Profile {}

    // let profiles = for entry in glob_with("profiles/*-*-*-*-*/*-*-*-*-*-*", MatchOptions::new())
    //     .expect("Failed to read any profiles")
    // {
    //     match entry {
    //         Ok(path) => Profile::new(&path),
    //         Err(e) => {}
    //     }
    // };

    // for entry in glob_with("profiles/*-*/*", MatchOptions::new()).unwrap() {
    //     if let Ok(path) = entry {
    //         println!("{:?}", path.display())
    //     }
    // }

    // let glob = glob::glob("profiles/*-*/*-*").unwrap();
    // println!("{:#?}", glob);
    // println!("{:#?}", glob[0]);

    Ok(())
}

// impl<T> From<T> for ProfileError<T> {
//     fn from(error: T) -> Self {
//         match error {
//             ProfileError::Io<T> { v } => ProfileError::Io {
//                 path: PathBuf::from(""),
//                 // error: error.into_error(),
//                 error,
//             },
//         }
//     }
// }

#[derive(Debug)]
enum ProfileError {
    Io {
        path: PathBuf,
        error: std::io::Error,
    },
    Parse {
        path: PathBuf,
        error: toml::de::Error,
    },
    // Glob {
    //     path: PathBuf,
    //     error: T,
    // },
}

fn read_meta<T: DeserializeOwned>() -> Result<HashMap<Uuid, T>, ProfileError> {
    let path = get_path::<T>();
    let path = Path::new(&path);
    let raw = read_to_string(&path)?;
    let data: HashMap<Uuid, T> = parse_toml(&raw, &path)?;

    Ok(data)
}

fn get_path<T>() -> String {
    format!(
        "profiles/_meta/{}.toml",
        std::any::type_name::<T>()
            .to_case(Case::Snake)
            .strip_prefix("toml_parse::")
            .unwrap()
    )
}

fn parse_toml<D: DeserializeOwned>(raw: &str, path: &Path) -> Result<D, ProfileError> {
    ::toml::from_str(raw).map_err(|error| ProfileError::Parse {
        path: path.into(),
        error,
    })
}

fn read_to_string(path: &Path) -> Result<String, ProfileError> {
    std::fs::read_to_string(path).map_err(|error| ProfileError::Io {
        path: path.into(),
        error,
    })
}
// fn glob_profiles_dir() -> Result<std::path::Paths, ProfileError> {
//     let path = "profiles/*-*-*-*-*/*-*-*-*-*-*";
//     glob::glob_with(path, MatchOptions::new())
//         .map_err(|error| ProfileError::Io { path: path, error })
// }
