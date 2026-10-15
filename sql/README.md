# Historical SQL archive — do not run

The original 000–041 scripts are retained unchanged here for review, including
the two 017 files and the 014/019 data-reset scripts. They are not an executable
migration sequence and are not read by SQLx's default migration runner.

The official fresh-install history is now `../migrations/`. See
`../docs/database-baseline.md` before initializing or adopting a database.
Never point `sqlx migrate run --source` at this historical directory.
