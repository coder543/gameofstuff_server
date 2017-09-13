CREATE TABLE topic (
    id serial PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT now(),
    updated timestamptz NOT NULL DEFAULT now(),

    categories text[] NOT NULL,
    topic text NOT NULL
);

INSERT INTO topic (categories, topic) VALUES
    ('{"social", "taboo"}', 'what is one thing you should not do on a first date?'),
    ('{"fun", "weekend", "social"}', 'what is the best way to spend a Saturday?'),
    ('{"space", "science"}', 'how far is the horizon?')
;

CREATE TABLE game (
    id serial PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT now(),
    updated timestamptz NOT NULL DEFAULT now(),

    name       text NOT NULL,
    numplayers int NOT NULL DEFAULT 0
);

CREATE TABLE player (
    id serial PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT now(),
    updated timestamptz NOT NULL DEFAULT now(),

    game int REFERENCES game(id) ON DELETE CASCADE,

    name      text NOT NULL,
    score     int  NOT NULL DEFAULT 0,
    playernum int  NOT NULL
);

CREATE TABLE gameround (
    id serial PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT now(),
    updated timestamptz NOT NULL DEFAULT now(),

    game    int REFERENCES game(id)     ON DELETE CASCADE,
    topic   int REFERENCES topic(id)    ON DELETE CASCADE,

    num     int NOT NULL
);

CREATE TABLE answer (
    id serial PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT now(),
    updated timestamptz NOT NULL DEFAULT now(),

    gameround   int REFERENCES gameround(id)    ON DELETE CASCADE,
    player      int REFERENCES player(id)       ON DELETE CASCADE,
    
    answer text NOT NULL
);

CREATE TABLE guess (
    id serial PRIMARY KEY,
    created timestamptz NOT NULL DEFAULT now(),
    updated timestamptz NOT NULL DEFAULT now(),

    answer          int REFERENCES answer(id)   ON DELETE CASCADE,

    guessed_player  int REFERENCES player(id)   ON DELETE CASCADE,
    guesser         int REFERENCES player(id)   ON DELETE CASCADE
);

-- INSERT INTO game (name) VALUES ('test');
-- /* np = */ UPDATE game SET numplayers = numplayers + 1 WHERE id=/* game */ RETURNING numplayers;
-- INSERT INTO player (game, name, playernum) VALUES (/* game */, /* name */, /* np */);