# Purpose

Analyze whether the alloc optimization works. 

# Run

```bash
cargo run
```

# Result

## topsort

tj_scc calls petgraph::algo::tarjan_scc
iter_scc uses the iter version
They all doing topology sort. 

```bash
testing node count: 0
  tj_scc alloc count: (0, 0, 0)
  iter_scc_counts alloc count: (0, 0, 0)
testing node count: 100
  tj_scc alloc count: (104, 10, 103)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 200
  tj_scc alloc count: (204, 12, 203)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 300
  tj_scc alloc count: (304, 14, 303)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 400
  tj_scc alloc count: (404, 14, 403)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 500
  tj_scc alloc count: (504, 14, 503)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 600
  tj_scc alloc count: (604, 16, 603)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 700
  tj_scc alloc count: (704, 16, 703)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 800
  tj_scc alloc count: (804, 16, 803)
  iter_scc_counts alloc count: (3, 0, 2)
testing node count: 900
  tj_scc alloc count: (904, 16, 903)
  iter_scc_counts alloc count: (3, 0, 2)
```

## bevy
bevy_main is bevy's current main branch. bevy_optimized is the optimized version, which use petgraph's iter scc.
alloc count: (alloc, realloc, dealloc)

```bash
testing bevy graph count: 100
  bevy_main alloc count: (1170, 159, 1217)
  bevy_optimized count: (860, 144, 912)
testing bevy graph count: 200
  bevy_main alloc count: (2035, 181, 2242)
  bevy_optimized count: (1430, 163, 1637)
testing bevy graph count: 300
  bevy_main alloc count: (2954, 211, 3264)
  bevy_optimized count: (2049, 190, 2359)
testing bevy graph count: 400
  bevy_main alloc count: (3857, 211, 4270)
  bevy_optimized count: (2652, 190, 3065)
testing bevy graph count: 500
  bevy_main alloc count: (4776, 226, 5293)
  bevy_optimized count: (3271, 205, 3788)
testing bevy graph count: 600
  bevy_main alloc count: (5676, 241, 6296)
  bevy_optimized count: (3871, 217, 4491)
testing bevy graph count: 700
  bevy_main alloc count: (6579, 241, 7302)
  bevy_optimized count: (4474, 217, 5197)
testing bevy graph count: 800
  bevy_main alloc count: (7479, 241, 8305)
  bevy_optimized count: (5074, 217, 5900)
testing bevy graph count: 900
  bevy_main alloc count: (8395, 256, 9325)
  bevy_optimized count: (5690, 232, 6620)
```