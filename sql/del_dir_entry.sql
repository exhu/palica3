--BEGIN TRANSACTION;

DELETE FROM dir_to_sub WHERE entry_id = ?1;
DELETE FROM dir_entries WHERE id = ?1;
DELETE FROM tag_to_dir_entry WHERE dir_entry_id = ?1;
DELETE FROM last_edit WHERE dir_entry_id = ?1;
DELETE FROM mime_to_dir_entry WHERE dir_entry_id = ?1;

-- need to repeat the same recursively for all
-- SELECT FROM dir_to_sub WHERE directory_id = ?1;

--COMMIT TRANSACTION;
