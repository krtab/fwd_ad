

## Comparision with other (forward) AD rust libraries

| crate      | version | multi-variate | higher-order | last update |
|------------|--------:|:-------------:|:------------:|------------:|
| **smolad** |   0.1.0 |       ✔️       |      ❌       |  2020-04-23 |
| peroxide   |  0.21.7 |       ❌       | (2nd order)  |  2020-04-21 |
| hyperdual² |   0.3.4 |       ✔️       |      ❌       |  2020-02-08 |
| ad         |   0.1.0 |       ❌       |      ❌       |  2020-01-01 |
| autodiff   |   0.1.9 |       ❌       |      ❌       |  2019-11-07 |
| dual_num   |   0.2.7 |       ❌       |      ❌       |  2019-04-03 |
| descent¹   |     0.3 |       ✔️       | (2nd order?) |  2018-12-10 |
| dual       |   0.2.0 |       ❌       |      ❌       |  2015-12-25 |


1. `descent` Automatic differentiation seems promising but isn't very documented and is mixed-up with the IP-OPT interface
2. `hyperdual` has similar properties to smolad, except that all operations will allocate when `smolad` tries to reuse existing memory

 