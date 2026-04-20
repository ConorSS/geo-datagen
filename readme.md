# geo-datagen

Small rust command-line program, built to generate semi-realistic time-series .csv files for testing purposes.

## Libraries

- [chrono](https://docs.rs/chrono/latest/chrono/)
- [glam](https://docs.rs/glam/latest/glam/)
- [rand](https://docs.rs/rand/latest/rand/)
    - [rand-xoshiro](https://docs.rs/rand_xoshiro/latest/rand_xoshiro/)

Adapts [Stefan Gustavason's implementation](https://www.itn.liu.se/~stegu76/aqsis/aqsis-newnoise/simplexnoise1234.cpp) of the [simplex noise algorithm](https://en.wikipedia.org/wiki/Simplex_noise).