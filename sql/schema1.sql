-- some global metadata
BEGIN TRANSACTION;
CREATE TABLE app_info(id INTEGER PRIMARY KEY, info_key TEXT UNIQUE NOT NULL, info_value TEXT NOT NULL);
-- schema version
INSERT INTO app_info(info_key, info_value) VALUES('db_version', '1');
-- app must update this field on write operations
INSERT INTO app_info(info_key, info_value) VALUES('app_version', '1');
COMMIT TRANSACTION;

-- settings used to populate the db, e.g. include/exclude glob masks, mime types
BEGIN TRANSACTION;
CREATE TABLE settings(id INTEGER PRIMARY KEY, setting_key TEXT UNIQUE NOT NULL, setting_value TEXT NOT NULL);
-- rewrite sidecar .xmp file on adding/removing tags
INSERT INTO settings(setting_key, setting_value)
    VALUES('update_xmp', '1');
INSERT INTO settings(setting_key, setting_value)
    VALUES('default_filter', 'default');
COMMIT TRANSACTION;

-- general storage for glob patterns, which can be used for filtering
-- during populating the collection, or automatically tagging the files.
BEGIN TRANSACTION;
CREATE TABLE glob_patterns(id INTEGER PRIMARY KEY, regexp TEXT NOT NULL);
INSERT INTO glob_patterns(id, regexp) VALUES(1, '/\.git$');
INSERT INTO glob_patterns(id, regexp) VALUES(2, '/\.hg$');
INSERT INTO glob_patterns(id, regexp) VALUES(3, '/\.svn$');
INSERT INTO glob_patterns(id, regexp) VALUES(4, '/\.thumbnails$');
INSERT INTO glob_patterns(id, regexp) VALUES(5, '/\.DS_Store$');
INSERT INTO glob_patterns(id, regexp) VALUES(6, '(?i)/Thumbs.db$');
INSERT INTO glob_patterns(id, regexp) VALUES(7, '(?i)jpe?g$');
INSERT INTO glob_patterns(id, regexp) VALUES(8, '(?i)orf$');
INSERT INTO glob_patterns(id, regexp) VALUES(9, '(?i)cr.$');
INSERT INTO glob_patterns(id, regexp) VALUES(10, '(?i)xmp$');
COMMIT TRANSACTION;

BEGIN TRANSACTION;
CREATE TABLE glob_filters(id INTEGER PRIMARY KEY, name TEXT UNIQUE NOT NULL);
-- 'include': 0 = exclude, 1 = include files matched with
-- 'position' used for sorting, order of glob include/exclude application
-- for very complex filtering, e.g. give three entries with positions
-- 1) exclude all */some-dir/* files but
-- 2) include jpegs
-- 3) but if it's some '.preview', then exclude, it's easy to reason about.
INSERT INTO glob_filters(id, name) VALUES(1, 'default');

CREATE TABLE glob_filter_to_pattern(id INTEGER PRIMARY KEY,
    filter_id INTEGER NOT NULL, glob_pattern_id INTEGER NOT NULL,
    include INTEGER NOT NULL,
    position INTEGER NOT NULL);
-- fill in default exclude list
INSERT INTO glob_filter_to_pattern(filter_id, glob_pattern_id, include,
    position) VALUES(1, 1, 0, 10);
INSERT INTO glob_filter_to_pattern(filter_id, glob_pattern_id, include,
    position) VALUES(1, 2, 0, 11);
INSERT INTO glob_filter_to_pattern(filter_id, glob_pattern_id, include,
    position) VALUES(1, 3, 0, 12);
INSERT INTO glob_filter_to_pattern(filter_id, glob_pattern_id, include,
    position) VALUES(1, 4, 0, 13);
INSERT INTO glob_filter_to_pattern(filter_id, glob_pattern_id, include,
    position) VALUES(1, 5, 0, 14);
INSERT INTO glob_filter_to_pattern(filter_id, glob_pattern_id, include,
    position) VALUES(1, 6, 0, 15);
COMMIT TRANSACTION;

-- collection reference, user name, path on the filesystem, last syncronization
-- root id from dir_entries table
CREATE TABLE collections(id INTEGER PRIMARY KEY, coll_name TEXT UNIQUE NOT NULL,
    fs_path TEXT NOT NULL, root_id INTEGER NOT NULL);

-- timestamps in nanoseconds since unix epoch
-- last_sync_time must be updated when metadata is reread from the file for xmp/db.
-- when adding a new collection
-- fs_size is for files only, 0 for dirs.
CREATE TABLE dir_entries(id INTEGER PRIMARY KEY, fs_name TEXT NOT NULL,
    fs_mod_time INTEGER NOT NULL,
    last_sync_time INTEGER NOT NULL,
    is_dir INTEGER NOT NULL,
    fs_size INTEGER NOT NULL);

-- directory to file/subdir mapping (id from dir_entries)
CREATE TABLE dir_to_sub(id INTEGER PRIMARY KEY, directory_id INTEGER NOT NULL, entry_id INTEGER NOT NULL UNIQUE);

-- subject tags, e.g. 'family'
CREATE TABLE subject_tags(id INTEGER PRIMARY KEY, tag_value TEXT UNIQUE NOT NULL);

-- assign tags to directory entries, collection is tagged by the root element
CREATE TABLE tag_to_dir_entry(id INTEGER PRIMARY KEY, subject_tag_id INTEGER NOT NULL, dir_entry_id INTEGER NOT NULL,
    UNIQUE(subject_tag_id, dir_entry_id));

-- last metadata edit
CREATE TABLE last_edit(id INTEGER PRIMARY KEY, dir_entry_id INTEGER NOT NULL UNIQUE,
    date_time INTEGER NOT NULL UNIQUE);

-- e.g. image/jxl
CREATE TABLE mime_types(id INTEGER PRIMARY KEY, mime_value TEXT NOT NULL UNIQUE);
CREATE TABLE mime_to_dir_entry(id INTEGER PRIMARY KEY,
    mime_type_id INTEGER NOT NULL, dir_entry_id INTEGER NOT NULL,
    UNIQUE(mime_type_id, dir_entry_id));
