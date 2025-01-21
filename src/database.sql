DROP TABLE IF EXISTS users cascade;

DROP TABLE IF EXISTS groups cascade;

DROP TABLE IF EXISTS user_group cascade;

CREATE TABLE users (
    user_id uuid DEFAULT gen_random_uuid () PRIMARY KEY,
    user_email varchar UNIQUE NOT NULL,
    user_password varchar NOT NULL, -- 보안 목적으로 'password_hash'로 이름 변경 권장
    user_name varchar NOT NULL,
    created_at timestamp DEFAULT now ()
);

CREATE TABLE groups (
    group_id uuid DEFAULT gen_random_uuid () PRIMARY KEY,
    group_address varchar NOT NULL,
    group_image varchar NOT NULL,
    group_name varchar NOT NULL,
    group_description text,
    created_at timestamp DEFAULT now ()
);

CREATE TABLE user_group (
    user_id uuid NOT NULL,
    group_id uuid NOT NULL,
    joined_at timestamp DEFAULT now (),
    UNIQUE (user_id, group_id) -- 인덱스와 UNIQUE 제약 조건을 동시에 설정
);

ALTER TABLE user_group ADD FOREIGN KEY (group_id) REFERENCES groups (group_id) ON DELETE CASCADE;

ALTER TABLE user_group ADD FOREIGN KEY (user_id) REFERENCES users (user_id) ON DELETE CASCADE;

-- 테스트 데이터 삽입
INSERT INTO
    users (user_email, user_password, user_name)
VALUES
    (
        'user@example.com',
        'hashed_password_example',
        'user'
    );

-- 해싱된 비밀번호 사용
