-- Your SQL goes here
CREATE TABLE user_models
(
    id          SERIAL PRIMARY KEY,
    name        VARCHAR(50) NOT NULL,
    avatar      VARCHAR(255),
    email       VARCHAR(100) UNIQUE,
    tel         VARCHAR(20),
    addr        VARCHAR(255),
    link        VARCHAR(255),
    sign        TEXT,
    integral    INT                  DEFAULT 0,
    ip          VARCHAR(45),
    role        VARCHAR(20) NOT NULL DEFAULT 'user' CHECK (role IN ('admin', 'user', 'guest')),
    sign_status BOOLEAN              DEFAULT false,
    created_at  TIMESTAMP            DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP            DEFAULT CURRENT_TIMESTAMP
)