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
# `weaver-state-PRD` section 4 as ruled 2026-09-04. The service gate is
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

say()  { printf '\n== %s\n' "$*"; }
plan() { printf '   %s\n' "$*"; }
die()  { printf '\nREFUSED: %s\n' "$*" >&2; exit 1; }

NAME=${1:-}
shift || true
APPLY=0
ARTIFACT=""
SESSION=""
MEMBER_IDENTITY=""
while [ $# -gt 0 ]; do
  case "$1" in
    --apply)    APPLY=1 ;;
    # **The value is required before the shift consumes it.** Without the
    # arity check the inline shift runs past the end of the list and `set -e`
    # exits with no message at all, which is a worse answer than the refusal
    # below.
    --artifact) [ $# -ge 2 ] || die "--artifact needs a path"; ARTIFACT=$2; shift ;;
    --session)  [ $# -ge 2 ] || die "--session needs a name"; SESSION=$2; shift ;;
    --member-identity) [ $# -ge 2 ] || die "--member-identity needs an account"; MEMBER_IDENTITY=$2; shift ;;
    *) printf 'unknown argument: %s\n' "$1" >&2; exit 2 ;;
  esac
  shift
done


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
ADMIN_CONFIG=${WEAVER_ADMIN_CONFIG:-/etc/weaver/config}
AGENTS_DIR=$(sudo -n cat "$ADMIN_CONFIG/agent-config-directory" 2>/dev/null || echo /etc/weaver/agents)
ALLOW_LIST="$ADMIN_CONFIG/allow-list"
DECLARATION="$AGENTS_DIR/$NAME.yaml"

# **Whose identity the store admits is unresolved and this script will not
# guess it.** The charter has the member hold a uid of its own and dial the
# store under it. The code does not do that yet: `weaver-admin` asks the
# store from its own process, so the identity that must be admitted is
# whichever account admin runs as, which is root today. Mapping the member's
# account would provision for a design nothing implements, and mapping root
# silently would bake in the weaker property. So the operator names it.
[ -n "$MEMBER_IDENTITY" ] || die "name the account the store must admit: --member-identity <account>
   'root' matches what weaver-admin runs as today and works now.
   '$MEMBER_USER' is the charter's design and needs admin's privilege drop first."
getent passwd "$MEMBER_IDENTITY" >/dev/null || [ "$MEMBER_IDENTITY" = "$MEMBER_USER" ] \
  || die "no such account: $MEMBER_IDENTITY"
HBA=""; IDENT=""   # asked of the store itself rather than guessed from a distro path

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
plan "identity map    weaver $MEMBER_IDENTITY -> $ROLE"
plan "allow-list      $NAME appended to $ALLOW_LIST"
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
sudo -n grep -qxF "$NAME" "$ALLOW_LIST" 2>/dev/null && die "$NAME is already in $ALLOW_LIST"
# **The store's catalogs are asked before anything local is made.** Retiring
# an agent leaves its role and database behind unless they were dropped by
# hand, and a collision at CREATE ROLE would otherwise die after both
# accounts and both directories exist, leaving the half-made agent this
# script refuses to produce.
if sudo -n systemctl is-active --quiet postgresql 2>/dev/null; then
  sudo -n -u postgres psql -tAc "select 1 from pg_roles where rolname='$ROLE'" 2>/dev/null | grep -q 1 \
    && die "the role $ROLE already exists: drop it or pick another name"
  sudo -n -u postgres psql -tAc "select 1 from pg_database where datname='$DATABASE'" 2>/dev/null | grep -q 1 \
    && die "the database $DATABASE already exists: drop it or pick another name"
  HBA=$(sudo -n -u postgres psql -tAc 'show hba_file' 2>/dev/null | tr -d ' ')
  IDENT=$(sudo -n -u postgres psql -tAc 'show ident_file' 2>/dev/null | tr -d ' ')
  printf '   the store is up and carries no %s\n' "$ROLE"
else
  printf '   the store is down, so its catalogs are unchecked until --apply starts it\n'
fi
[ -r "$ARTIFACT" ] || printf '   WARNING: the artifact is not readable from this shell: %s\n' "$ARTIFACT"
printf '   nothing of this agent exists yet\n'
# **Traversal is asked about here rather than discovered halfway through.**
# root reaches every directory whatever the mode says, so no entry is needed
# or written for it. Any other identity needs passage along a chain that runs
# through the operator's own home, and this pool answers `setfacl` with
# Operation not supported, so the need and the means are checked together
# before anything is made.
if [ "$MEMBER_IDENTITY" = "root" ]; then
  printf '   root traverses by identity, so no access entry is needed\n'
