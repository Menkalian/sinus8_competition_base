# Benchmarkergebnisse

## 1. Reiner Port von Mertens impl.

````
     Running benches\sinus8_benchmark.rs (target\release\deps\sinus8_benchmark-48531b2b3ad54c2e.exe)
Benchmarking sinus8 8 Bytes
Benchmarking sinus8 8 Bytes: Warming up for 3.0000 s
Benchmarking sinus8 8 Bytes: Collecting 100 samples in estimated 5.0001 s (72M iterations)
Benchmarking sinus8 8 Bytes: Analyzing
sinus8 8 Bytes          time:   [67.299 ns 67.762 ns 68.270 ns]
                        change: [+204.88% +207.92% +210.82%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking sinus8 readable ascii
Benchmarking sinus8 readable ascii: Warming up for 3.0000 s
Benchmarking sinus8 readable ascii: Collecting 100 samples in estimated 5.0005 s (5.1M iterations)
Benchmarking sinus8 readable ascii: Analyzing
sinus8 readable ascii   time:   [942.96 ns 946.98 ns 951.05 ns]
                        change: [+247.60% +252.86% +256.88%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 10 outliers among 100 measurements (10.00%)
  2 (2.00%) low mild
  6 (6.00%) high mild
  2 (2.00%) high severe

Benchmarking sinus8 large data
Benchmarking sinus8 large data: Warming up for 3.0000 s

Warning: Unable to complete 100 samples in 5.0s. You may wish to increase target time to 13.2s, or reduce sample count to 30.
Benchmarking sinus8 large data: Collecting 100 samples in estimated 13.227 s (100 iterations)
Benchmarking sinus8 large data: Analyzing
sinus8 large data       time:   [130.56 ms 130.93 ms 131.31 ms]
                        change: [+226.24% +227.90% +229.56%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 2 outliers among 100 measurements (2.00%)
  2 (2.00%) high mild
````

## 2. Umsetzung von Sinus über eine LUT

````
Benchmarking sinus8 8 Bytes
Benchmarking sinus8 8 Bytes: Warming up for 3.0000 s
Benchmarking sinus8 8 Bytes: Collecting 100 samples in estimated 5.0000 s (230M iterations)
Benchmarking sinus8 8 Bytes: Analyzing
sinus8 8 Bytes          time:   [21.493 ns 21.627 ns 21.774 ns]
                        change: [-68.243% -67.985% -67.725%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 5 outliers among 100 measurements (5.00%)
  2 (2.00%) high mild
  3 (3.00%) high severe

Benchmarking sinus8 readable ascii
Benchmarking sinus8 readable ascii: Warming up for 3.0000 s
Benchmarking sinus8 readable ascii: Collecting 100 samples in estimated 5.0005 s (19M iterations)
Benchmarking sinus8 readable ascii: Analyzing
sinus8 readable ascii   time:   [262.69 ns 264.33 ns 266.10 ns]
                        change: [-72.350% -72.168% -71.983%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 7 outliers among 100 measurements (7.00%)
  5 (5.00%) high mild
  2 (2.00%) high severe

Benchmarking sinus8 large data
Benchmarking sinus8 large data: Warming up for 3.0000 s
Benchmarking sinus8 large data: Collecting 100 samples in estimated 8.1591 s (200 iterations)
Benchmarking sinus8 large data: Analyzing
sinus8 large data       time:   [40.362 ms 40.576 ms 40.797 ms]
                        change: [-69.206% -69.010% -68.788%] (p = 0.00 < 0.05)
                        Performance has improved.
Found 1 outliers among 100 measurements (1.00%)
  1 (1.00%) high mild
````

## 3. Rewrite des Algorithmus mit LUT
````
Benchmarking sinus8 8 Bytes
Benchmarking sinus8 8 Bytes: Warming up for 3.0000 s
Benchmarking sinus8 8 Bytes: Collecting 100 samples in estimated 5.0000 s (514M iterations)
Benchmarking sinus8 8 Bytes: Analyzing
sinus8 8 Bytes          time:   [9.8134 ns 10.008 ns 10.224 ns]
                        change: [+0.9337% +3.1595% +5.3731%] (p = 0.00 < 0.05)
                        Change within noise threshold.
Found 4 outliers among 100 measurements (4.00%)
  3 (3.00%) high mild
  1 (1.00%) high severe

Benchmarking sinus8 readable ascii
Benchmarking sinus8 readable ascii: Warming up for 3.0000 s
Benchmarking sinus8 readable ascii: Collecting 100 samples in estimated 5.0009 s (21M iterations)
Benchmarking sinus8 readable ascii: Analyzing
sinus8 readable ascii   time:   [247.29 ns 250.30 ns 253.65 ns]
                        change: [+1.0068% +2.2076% +3.4034%] (p = 0.00 < 0.05)
                        Performance has regressed.
Found 6 outliers among 100 measurements (6.00%)
  5 (5.00%) high mild
  1 (1.00%) high severe

Benchmarking sinus8 large data
Benchmarking sinus8 large data: Warming up for 3.0000 s
Benchmarking sinus8 large data: Collecting 100 samples in estimated 8.6048 s (200 iterations)
Benchmarking sinus8 large data: Analyzing
sinus8 large data       time:   [42.372 ms 42.540 ms 42.725 ms]
                        change: [-0.4846% +0.0689% +0.7025%] (p = 0.82 > 0.05)
                        No change in performance detected.
Found 8 outliers among 100 measurements (8.00%)
  4 (4.00%) high mild
  4 (4.00%) high severe

````

## 4. Verwendung von i32 als Wertebereich
````
Benchmarking sinus8 8 Bytes
Benchmarking sinus8 8 Bytes: Warming up for 3.0000 s
Benchmarking sinus8 8 Bytes: Collecting 100 samples in estimated 5.0001 s (344M iterations)
Benchmarking sinus8 8 Bytes: Analyzing
sinus8 8 Bytes          time:   [14.460 ns 14.498 ns 14.540 ns]
Found 7 outliers among 100 measurements (7.00%)
  3 (3.00%) low mild
  2 (2.00%) high mild
  2 (2.00%) high severe

Benchmarking sinus8 readable ascii
Benchmarking sinus8 readable ascii: Warming up for 3.0000 s
Benchmarking sinus8 readable ascii: Collecting 100 samples in estimated 5.0002 s (100M iterations)
Benchmarking sinus8 readable ascii: Analyzing
sinus8 readable ascii   time:   [50.838 ns 51.100 ns 51.399 ns]
Found 16 outliers among 100 measurements (16.00%)
  8 (8.00%) high mild
  8 (8.00%) high severe

Benchmarking sinus8 large data
Benchmarking sinus8 large data: Warming up for 3.0000 s
Benchmarking sinus8 large data: Collecting 100 samples in estimated 5.3666 s (1200 iterations)
Benchmarking sinus8 large data: Analyzing
sinus8 large data       time:   [4.3783 ms 4.3969 ms 4.4187 ms]
Found 7 outliers among 100 measurements (7.00%)
  4 (4.00%) high mild
  3 (3.00%) high severe
````