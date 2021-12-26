CREATE TABLE nes_rom (
    id          INTEGER PRIMARY KEY
                        NOT NULL,
    title       TEXT    NOT NULL,
    description TEXT    NOT NULL,
    filename    TEXT    NOT NULL,
    image       TEXT    NOT NULL,
    size        INTEGER NOT NULL,
    md5         TEXT    NOT NULL
);
