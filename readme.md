# geo-datagen

Small rust command-line program, built to generate semi-realistic time-series .csv files for testing purposes.

All timestamps output by system should be in first column and in [RFC3339 format](https://datatracker.ietf.org/doc/html/rfc3339).
All ID values are present in string format, in the form of hyphenated UUIDs.

## Usage (Windows)

Creates 1000 entries for every data generator;
```batch
geo-datagen.exe -e 1000
```

Generate only temperature data using seed 451;
```batch
geo-datagen.exe -t temperature -s 451
```

Set output .csv data suffix to "table" (output files are `windspeed_table.csv`, `temperature_table.csv` etc.);
```batch
geo-datagen.exe table
```

For complete usage guide, run `geo-datagen.exe -h`.

## Libraries

- [chrono](https://docs.rs/chrono/latest/chrono/)
- [glam](https://docs.rs/glam/latest/glam/)
- [rand](https://docs.rs/rand/latest/rand/)
    - [rand-xoshiro](https://docs.rs/rand_xoshiro/latest/rand_xoshiro/)
- [uuid](https://docs.rs/uuid/latest/uuid/)

Adapts [Stefan Gustavason's implementation](https://www.itn.liu.se/~stegu76/aqsis/aqsis-newnoise/simplexnoise1234.cpp) of the [simplex noise algorithm](https://en.wikipedia.org/wiki/Simplex_noise).