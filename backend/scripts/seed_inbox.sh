#!/usr/bin/env bash
# Seed 6 users and have each DM the admin, then test unread + mark-read.
# Usage:
#   ./backend/scripts/seed_inbox.sh
#   BASE=http://localhost:8080 ADMIN_ID=1 ADMIN_EMAIL=admin@example.com ./backend/scripts/seed_inbox.sh

set -euo pipefail

BASE="${BASE:-http://localhost:8080}"
ADMIN_ID="${ADMIN_ID:-1}"
ADMIN_EMAIL="${ADMIN_EMAIL:-admin@example.com}"
PASS="ChatPass123!"
COUNT=6

if ! command -v jq >/dev/null 2>&1; then
  echo "This script requires 'jq' (JSON CLI). Install it and re-run." >&2
  exit 1
fi

echo "== Config =="
echo "API base   : $BASE"
echo "Admin ID   : $ADMIN_ID"
echo "Admin email: $ADMIN_EMAIL"
echo "Seed count : $COUNT"
echo

TMPDIR="$(mktemp -d -t seed-chats-XXXXXX)"
cleanup() { rm -rf "$TMPDIR"; }
trap cleanup EXIT

# --- 1) Create 6 chatters and DM admin once each ---
for i in $(seq 1 $COUNT); do
  EMAIL="chatter${i}@example.com"
  USER="chatter${i}"
  COOKIE="${TMPDIR}/chatter${i}.cookie"

  echo "==> Register/login ${USER} (${EMAIL})"
  # Register (ignore conflict)
  curl -sS "$BASE/api/auth/register" \
    -H 'Content-Type: application/json' \
    -d "{\"email\":\"$EMAIL\",\"username\":\"$USER\",\"password\":\"$PASS\"}" >/dev/null || true

  # Login (store session cookie)
  curl -sS -c "$COOKIE" "$BASE/api/auth/login" \
    -H 'Content-Type: application/json' \
    -d "{\"identifier\":\"$EMAIL\",\"password\":\"$PASS\"}" \
    | jq -c '.'

  echo "==> Open (or get) chat with admin ($ADMIN_ID)"
  OPEN_RESP=$(curl -sS -b "$COOKIE" "$BASE/api/chats/open" \
    -H 'Content-Type: application/json' \
    -d "{\"peer_id\": $ADMIN_ID}")

  CHAT_ID=$(echo "$OPEN_RESP" | jq -r '.chat_id // empty')
  if [[ -z "$CHAT_ID" ]]; then
    echo "Failed to open chat for $USER -> admin. Response:" >&2
    echo "$OPEN_RESP" >&2
    continue
  fi
  echo "Chat id: $CHAT_ID"

  MSG="Hi admin, I'm ${USER} 👋"
  echo "==> ${USER} sends: $MSG"
  curl -sS -b "$COOKIE" "$BASE/api/chats/${CHAT_ID}/messages" \
    -H 'Content-Type: application/json' \
    -d "$(jq -nc --arg t "$MSG" '{text:$t}')" \
    | jq -c '.message | {id, chat_id, sender_id, created_at}'
  echo
done

# --- 2) Login as admin ---
ADMIN_COOKIE="${TMPDIR}/admin.cookie"
echo "==> Login admin ($ADMIN_EMAIL)"
curl -sS -c "$ADMIN_COOKIE" "$BASE/api/auth/login" \
  -H 'Content-Type: application/json' \
  -d "{\"identifier\":\"$ADMIN_EMAIL\",\"password\":\"AdminPass123!\"}" \
  | jq -c '.'

# --- 3) Total unread before reading ---
echo "==> Admin unread_count BEFORE reading"
curl -sS -b "$ADMIN_COOKIE" "$BASE/api/chats/unread_count" | jq -c '.'

# --- 4) List chats with unread_count (debug to stderr only) ---
echo "==> Admin list chats (page=1, limit=20)"
LIST_JSON=$(curl -sS -b "$ADMIN_COOKIE" "$BASE/api/chats?page=1&limit=20")
echo "$LIST_JSON" | jq -c '.items | map({chat_id:.id, peer:.peer_username, unread:.unread_count})' >&2

# Pick ONE chat to mark as read: prefer one with unread>0, else first
TARGET_CHAT_ID=$(echo "$LIST_JSON" \
  | jq -r '.items[] | select(.unread_count > 0) | .id' \
  | head -n1)

if [[ -z "${TARGET_CHAT_ID:-}" ]]; then
  # fallback to first chat id if any
  TARGET_CHAT_ID=$(echo "$LIST_JSON" | jq -r '.items[0].id // empty')
fi

if [[ -z "${TARGET_CHAT_ID:-}" ]]; then
  echo "No chats found for admin; nothing to mark as read."
  exit 0
fi

echo "Chosen chat to mark read: $TARGET_CHAT_ID"

# --- 5) Mark that chat as read ---
echo "==> PUT /api/chats/${TARGET_CHAT_ID}/read"
curl -sS -X PUT -b "$ADMIN_COOKIE" "$BASE/api/chats/${TARGET_CHAT_ID}/read" | jq -c '.'

# --- 6) Total unread after reading one chat ---
echo "==> Admin unread_count AFTER reading one chat"
curl -sS -b "$ADMIN_COOKIE" "$BASE/api/chats/unread_count" | jq -c '.'

# --- 7) Verify chosen chat now has unread_count=0 (debug to stderr) ---
curl -sS -b "$ADMIN_COOKIE" "$BASE/api/chats?page=1&limit=20" \
  | jq -c --argjson id "$TARGET_CHAT_ID" '
      .items
      | map(select(.id == $id) | {chat_id:.id, peer:.peer_username, unread:.unread_count})
    ' >&2

echo
echo "==> Done."