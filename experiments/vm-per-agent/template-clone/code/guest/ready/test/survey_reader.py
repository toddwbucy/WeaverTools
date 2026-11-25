"""survey_reader: a reader of weaver-survey/2 and weaver-survey/1, written from the format
document's sections 1, 1.1, 2 and 3 (WeaverTools docs/profiles/weaver-survey-format.md at 4291e118;
version 1 as its section 7 states it, at e16aecd): reader(raw) returns the report's members
or raises Refuse. A reader of N + 1 may also read N (section 4). Standard library only."""
import ipaddress, json, re, time

FORMATS = ("weaver-survey/1", "weaver-survey/2")
V1_PLACES = (("state", "/mnt/weaver/state"), ("models", "/mnt/weaver/models"))
PART = re.compile(r"[A-Za-z0-9._-]{1,255}")


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


class Signed:
    """An integer written with a sign that is not negative, -0: neither an integer nor a
    string, so get() refuses it wherever section 1 names a member, and a member the reader
    does not know keeps it."""

    def __init__(self, written):
        self.written = written


def parse_int(s):
    return Signed(s) if s == "-0" else int(s)


def is_place(v):
    """Section 1.1's one spelling of a place. The characters are judged first, so a lone
    surrogate is refused as a place, not raised."""
    parts = v[1:].split("/") if v.startswith("/") else None
    return not (parts is None or parts == [""]
                or any(p in ("", ".", "..") or not PART.fullmatch(p) for p in parts) or len(v) > 255)


def inside(one, other):
    a, b = one[1:].split("/"), other[1:].split("/")
    return len(a) > len(b) and a[:len(b)] == b


def plane_address(v, where):
    """Section 1.1's one spelling of an address, of a kind it allows, with no zone."""
    if "%" in v:
        raise Refuse("%s %r carries a zone" % (where, v))
    try:
        ip = ipaddress.ip_address(v)
    except ValueError:
        raise Refuse("%s %r is not an address" % (where, v))
    if str(ip) != v:
        raise Refuse("%s %r is not in the one spelling" % (where, v))
    if ip.is_unspecified or ip.is_link_local or ip.is_multicast \
            or (ip.version == 4 and v == "255.255.255.255") or (ip.version == 6 and ip.ipv4_mapped is not None):
        raise Refuse("%s %s is of a refused kind" % (where, v))
    return ip


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
    dec = json.JSONDecoder(object_pairs_hook=no_dup, parse_constant=no_constant, parse_int=parse_int)
    start = len(t) - len(t.lstrip(" \t\r\n"))
    try:
        obj, end = dec.raw_decode(t, start)
    except ValueError as e:
        raise Refuse("not JSON: %s" % e)
    if t[end:] not in ("", "\n"):
        raise Refuse("something after the object: %r" % t[end:end + 20])
    if not isinstance(obj, dict):
        raise Refuse("not one JSON object")
    fmt = obj.get("format")
    if fmt not in FORMATS:
        raise Refuse("format %r" % fmt)
    v2 = fmt == "weaver-survey/2"

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
    if stage not in (("pairing",) if v2 else ("pairing", "drives")):
        raise Refuse("stage %r" % stage)
    taken = get(obj, "taken", str, "")
    try:
        if not re.fullmatch(r"[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z", taken):
            raise ValueError
        # strptime takes the seconds 60 and 61. Section 1 refuses a leap second.
        if int(taken[17:19]) > 59:
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
        a = s(card, "pci_address", 16, "cards[%d]" % n, r"[0-9a-f]{4,8}:[0-9a-f]{2}:[0-9a-f]{2}\.[0-7]")
        i(card, "memory_bytes", "cards[%d]" % n)
        key = tuple(int(x, 16) for x in re.split(r"[:.]", a))
        if prev is not None and key <= prev:
            raise Refuse("pci_address values do not strictly ascend")
        prev = key
    drives = get(obj, "drives", list, "")
    if len(drives) != 2:
        raise Refuse("drives has %d entries" % len(drives))
    for n, (drive, (role, mp)) in enumerate(zip(drives, V1_PLACES)):
        if get(drive, "role", str, "drive") != role:
            raise Refuse("drives not state then models")
        at = get(drive, "mount_point", str, "drive")
        if (v2 and not is_place(at)) or (not v2 and at != mp):
            raise Refuse("drives[%d].mount_point %r is not a place in the one spelling" % (n, at) if v2
                         else "drives not state then models with their mount points")
        if get(drive, "mounted", bool, "drive"):
            s(drive, "label", 16, role); s(drive, "filesystem", 16, role)
            i(drive, "bytes", role); i(drive, "free_bytes", role)
            get(drive, "project_quota", bool, role)
        elif set(drive) & {"label", "filesystem", "bytes", "free_bytes", "project_quota"}:
            raise Refuse("an unmounted drive carries a mounted drive's members")
    if v2:
        a, b = drives[0]["mount_point"], drives[1]["mount_point"]
        if a == b or inside(a, b) or inside(b, a):
            raise Refuse("the two mount points are equal, or one is inside the other")
        net = get(obj, "network", dict, "")
        ends = {}
        for plane in ("management", "data"):
            p = get(net, plane, dict, "network")
            ip = plane_address(s(p, "address", 39, "network." + plane), "network.%s.address" % plane)
            ends[plane] = (ip, i(p, "prefix_length", "network." + plane, 1, 32 if ip.version == 4 else 128))
        planes = get(net, "planes", str, "network")
        if planes not in ("separate", "shared"):
            raise Refuse("network.planes %r" % planes)
        (mi, mn), (di, dn) = ends["management"], ends["data"]
        if planes == "separate" and (mi == di or (mi.version == di.version and (
                mi in ipaddress.ip_network((di, dn), strict=False) or di in ipaddress.ip_network((mi, mn), strict=False)))):
            raise Refuse("separate planes on one address or one network")
    return obj


def refused(raw):
    try:
        reader(raw)
    except Refuse:
        return True
    return False
