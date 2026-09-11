#!/usr/bin/env bash
# Create one agent's territory on this box: its accounts, its directory, and
# the state store behind its member's seam.
#
#   ./deploy/create-agent.sh fred --artifact /path/to.gguf            plan only
#   ./deploy/create-agent.sh fred --artifact /path/to.gguf --apply    act
#
# **The program creates nothing and this is why the script exists.** Admin's
# inventory refuses a missing home rather than building one, per
# `weaver-admin-Spec` section 4, so every part of an agent's territory is the
# operator's to make. This file is that act written down once instead of
# remembered.
#
# **Two accounts, because the wall is two gates and one identity**, per
# `weaver-state-PRD` section 5 as ruled 2026-09-04. The service gate is
# kernel-class: the member dials the store's socket under its own account and
# the peer is verified by credential. The object gate is the database and the
# role's grants. Peer authentication welds them, the store mapping the
# member's kernel identity to its role, so the agent's own uid maps to no
# role and is refused at the second gate where it was not refused at the
# first. That refusal is verified at the end rather than assumed.
#
# **No password exists anywhere in here.** Peer authentication derives the
# object gate's identity from the kernel fact rather than asserting it a
# second time, so there is no secret to store, rotate, or leak.
set -euo pipefail

NAME=${1:-}
shift || true
APPLY=0
ARTIFACT=""
SESSION=""
while [ $# -gt 0 ]; do
  case "$1" in
    --apply)    APPLY=1 ;;
    --artifact) ARTIFACT=${2:-}; shift ;;
    --session)  SESSION=${2:-}; shift ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
  shift
done

say()  { printf '\n== %s\n' "$*"; }
plan() { printf '   %s\n' "$*"; }
die()  { printf '\nREFUSED: %s\n' "$*" >&2; exit 1; }

# The name is a unix user, a role, a database and a directory, so it is
# bounded to what all four accept without quoting.
[ -n "$NAME" ] || die "name the agent: create-agent.sh <name> --artifact <path>"
[[ "$NAME" =~ ^[a-z][a-z0-9]{1,15}$ ]] || die "the name is lowercase letters and digits, 2 to 16 characters: '$NAME'"
[ -n "$ARTIFACT" ] || die "name the artifact the decoder binds: --artifact <path>"
SESSION=${SESSION:-$NAME-001}

OPERATOR=${SUDO_USER:-$USER}
AGENT_USER="weaver-$NAME"          # the agent's own uid: the worker's identity
MEMBER_USER="weaver-$NAME-state"   # the member's uid: holds the territory
ROLE="weaver_$NAME"                # postgres spells with underscores
DATABASE="weaver_$NAME"
HOME_DIR="/home/$OPERATOR/.weaveragents/$AGENT_USER"
STATE_DIR="$HOME_DIR/state"
DECLARATION="/etc/weaver/agents/$NAME.yaml"
PGDATA=${PGDATA:-/var/lib/postgres/data}

say "plan for agent '$NAME'"
plan "agent account   $AGENT_USER      (system, nologin, the worker's uid)"
plan "member account  $MEMBER_USER     (system, nologin, owns the state territory)"
plan "operator        $OPERATOR joins group $AGENT_USER"
plan "home            /home/$AGENT_USER        the agent's own, where its tools run"
plan "directory       $HOME_DIR        root:$OPERATOR 2750"
plan "state territory $STATE_DIR       $MEMBER_USER 0700, which the agent's uid cannot enter"
plan "role            $ROLE            postgres, no password, peer only"
plan "database        $DATABASE        owned by $ROLE"
plan "admission       local $DATABASE $ROLE peer map=weaver"
plan "identity map    weaver $MEMBER_USER -> $ROLE"
plan "declaration     $DECLARATION     session $SESSION, artifact $ARTIFACT"

# What must not already be there. Creation is refused rather than merged,
# because a half-made agent that looks whole is worse than an absent one.
say "checks"
for u in "$AGENT_USER" "$MEMBER_USER"; do
  getent passwd "$u" >/dev/null && die "the account $u already exists"
done
[ -e "/home/$AGENT_USER" ] && die "the home /home/$AGENT_USER already exists"
[ -e "$HOME_DIR" ] && die "the directory $HOME_DIR already exists"
[ -e "$DECLARATION" ] && die "the declaration $DECLARATION already exists"
[ -r "$ARTIFACT" ] || printf '   WARNING: the artifact is not readable from this shell: %s\n' "$ARTIFACT"
printf '   nothing of this agent exists yet\n'

if [ "$APPLY" -eq 0 ]; then
  say "plan only"
  printf '   rerun with --apply to make it\n'
  exit 0
