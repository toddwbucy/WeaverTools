# Commands

How each script in `code/` was run for the measurement of 2026-11-11, in order. Every
script takes `VM_ROOT`, the directory holding the images, the template, the clones,
the logs and the measurement key, and has no default for it. `MODELS_DIR` names the
host directory holding the model artifacts. Both were set on the measuring host to a
dataset of their own; their values are in the deposit, not here. `CPUSET`, where set,
pins the guest's virtual CPUs and QEMU's own threads to those host CPUs; it was set to
the CPUs the Planner left for this work.

Each step's output and exit status land in `$VM_ROOT/logs/<guest>/<step>.txt`, which
the deposit holds. A step rerun after a break was copied aside first as
`<step>-attempt<n>.txt`, so every attempt is kept.

## The network, before any template

```bash
# libvirt's NAT network: DHCP, DNS, ICMP, TCP, HTTP and HTTPS from a throwaway guest
VM_ROOT=... code/net-probe.sh run
VM_ROOT=... code/net-probe.sh destroy

# QEMU user-mode networking, the same probe
NET=user,model=virtio NAME=weaver-nettest-user VM_ROOT=... code/net-probe.sh run
NAME=weaver-nettest-user VM_ROOT=... code/net-probe.sh destroy

# user-mode networking with SSH forwarded from host loopback, and a TCP connect (no
# protocol) from the guest to five ports the host binds to loopback
NET=user-fwd NAME=weaver-nettest-fwd HOST_PORTS="<five loopback ports>" \
  VM_ROOT=... code/net-probe.sh run
# then, by hand in that guest, the guard write_seed now lays at boot (code/lib.sh), and
# the same five connects again
NAME=weaver-nettest-fwd VM_ROOT=... code/net-probe.sh destroy

# whether this QEMU can refuse the guest's path to host loopback from the host side
qemu-system-x86_64 -M none -nodefaults -display none -S \
  -netdev user,id=n0,disable-host-loopback=on          # also disable_host_loopback=on, map-host-loopback=none
qemu-system-x86_64 -M none -display none -netdev passt,id=p0
```

## The template

```bash
git -C <WeaverAgent checkout> bundle create $VM_ROOT/seed/weaveragent.bundle code/unseeded-turn-check

export VM_ROOT=... CPUSET=...
code/build-template.sh create
code/build-template.sh ship $VM_ROOT/seed/weaveragent.bundle
for s in facts packages rustup trees bootstrap-no-toolkit nvidia gpu-state \
         lock-gate bootstrap-default-cuda-dir cccl-in-window; do
  code/build-template.sh step "$s"
done

# the build's two workarounds, named on every command that needed them
G="CUDA_COMPUTE_CAP=86 NVCC_PREPEND_FLAGS=-I/usr/local/cuda-13.3/targets/x86_64-linux/include/cccl"
GUEST_ENV="$G" code/build-template.sh step clean-cuda-crates
GUEST_ENV="$G" code/build-template.sh step bootstrap-plan           # run 1: breaks 4 and 5
GUEST_ENV="$G" code/build-template.sh step bootstrap-plan-relaxed   # runs 2-22, until the test step passed
GUEST_ENV="$G" code/build-template.sh step bootstrap-install-relaxed  # runs 1-2
for s in accounts connectors finish; do code/build-template.sh step "$s"; done
code/build-template.sh seal
```

`bootstrap-no-toolkit` and `bootstrap-default-cuda-dir` are the plan run as found, to
record its refusals (breaks 2 and 3); `lock-gate` records `process/gates/lock.sh` cold
and after `cargo fetch --locked` (break 8); `cccl-in-window` installs `cccl-13-3` and
compiles a probe reading `CCCL_VERSION` with and without the prepended path (break 6).
The `-relaxed` steps set the two test-step settings of breaks 4 and 5 and restore them.

