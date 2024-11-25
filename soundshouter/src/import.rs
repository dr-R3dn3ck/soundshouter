use std::path::{PathBuf, Component};
use std::io;
use std::fs::{self, DirEntry, File};
use std::io::BufReader;
use std::path::Path;
use log::{error, info};
use rodio::Source;
use crate::db::{establish_connection, get_or_create_category, get_or_create_sound, get_or_create_subcategory};
use crate::db::models;

pub fn import_sounds(src: &PathBuf, dest: &PathBuf, db_uri: &String) {

    let res = visit_dirs(src, &|entry| {
        // use decoder to confirm file format
        let file = File::open(entry.path()).unwrap();
        let res = rodio::Decoder::new(BufReader::new(file));
        let mut db = establish_connection(Some(&db_uri));

        // create database entry
        match res {
            Ok(decoder) => {
                let soundfile = entry.path();
                let components: Vec<Component> = soundfile.strip_prefix(&src).unwrap().components().collect();
                let sound_name = soundfile.file_stem().unwrap().to_str().unwrap();

                let cat: Option<models::Category> = if components.len() == 2 || components.len() == 3 {
                    let res = get_or_create_category(&mut db,
                                                     components[0].as_os_str().to_str().unwrap());
                    if let Ok(category) = res { Some(category) }
                    else { None }
                } else { None };

                let catid = cat.as_ref().unwrap().id;
                let subcat: Option<models::SubCategory> = if components.len() == 3 {
                    let res = get_or_create_subcategory(&mut db,
                                                        components[1].as_os_str().to_str().unwrap(),
                                                        catid);
                    if let Ok(subcategory) = res { Some(subcategory) }
                    else { None }
                } else { None };

                let _sound: Option<models::Sound> = {
                    let catid = if let Some(_cat) = &cat { Some(_cat.id) } else { None };
                    let subcatid = if let Some(_subcat) = &subcat { Some(_subcat.id) } else { None };

                    // FIXME: this often fails to provide a correct number
                    let duration = if let Some(duration) = decoder.total_duration() {
                        duration.as_secs_f32()
                    }
                    else { 0.0 };
                    println!("{:?}", duration);
                    // TODO: copy sound and store destination path in database
                    let res = get_or_create_sound(&mut db,
                                                  sound_name, soundfile.to_str().unwrap(),
                                                  duration, catid, subcatid);
                    if let Ok(sound) = res { Some(sound) }
                    else { None }
                };
                // TODO: better error reporting and logging
            },
            Err(e) => { error!("error decoding file: {}", e); }
        }

    });

    match res {
        // TODO: logging
        Ok(_) => { println!("successfully imported sounds"); },
        Err(e) => { println!("error importing sounds: {}", e); }
    }
}

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}