-- some global metadata
BEGIN TRANSACTION;
CREATE TABLE app_info(info_key TEXT UNIQUE NOT NULL, info_value TEXT NOT NULL);
INSERT INTO app_info VALUES('db_version', '1');
-- app must update this field on write operations
INSERT INTO app_info VALUES('app_version', '1');
COMMIT TRANSACTION;

-- settings used to populate the db, e.g. include/exclude glob masks, mime types
BEGIN TRANSACTION;
CREATE TABLE settings(setting_key TEXT UNIQUE NOT NULL, setting_value TEXT NOT NULL);
-- rewrite sidecar .xmp file on adding/removing tags
INSERT INTO settings VALUES('update_xmp', '1');
COMMIT TRANSACTION;

-- general storage for glob patterns
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

-- fill in default exclude list
BEGIN TRANSACTION;
CREATE TABLE exclude_globs(id INTEGER PRIMARY KEY, glob_pattern_id INTEGER UNIQUE NOT NULL);
INSERT INTO exclude_globs(glob_pattern_id) VALUES(1);
INSERT INTO exclude_globs(glob_pattern_id) VALUES(2);
INSERT INTO exclude_globs(glob_pattern_id) VALUES(3);
INSERT INTO exclude_globs(glob_pattern_id) VALUES(4);
INSERT INTO exclude_globs(glob_pattern_id) VALUES(5);
INSERT INTO exclude_globs(glob_pattern_id) VALUES(6);
COMMIT TRANSACTION;

-- empty tables means include every directory entry that doesn't match exclude_globs
CREATE TABLE include_globs(id INTEGER PRIMARY KEY, glob_pattern_id INTEGER UNIQUE NOT NULL);

-- (date)time values are the total hnsecs from midnight, January 1st, 1 A.D. UTC. 
-- An hnsec (hecto-nanosecond) is 100 nanoseconds. There are 10,000,000 hnsecs in a second.
-- https://dlang.org/phobos/std_datetime_systime.html#.SysTime.stdTime

-- collection reference, user name, path on the filesystem, last syncronization
-- root id from dir_entries table
CREATE TABLE collections(id INTEGER PRIMARY KEY, coll_name TEXT NOT NULL,
    fs_path TEXT NOT NULL,
    last_sync_time INTEGER NOT NULL, root_id INTEGER NOT NULL);

-- last_sync_time must be updated when metadata is reread from the file for xmp/db.
-- when adding a new collection, a fake root item must be added with fs_name '/'
CREATE TABLE dir_entries(id INTEGER PRIMARY KEY, fs_name TEXT NOT NULL,
    last_mod_time INTEGER NOT NULL,
    last_sync_time INTEGER NOT NULL);

-- directory to file/subdir mapping (id from dir_entries)
CREATE TABLE dir_to_sub(directory_id INTEGER NOT NULL, entry_id INTEGER NOT NULL UNIQUE);

-- subject tags, e.g. 'family'
CREATE TABLE subject_tags(id INTEGER PRIMARY KEY, tag_value TEXT UNIQUE NOT NULL);

-- assign tags to directory entries, collection is tagged by the fake root element
CREATE TABLE tag_to_dir_entry(subject_tag_id INTEGER NOT NULL, dir_entry_id INTEGER NOT NULL,
    UNIQUE(subject_tag_id, dir_entry_id));