The first attempt stopped at `nvidia`, which installed driver 620.70.06 for
`nvidia-open=615.71.09`; the step was corrected (README, break 1) and the template
rebuilt from the cloud image. Its logs are kept in the deposit as `template-attempt1/`.

The first seal left fwupd's client pair on the disk. The template was booted once more
from its sealed domain definition (the disk made writable for it), looked over, the
seal step extended, and sealed again:

```bash
chmod u+w $VM_ROOT/template/template.qcow2
virsh -c qemu:///system define $VM_ROOT/logs/template/domain-sealed.xml
virsh -c qemu:///system start weaver-template
code/build-template.sh seal
```

## The clone

```bash
export VM_ROOT=... CPUSET=...
MODELS_DIR=... code/clone.sh create
code/clone.sh virtiofsd                         # the share's host side as it runs
code/clone.sh step identity
code/clone.sh step models
code/clone.sh step models-host-refusal          # only once virtiofsd showed --readonly
code/clone.sh step lifecycle-plan vtest1

G="CUDA_COMPUTE_CAP=86 NVCC_PREPEND_FLAGS=-I/usr/local/cuda-13.3/targets/x86_64-linux/include/cccl"
GUEST_ENV="$G" code/clone.sh step lifecycle-relaxed vtest1    # runs 1-2; run 2 is break 12
code/clone.sh step coreutils-probe
code/clone.sh step gnu-coreutils                # Ubuntu's switch: refused by apt, kept as gnu-coreutils-refused
code/clone.sh step gnu-test
code/clone.sh step lifecycle-cleanup vtest1
GUEST_ENV="$G" code/clone.sh step lifecycle-relaxed vtest1    # runs 3-4; run 4 is break 13
code/clone.sh step lifecycle-cleanup vtest1
code/clone.sh step sudo-ws
GUEST_ENV="$G" code/clone.sh step lifecycle-relaxed vtest1    # runs 5-6; run 6 reached the load
code/clone.sh step lifecycle-cleanup vtest1

code/clone.sh step create-plan vmagent1
code/clone.sh step create-apply vmagent1
code/clone.sh step validate vmagent1
code/clone.sh step load vmagent1

GUEST_ENV="$G" code/clone.sh step test-step     # the test step alone, rerun with the host's load recorded

# the host's loopback over IPv4 and IPv6, against each backend option
P="<the five loopback ports>"                   # as in the network probe above
code/clone.sh step loopback-probe $P            # as built: IPv6 on, the IPv4 guard
#   then, in the guest: sudo nft add rule inet hostguard out ip6 daddr fec0::2 counter drop
code/clone.sh step loopback-probe $P
code/clone.sh netdev 'user,id=un0,ipv6=off,hostfwd=tcp:127.0.0.1:2231-:22'
code/clone.sh step loopback-probe $P
# a temporary listener on host loopback as the guestfwd endpoint, stopped after
python3 -I -m http.server 18080 --bind 127.0.0.1 &
code/clone.sh netdev 'user,id=un0,ipv6=off,restrict=on,hostfwd=tcp:127.0.0.1:2231-:22,guestfwd=tcp:10.0.2.100:8080-tcp:127.0.0.1:18080'
#   then, in the guest: sudo nft delete table inet hostguard
code/clone.sh step loopback-probe $P
code/clone.sh netdev 'user,id=un0,hostfwd=tcp:127.0.0.1:2231-:22'   # back as created
```

The first three netdev settings were made with a helper doing what `clone.sh netdev`
does, which was then added to `clone.sh`; the last, setting the clone back, was made
with `clone.sh netdev` itself.

The lifecycle runs were driven by a loop that reran `lifecycle-relaxed` while it failed
only at step 0's test step and stopped at anything else, recording each run's start,
the host's load average and its outcome; the test-only runs by the same kind of loop.
`gnu-coreutils` was the step as first written (Ubuntu's package switch), which apt's
solver refused without removing `coreutils-from-uutils`; the step was then rewritten as
`gnu-test`, and `code/guest/clone-steps` carries the rewritten one.
