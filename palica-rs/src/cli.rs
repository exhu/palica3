use anyhow::Context;

/*
    palica media catalogue program
    Copyright (C) 2023 Yury Benesh

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/
use crate::coll_builder;
use crate::dblayer::read;
use crate::dblayer::write;
use crate::dblayer::DbId;
use std::path::Path;

enum YesNo {
    Yes,
    No,
}

fn ask_confirmation(msg: &str) -> Result<YesNo, std::io::Error> {
    loop {
        println!("{msg}(y/n)");
        let mut answer = String::new();
        std::io::stdin().read_line(&mut answer)?;
        match answer.as_str() {
            "y" | "Y" => return Ok(YesNo::Yes),
            "n" | "N" => return Ok(YesNo::No),
            _ => (),
        }
    }
}

pub struct CollectionAdd {
    pub db_file_name: String,
    pub verbose: bool,
    pub yes: bool,
    pub name: String,
    pub path: String,
    pub filter_id: DbId,
}

fn check_with_existing_paths(rdb: &read::Db, fs_path: &str) -> anyhow::Result<()> {
    let cols = rdb
        .collections_by_fs_path(fs_path)
        .with_context(|| "check_with_existing_paths: Failed to read db.")?;
    if cols.is_empty() == false {
        println!("WARNING: there are existing collections with the same path '{fs_path}':");
        for c in cols {
            eprintln!("{}, {}", c.id, c.coll_name);
        }
        match ask_confirmation("Still continue?") {
            Err(e) => {
                return Err(e.into());
            }
            Ok(YesNo::No) => {
                return Err(anyhow::Error::msg("User canceled."));
            }
            Ok(YesNo::Yes) => {}
        }
    }
    Ok(())
}

pub fn collection_add(args: CollectionAdd) -> anyhow::Result<()> {
    // TODO check yes for create new db
    // TODO check for existing col
    let conn = write::open_or_make(&args.db_file_name)?;
    let rdb = read::Db::new(&conn)?;
    let norm_path = crate::fslayer::read::normalized_abspath(&args.path);
    check_with_existing_paths(&rdb, &norm_path)?;

    let mut filter = rdb.glob_filter_by_id(args.filter_id)?;
    let mut wdb = write::Db::new(&conn)?;
    coll_builder::new_collection(
        &mut wdb,
        &args.name,
        &Path::new(&norm_path),
        args.filter_id,
        &mut filter,
        &|e| {
            println!("new entry {:?}", &e);
        },
    )?;
    Ok(())
}

pub fn collection_list(db_file_name: &str) -> anyhow::Result<()> {
    let conn = read::open_existing(db_file_name)?;
    let rdb = read::Db::new(&conn)?;
    let cols = rdb.enum_collections()?;
    for col in cols {
        println!("{},{}", col.coll_name, col.fs_path);
    }
    Ok(())
}

pub fn collection_tree(db_file_name: &str, col_name: &str) -> anyhow::Result<()> {
    let conn = read::open_existing(db_file_name)?;
    let mut rdb = read::Db::new(&conn)?;
    let col = rdb.collection_by_name(col_name)?;
    if col.is_none() {
        return Err(anyhow::Error::msg("No such collection."));
    }
    let col = col.unwrap();

    // stack of subdirs to visit
    let mut root_ids = Vec::<RootIdAndOffset>::new();
    root_ids.push(RootIdAndOffset {
        root_id: col.root_id,
        depth: 0,
        display_at: 0,
    });
    let mut tree_items = Vec::<TreeItem>::new();
    // dir, subdirs..., subdirs of subdirs,next dir
    let mut display_order = Vec::<usize>::new();

    while let Some(root_id_offset) = root_ids.pop() {
        let cur_depth = root_id_offset.depth + 1;
        let mut cur_display_at = root_id_offset.display_at;
        let contents = rdb.enum_dir_entries(root_id_offset.root_id)?;
        for diritem in contents {
            let new_item_index = tree_items.len();
            display_order.insert(cur_display_at, new_item_index);
            cur_display_at += 1;
            tree_items.push(TreeItem {
                depth: cur_depth,
                name: diritem.fs_name,
                size: diritem.fs_size,
            });
            if diritem.is_dir {
                root_ids.push(RootIdAndOffset {
                    root_id: diritem.id,
                    depth: cur_depth,
                    display_at: display_order.len(),
                });
            }
        }
    }

    for display_index in display_order {
        let item = &tree_items[display_index];
        for _ in 0..item.depth {
            print!(" ");
        }
        println!("{}\t{}", item.name, item.size);
    }
    Ok(())
}

struct RootIdAndOffset {
    pub root_id: DbId,
    pub depth: u32,
    pub display_at: usize,
}

struct TreeItem {
    pub depth: u32,
    pub name: String,
    pub size: i64,
}