fi

say "accounts"
# **The agent gets a home and the member does not.** The agent's tools run
# somewhere, and the retirement of 2026-09-11 found one of three agents with
# a home and the others without, which is the drift a script ends. The member
# runs nothing and owns one room of the operator's tree instead.
sudo useradd --system --shell /usr/sbin/nologin --create-home --user-group "$AGENT_USER"
sudo useradd --system --shell /usr/sbin/nologin --no-create-home --user-group "$MEMBER_USER"
sudo usermod -aG "$AGENT_USER" "$OPERATOR"
sudo chmod 2750 "/home/$AGENT_USER"
printf '   %s uid %s, %s uid %s\n' \
  "$AGENT_USER" "$(id -u "$AGENT_USER")" "$MEMBER_USER" "$(id -u "$MEMBER_USER")"

say "territory"
# The directory is the operator's to read and the member's to own one room
# of. Setgid so the operator's group survives whatever writes here, and the
# state subdirectory closed to everyone else, which is what makes the
# charter's "one subdirectory the agent's uid cannot enter" true rather than
# stated.
sudo install -d -o root -g "$OPERATOR" -m 2750 "$HOME_DIR"
sudo install -d -o "$MEMBER_USER" -g "$MEMBER_USER" -m 0700 "$STATE_DIR"

say "store"
sudo systemctl is-active --quiet postgresql || sudo systemctl start postgresql
sudo -u postgres psql -v ON_ERROR_STOP=1 -c "CREATE ROLE $ROLE LOGIN;"
sudo -u postgres psql -v ON_ERROR_STOP=1 -c "CREATE DATABASE $DATABASE OWNER $ROLE;"

say "gates"
# The admission line precedes the catch-all, because pg_hba takes the first
# match and `local all all peer` would otherwise demand that the kernel name
# equal the role name, which is exactly what the map exists to avoid.
HBA="$PGDATA/pg_hba.conf"
IDENT="$PGDATA/pg_ident.conf"
sudo cp -a "$HBA" "$HBA.before-$NAME"
sudo cp -a "$IDENT" "$IDENT.before-$NAME"
sudo sed -i "0,/^local\s\+all\s\+all\s\+peer/s||local   $DATABASE   $ROLE   peer map=weaver\nlocal   all             all                                     peer|" "$HBA"
printf 'weaver          %s                    %s\n' "$MEMBER_USER" "$ROLE" | sudo tee -a "$IDENT" >/dev/null
sudo systemctl reload postgresql

say "declaration"
# **The sink is inside this agent's own territory and the script will not
# write it anywhere else.** A declaration of 2026-08-23 pointed one agent's
# sink at another's directory, so two agents were configured to write one
# record, and it survived three weeks because nothing checked. The path is
# derived here rather than accepted.
sudo install -d -o root -g root -m 0755 /etc/weaver/agents
sudo tee "$DECLARATION" >/dev/null <<YAML
session: $SESSION
spu-instruction:
  decoder:
    model-binding:
      artifact: $ARTIFACT
      devices: [0]
    residual-readout-election: false
    surprisal-election: true
    tunable-values:
      context-capacity: 32768
      max-tokens-per-turn: 4096
      seed: 451234785645
    identity:
      - role: system
        content:
          - type: text
            text: |
              You are a careful assistant. Answer from what you know, say
              plainly when you do not know, and keep answers as short as the
              question allows.
tool-set: []
permission-mode: deny
trace-sink:
  kind: file
  path: $HOME_DIR/trace.ndjson
  create: true
state-election:
  all-kinds: true
  keys:
    - kind: message.user
      paths: [content]
    - kind: message.assistant
      paths: [content]
# **The engine, the database and the role are members of the binding**, per
# weaver-state-PRD section 5: declared here, changing only across the load
# boundary, and named on the load event like every fact that decides a
# record.
state-store:
  engine: postgres
  database: $DATABASE
  role: $ROLE
YAML

say "both gates, verified rather than assumed"
if sudo -u "$MEMBER_USER" psql -d "$DATABASE" -c 'select 1' >/dev/null 2>&1; then
  printf '   the member reaches its database\n'
else
  die "the member cannot reach its database: the first gate or the map is wrong"
fi
if sudo -u "$AGENT_USER" psql -d "$DATABASE" -c 'select 1' >/dev/null 2>&1; then
  die "THE AGENT'S UID REACHED THE DATABASE: the second gate is open"
else
  printf "   the agent's own uid is refused, which is the gate the charter asks for\n"
fi

say "made"
printf '   validate it before loading:  weaver-admin validate --config %s\n' "$DECLARATION"
