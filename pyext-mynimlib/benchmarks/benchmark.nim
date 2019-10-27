import times, ../src/libmynimlib

let t = cpuTime()
discard nim_concat(1000.uint16)
echo(cpuTime() - t, " Secs")
