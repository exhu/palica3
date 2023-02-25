-- some metadata
BEGIN TRANSACTION;
CREATE TABLE app_info(info_key TEXT UNIQUE NOT NULL, info_value TEXT NOT NULL);
-- schema version
INSERT INTO app_info VALUES('db_version', '1');
-- app must update this field on write operations
INSERT INTO app_info VALUES('app_version', '1');
-- public mime-type of blobs
INSERT INTO app_info VALUES('mime', 'image/jxl');
INSERT INTO app_info VALUES('max_width', '256');
INSERT INTO app_info VALUES('max_height', '256');
-- command line tool
-- [^{]\{input\}[^}]
-- [^{]\{output\}[^}]
INSERT INTO app_info VALUES('cmd_line', 'cjxl -j 0 {input} {output}.jxl');
INSERT INTO app_info VALUES('hash_type', 'sha256');
-- if running the command then 'cmd_line', if using a lib, e.g. 'libjxl'
INSERT INTO app_info VALUES('produced_by', 'cmd_line');
COMMIT TRANSACTION;

-- store path to restore in case a hash collides or the metadata db is broken.
-- virtual path, e.g. 'collection_name/my_file.jpg'
CREATE TABLE src_item(id INTEGER PRIMARY KEY, v_path TEXT UNIQUE NOT NULL,
    fs_mod_time INTEGER NOT NULL);

CREATE TABLE hash_values(id INTEGER PRIMARY KEY, hash_value TEXT NOT NULL UNIQUE);

CREATE TABLE src_to_hash(src_item_id INTEGER NOT NULL UNIQUE,
    hash_value_id INTEGER NOT NULL);

-- actual data produced by running the 'cmd_line' or a library in 'produced_by'
CREATE TABLE thumbnail_blob(id INTEGER PRIMARY KEY, binary_data BLOB NOT NULL);

-- find a blob by a src_item
CREATE TABLE src_to_thumbnail(src_item_id INTEGER UNIQUE NOT NULL,
    thumbnail_blob_id INTEGER NOT NULL);
