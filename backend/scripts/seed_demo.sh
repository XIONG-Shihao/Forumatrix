#!/usr/bin/env bash
set -euo pipefail

DB="/app/data/app.db"
mkdir -p /app/data
: > "$DB" # ensure the file exists

sql() { sqlite3 "$DB" "$@"; }

# Wait until core tables exist (created by your app migrations)
need=$(sql "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users','posts','comments','post_likes');")
if [ "$need" -lt 4 ]; then
  echo "[seed] Waiting for tables (users, posts, comments, post_likes)…"
  for _ in $(seq 1 30); do
    sleep 0.5
    need=$(sql "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users','posts','comments','post_likes');")
    [ "$need" -ge 4 ] && break
  done
fi
need=$(sql "SELECT COUNT(*) FROM sqlite_master WHERE type='table' AND name IN ('users','posts','comments','post_likes');")
[ "$need" -lt 4 ] && { echo "[seed] Required tables missing; aborting."; exit 0; }

echo "[seed] Upserting demo users…"
sql "
INSERT OR IGNORE INTO users (email, password_hash, username, is_active, is_admin)
VALUES
 ('alice@example.com','demo-hash','alice',1,0),
 ('bob@example.com',  'demo-hash','bob',  1,0),
 ('carol@example.com','demo-hash','carol',1,0),
 ('dave@example.com', 'demo-hash','dave', 1,0);
"
ALICE=$(sql "SELECT id FROM users WHERE email='alice@example.com';")
BOB=$(sql   "SELECT id FROM users WHERE email='bob@example.com';")
CAROL=$(sql "SELECT id FROM users WHERE email='carol@example.com';")
DAVE=$(sql  "SELECT id FROM users WHERE email='dave@example.com';")

echo "[seed] Clearing previous demo content…"
# Only delete our demo content (titles prefixed with 'Demo:')
sql "
PRAGMA foreign_keys=ON;
DELETE FROM post_likes   WHERE post_id IN (SELECT id FROM posts WHERE title LIKE 'Demo:%');
DELETE FROM comment_likes WHERE comment_id IN (
  SELECT id FROM comments WHERE post_id IN (SELECT id FROM posts WHERE title LIKE 'Demo:%')
);
DELETE FROM comments     WHERE post_id IN (SELECT id FROM posts WHERE title LIKE 'Demo:%');
DELETE FROM posts        WHERE title LIKE 'Demo:%';
"

echo "[seed] Creating 10 demo posts…"
NOW=$(date +%s)
# Authors cycle through the 4 users
USERS=("$ALICE" "$BOB" "$CAROL" "$DAVE")

for i in $(seq 1 10); do
  uid="${USERS[$(( (i-1) % 4 ))]}"
  ts=$(( NOW - i*3600 ))   # i hours ago
  title="Demo: Post #$i"
  body="This is the body of demo post #$i. It showcases seeded content for the UI."
  sql "
    INSERT INTO posts (user_id, title, body, created_at, updated_at, edited, score, comment_count)
    VALUES ($uid, '$title', '$body', $ts, NULL, 0, 0, 0);
  "
done

echo "[seed] Seeding post likes (deterministic)…"
POST_IDS=$(sql "SELECT id FROM posts WHERE title LIKE 'Demo:%' ORDER BY id;")
for pid in $POST_IDS; do
  # for variability pick subset size 1..4 based on pid
  n=$(( (pid % 4) + 1 ))
  # like order: alice->dave
  i=0
  for liker in "${USERS[@]}"; do
    i=$((i+1))
    [ $i -le $n ] || break
    sql "INSERT OR IGNORE INTO post_likes (post_id, user_id, created_at) VALUES ($pid, $liker, $NOW);"
  done
done

echo "[seed] Choosing one post to host a small thread…"
TARGET_POST=$(sql "SELECT id FROM posts WHERE title LIKE 'Demo:%' ORDER BY created_at DESC LIMIT 1;")
TARGET_AUTHOR=$(sql "SELECT user_id FROM posts WHERE id=$TARGET_POST;")

echo "[seed] Inserting 3 top-level comments by non-author users…"
# commenters: the three users that are not the author
CANDS=()
for u in "${USERS[@]}"; do
  [ "$u" != "$TARGET_AUTHOR" ] && CANDS+=("$u")
done
c1=${CANDS[0]}; c2=${CANDS[1]}; c3=${CANDS[2]}

t1=$((NOW-300)); t2=$((NOW-240)); t3=$((NOW-180))
sql "
INSERT INTO comments (post_id,user_id,parent_id,body,created_at,edited,score)
VALUES
 ($TARGET_POST,$c1,NULL,'First!',            $t1,0,0),
 ($TARGET_POST,$c2,NULL,'Nice write-up 👍',  $t2,0,0),
 ($TARGET_POST,$c3,NULL,'Following along…', $t3,0,0);
"

# author replies to the second comment
PARENT2=$(sql "SELECT id FROM comments WHERE post_id=$TARGET_POST ORDER BY created_at LIMIT 1 OFFSET 1;")
t4=$((NOW-120))
sql "
INSERT INTO comments (post_id,user_id,parent_id,body,created_at,edited,score)
VALUES ($TARGET_POST,$TARGET_AUTHOR,$PARENT2,'Thanks! 🙏', $t4,0,0);
"

echo "[seed] Seeding comment likes (deterministic)…"
COMMENT_IDS=$(sql "SELECT id FROM comments WHERE post_id=$TARGET_POST ORDER BY id;")
for cid in $COMMENT_IDS; do
  # subset size 1..4 based on cid
  n=$(( (cid % 4) + 1 ))
  i=0
  for liker in "${USERS[@]}"; do
    i=$((i+1))
    [ $i -le $n ] || break
    sql "INSERT OR IGNORE INTO comment_likes (comment_id, user_id, created_at) VALUES ($cid, $liker, $NOW);"
  done
done

echo "[seed] Recomputing scores & counts from like tables…"
sql "
UPDATE posts
   SET score = (SELECT COUNT(*) FROM post_likes pl WHERE pl.post_id = posts.id);

UPDATE posts
   SET comment_count = (SELECT COUNT(*) FROM comments c WHERE c.post_id = posts.id);

UPDATE comments
   SET score = (SELECT COUNT(*) FROM comment_likes cl WHERE cl.comment_id = comments.id);
"

echo "[seed] Done."