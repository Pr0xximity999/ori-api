-- Add up migration script here
CREATE TABLE IF NOT EXISTS sentences (
	id UUID NOT NULL,
	sentence TEXT NOT NULL,
	CONSTRAINT sentences_pk PRIMARY KEY (id)
);
