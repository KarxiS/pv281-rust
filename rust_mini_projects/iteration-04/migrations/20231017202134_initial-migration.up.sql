/* User table */
CREATE TABLE IF NOT EXISTS "User"
(
    id              uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    ---------------------------------------------
    username        text UNIQUE NOT NULL,
    email           text UNIQUE NOT NULL,
    "name"          text        NOT NULL,
    surname         text        NOT NULL,
    bio             text        NOT NULL,
    profile_picture text        NOT NULL,
    password_hash   text        NOT NULL,
    password_salt   text        NOT NULL,
    created_at      timestamptz NOT NULL DEFAULT now(),
    edited_at       timestamptz NOT NULL DEFAULT now(),
    deleted_at      timestamptz
);

/* Post table */
CREATE TABLE IF NOT EXISTS "Post"
(
    id         uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    ---------------------------------------------
    creator_id uuid        NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    edited_at  timestamptz NOT NULL DEFAULT now(),
    deleted_at timestamptz,
    content    text        NOT NULL,

    FOREIGN KEY (creator_id) REFERENCES "User" (id)
);

/* Post creator id needs to be indexed, as it is accessed often */
CREATE INDEX IF NOT EXISTS "PostCreatorId" ON "Post" (creator_id);
/* Post's created / deleted at need to be indexed */
CREATE INDEX IF NOT EXISTS "PostCreatedDeletedAt" ON "Post" (created_at DESC, deleted_at NULLS LAST);

/* Comment table */
CREATE TABLE IF NOT EXISTS "Comment"
(
    id           uuid PRIMARY KEY     DEFAULT gen_random_uuid(),
    ---------------------------------------------
    commenter_id uuid        NOT NULL,
    post_id      uuid        NOT NULL,
    created_at   timestamptz NOT NULL DEFAULT now(),
    edited_at    timestamptz NOT NULL DEFAULT now(),
    deleted_at   timestamptz,
    content      text        NOT NULL,

    FOREIGN KEY (commenter_id) REFERENCES "User" (id),
    FOREIGN KEY (post_id) REFERENCES "Post" (id)
);

/* Comment's creator needs to be indexed */
CREATE INDEX IF NOT EXISTS "CommentCreatorId" ON "Comment" (commenter_id);
/* Comment's post id needs to be indexed */
CREATE INDEX IF NOT EXISTS "CommentPostId" ON "Comment" (post_id);
/* Comment's created / deleted at need to be indexed */
CREATE INDEX IF NOT EXISTS "CommentCreatedDeletedAt" ON "Comment" (created_at DESC, deleted_at NULLS LAST);
