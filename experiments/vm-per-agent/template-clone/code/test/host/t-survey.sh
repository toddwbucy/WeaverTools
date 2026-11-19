#!/usr/bin/env bash
# The survey the image carries, guest/ready/weaver-survey (weaver-survey-format; the ready
# layer of weaver-appliance-build-Spec section 2): its own tests, in guest/ready/test/,
# against made-up machines and a reader written from the format's section 3, checked
# first against WeaverWeb's worked example. They name a card's driver and its tools and
# run none of them but the survey itself, which finds no stamp and writes nothing. Then the
# survey as the image carries it: no suffix, run by python3 in isolated mode, and the path
# the ready step lays it at. Pure logic: no root, no guest.
source "$(dirname "$0")/lib-test.sh"
need python3
S=$CODE/guest/ready/weaver-survey
out=$(TMPDIR=$T python3 -I "$CODE/guest/ready/test/t-weaver-survey.py" 2>&1); rc=$?
printf '%s\n' "$out" | sed 's/^/    /'
check "the survey's own tests pass" test "$rc" = 0
check "and say so: none failed" grep -qE '^== [0-9]+ passed, 0 failed' <<<"$out"
first=$(sed -n 1p "$S")
check "the survey runs as python3 from its first line" test "$first" = "#!/usr/bin/env python3"
check "its usage names it as the image carries it, with no suffix" grep -qF 'usage: weaver-survey --stage pairing|drives' "$S"
eval "$(grep -E '^READY_SURVEY=' "$CODE/guest/template-steps")"
check "the ready step lays it at /opt/weaver-appliance/bin/weaver-survey" test "$READY_SURVEY" = /opt/weaver-appliance/bin/weaver-survey
check "the survey is ASCII" bash -c '! LC_ALL=C grep -qP "[^\x00-\x7F]" "$1"' _ "$S"
finish
