import nimpy

proc nim_concat*(iterations: uint16): string {.exportpy.} =
  for number in 0.uint16 .. iterations: result.add $number