else
  probe=$(mktemp -d "/home/$OPERATOR/.acl-probe-XXXXXX") || die "cannot write under /home/$OPERATOR"
  if setfacl -m "u:$MEMBER_IDENTITY:x" "$probe" 2>/dev/null; then
    printf '   this filesystem carries access entries, so %s can be given passage\n' "$MEMBER_IDENTITY"
  else
    rmdir "$probe"
    die "this filesystem refuses access entries, so $MEMBER_IDENTITY cannot traverse to its
   territory under /home/$OPERATOR. Either map root, or place the territory
   somewhere the member can reach by ownership alone."
  fi
  rmdir "$probe"
fi

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
# **Owning the room is not reaching it.** The operator's home is 0700 and
# every directory above the territory belongs to the operator, so the member
# cannot traverse to what it owns. Execute-only entries along the chain open
# passage without opening any listing, which is the narrowest thing that
# makes the ownership above true rather than stated.
if [ "$MEMBER_IDENTITY" != "root" ]; then
  for step in "/home/$OPERATOR" "/home/$OPERATOR/.weaveragents" "$HOME_DIR"; do
    sudo setfacl -m "u:$MEMBER_IDENTITY:x" "$step" \
      || die "no traversal for $MEMBER_IDENTITY at $step, and the member cannot reach its own territory"
  done
fi

say "store"
sudo systemctl is-active --quiet postgresql || sudo systemctl start postgresql
sudo -u postgres psql -v ON_ERROR_STOP=1 -c "CREATE ROLE $ROLE LOGIN;"
sudo -u postgres psql -v ON_ERROR_STOP=1 -c "CREATE DATABASE $DATABASE OWNER $ROLE;"

say "gates"
# The admission line precedes the catch-all, because pg_hba takes the first
# match and `local all all peer` would otherwise demand that the kernel name
# equal the role name, which is exactly what the map exists to avoid.
[ -n "$HBA" ] || HBA=$(sudo -u postgres psql -tAc 'show hba_file' | tr -d ' ')
[ -n "$IDENT" ] || IDENT=$(sudo -u postgres psql -tAc 'show ident_file' | tr -d ' ')
sudo test -f "$HBA" || die "the store names no readable hba file: $HBA"
sudo cp -a "$HBA" "$HBA.before-$NAME"
sudo cp -a "$IDENT" "$IDENT.before-$NAME"
# **The admission goes before the catch-all or the map is never consulted**,
# pg_hba taking the first match and `local all all peer` demanding that the
# kernel name equal the role name. A cluster without that catch-all needs a
# different anchor, so its absence refuses rather than substituting nothing
# and failing later with a message naming the wrong cause.
sudo grep -qE '^local[[:space:]]+all[[:space:]]+all[[:space:]]+peer' "$HBA" \
  || die "no 'local all all peer' line in $HBA to place the admission before"
sudo sed -i "0,/^local\s\+all\s\+all\s\+peer/s||local   $DATABASE   $ROLE   peer map=weaver\nlocal   all             all                                     peer|" "$HBA"
printf 'weaver          %s                    %s\n' "$MEMBER_IDENTITY" "$ROLE" | sudo tee -a "$IDENT" >/dev/null
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
# **A serving binding carries a gate instruction and the inventory refuses it
# absent.** An unstated binding-kind resolves to serving, so both are written
# rather than left to a default a reader cannot see.
binding-kind: serving
gate-instruction:
  access-rule:
    allowed-uids: [$(id -u "$OPERATOR")]
    allowed-gids: []
    denied-uids: []
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
# weaver-state-PRD section 4: declared here, changing only across the load
# boundary, and named on the load event like every fact that decides a
# record.
state-store:
  engine: postgres
  database: $DATABASE
  role: $ROLE
YAML

say "allow-list"
# Without this every admin verb answers NoSuchAgent for the agent just made,
# which is the one hand-step this script exists to remove.
printf '%s\n' "$NAME" | sudo tee -a "$ALLOW_LIST" >/dev/null
printf '   %s admitted in %s\n' "$NAME" "$ALLOW_LIST"

say "both gates, verified rather than assumed"
# **Each probe names the role.** Without `-U` psql defaults the role to the
# connecting account's own name, so the check would ask about a role nobody
# created and fail for a reason that is not the gate.
if sudo -u "$MEMBER_IDENTITY" psql -U "$ROLE" -d "$DATABASE" -c 'select 1' >/dev/null 2>&1; then
  printf '   %s reaches the database as %s\n' "$MEMBER_IDENTITY" "$ROLE"
else
  die "the member cannot reach its database: the first gate or the map is wrong"
fi
if sudo -u "$AGENT_USER" psql -U "$ROLE" -d "$DATABASE" -c 'select 1' >/dev/null 2>&1; then
  die "THE AGENT'S UID REACHED THE DATABASE: the second gate is open"
else
  printf "   the agent's own uid is refused, which is the gate the charter asks for\n"
fi

say "made"
printf '   validate it before loading:\n'
printf '     sudo WEAVER_ADMIN_CONFIG=%s weaver-admin validate %s\n' "$ADMIN_CONFIG" "$NAME"
