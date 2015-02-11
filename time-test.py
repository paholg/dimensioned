#!/usr/bin/env python3
import matplotlib
matplotlib.use("Agg")
import subprocess, re
from pylab import *

pretext = """
extern crate dimensioned;
use dimensioned::u::*;

fn main() {
    let x = unit;

    let y = """
posttext = """;\n
    println!("y: {}", y);
}"""

n = range(1, 9)
time = zeros(len(n))

for i in n:
    midtext = "*".join(["x/x"]*i)
    f = open("examples/time_test.rs", "w")
    f.write(pretext + midtext + posttext)
    f.close()
    cmd = "cargo run --example time_test | time".split()
    out = subprocess.check_output("time cargo run --example time_test", stderr=subprocess.STDOUT, shell=True).decode("utf-8")

    print(out)
    sys = re.search("sys\t([0-9]*)m([0-9]*\.[0-9]*)s", out)
    user = re.search("user\t([0-9]*)m([0-9]*\.[0-9]*)s", out)

    time[i-1] = float(sys.group(1) + user.group(1))*60 + float(sys.group(2)) + float(user.group(2))
    print("i:", i, "time:", time[i-1])

figure(figsize=(5,4))
plot(n, time)
xticks(n)
xlabel("Unit Exponent")
ylabel("Time (s)")
tight_layout()
savefig("time-test.svg")
