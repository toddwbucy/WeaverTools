"""survey_reader: a reader of weaver-survey/1 written from the format document's section 3
(WeaverTools docs/profiles/weaver-survey-format.md at e16aecd): reader(raw) returns the
report's members or raises Refuse. Standard library only."""
import json, re, time


class Refuse(Exception):
    pass


def no_dup(pairs):
    seen = set()
    for k, _ in pairs:
        if k in seen:
            raise Refuse("the member %r twice in one object" % k)
        seen.add(k)
    return dict(pairs)


def no_constant(s):
    raise Refuse("NaN or Infinity, which JSON does not have: %s" % s)


def reader(raw):
    """The report's members, or Refuse."""
    if len(raw) > 65536:
        raise Refuse("over 64 KiB")
    try:
        t = raw.decode("utf-8")
    except UnicodeDecodeError:
        raise Refuse("not UTF-8")
    if t.startswith("\ufeff"):
        raise Refuse("a byte-order mark")
    # Fractions and exponents parse as floats: a member the reader does not know is kept and
    # ignored at any depth (section 3), and get() refuses a float where an integer is known
    # (CodeRabbit on pull request 44, 2026-11-19). NaN and Infinity are refused anywhere.
    dec = json.JSONDecoder(object_pairs_hook=no_dup, parse_constant=no_constant)
    start = len(t) - len(t.lstrip(" \t\r\n"))
    try:
        obj, end = dec.raw_decode(t, start)
    except ValueError as e:
        raise Refuse("not JSON: %s" % e)
    if t[end:] not in ("", "\n"):
        raise Refuse("something after the object: %r" % t[end:end + 20])
    if not isinstance(obj, dict):
        raise Refuse("not one JSON object")
    if obj.get("format") != "weaver-survey/1":
        raise Refuse("format %r" % obj.get("format"))

    def get(o, k, typ, where):
        if not isinstance(o, dict) or k not in o or o[k] is None:
            raise Refuse("%s.%s missing" % (where, k))
        v = o[k]
        if typ is int and (isinstance(v, bool) or not isinstance(v, int)):
            raise Refuse("%s.%s is not an integer" % (where, k))
        if typ is not int and not isinstance(v, typ):
            raise Refuse("%s.%s is not %s" % (where, k, typ.__name__))
        return v

    def s(o, k, bound, where, pattern=None):
        v = get(o, k, str, where)
        if any(not (0x20 <= ord(c) <= 0x7E) for c in v) or len(v) > bound or (pattern and not re.fullmatch(pattern, v)):
            raise Refuse("%s.%s %r breaks its bound" % (where, k, v))
        return v

    def i(o, k, where, low=0, high=(1 << 53) - 1):
        v = get(o, k, int, where)
        if not (low <= v <= high):
            raise Refuse("%s.%s %d outside %d to %d" % (where, k, v, low, high))
        return v

    stage = get(obj, "stage", str, "")
    if stage not in ("pairing", "drives"):
        raise Refuse("stage %r" % stage)
    taken = get(obj, "taken", str, "")
    try:
        if not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z", taken):
            raise ValueError
        time.strptime(taken, "%Y-%m-%dT%H:%M:%SZ")
    except ValueError:
        raise Refuse("taken %r" % taken)
    img = get(obj, "image", dict, "")
    s(img, "version", 65536, "image", r"[0-9]+\.[0-9]+")
    s(img, "manifest_sha256", 64, "image", r"[0-9a-f]{64}")
    s(get(obj, "kernel", dict, ""), "release", 64, "kernel")
    m = get(obj, "machine", dict, "")
    s(m, "virtualization", 32, "machine")
    for k in ("vendor", "product", "version"):
        s(m, k, 128, "machine")
    c = get(obj, "cpu", dict, "")
    s(c, "vendor", 32, "cpu"); s(c, "model_name", 128, "cpu")
    for k in ("family", "model", "stepping"):
        i(c, k, "cpu", 0, 65535)
    i(c, "logical_cpus", "cpu", 1, 4096)
    flags = get(c, "flags", list, "cpu")
    if not (1 <= len(flags) <= 512):
        raise Refuse("cpu.flags has %d entries" % len(flags))
    for f in flags:
        if not isinstance(f, str) or not re.fullmatch(r"[a-z0-9_]{1,32}", f):
            raise Refuse("cpu.flags entry %r" % (f,))
    if any(a.encode() >= b.encode() for a, b in zip(flags, flags[1:])):
        raise Refuse("cpu.flags out of byte order or given twice")
    i(get(obj, "memory", dict, ""), "total_bytes", "memory")
    d = get(obj, "driver", dict, "")
    s(d, "version", 65536, "driver", r"[0-9]+(\.[0-9]+){1,2}")
    get(d, "loaded", bool, "driver")
    s(get(obj, "cuda_runtime", dict, ""), "version", 65536, "cuda_runtime", r"[0-9]+\.[0-9]+(\.[0-9]+)?")
    cards = get(obj, "cards", list, "")
    if len(cards) > 16:
        raise Refuse("more than 16 cards")
    prev = None
    for n, card in enumerate(cards):
        if i(card, "ordinal", "cards[%d]" % n) != n:
            raise Refuse("cards[%d].ordinal is not its index" % n)
        s(card, "name", 96, "cards[%d]" % n)
        a = s(card, "pci_address", 65536, "cards[%d]" % n, r"[0-9a-f]{4,8}:[0-9a-f]{2}:[0-9a-f]{2}\.[0-7]")
        i(card, "memory_bytes", "cards[%d]" % n)
        key = tuple(int(x, 16) for x in re.split(r"[:.]", a))
        if prev is not None and key <= prev:
            raise Refuse("pci_address values do not strictly ascend")
        prev = key
    drives = get(obj, "drives", list, "")
    if len(drives) != 2:
        raise Refuse("drives has %d entries" % len(drives))
    for drive, (role, mp) in zip(drives, (("state", "/mnt/weaver/state"), ("models", "/mnt/weaver/models"))):
        if get(drive, "role", str, "drive") != role or get(drive, "mount_point", str, "drive") != mp:
            raise Refuse("drives not state then models with their mount points")
        if get(drive, "mounted", bool, "drive"):
            s(drive, "label", 16, role); s(drive, "filesystem", 16, role)
            i(drive, "bytes", role); i(drive, "free_bytes", role)
            get(drive, "project_quota", bool, role)
        elif set(drive) & {"label", "filesystem", "bytes", "free_bytes", "project_quota"}:
            raise Refuse("an unmounted drive carries a mounted drive's members")
    return obj


def refused(raw):
    try:
        reader(raw)
    except Refuse:
        return True
    return False
