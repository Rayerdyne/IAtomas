# Atom drawings

This is a short eplaination of how I draw the incoming atoms.

This is based on the [Atomas Wiki](https://atomas.fandom.com/wiki/Atomas_Wiki) and on some experiment I made. A description is given here after.

## Special atoms

| Atom      | Spawning chance |
| ----      | --------------- |
| Plus      | « *At least every 5 rounds* » → I will use **23%** spawning chance, output of my experiment |
| Minus     | « *Every 20 moves* » → I will use **5%** spawning chance |
| Dark Plus | « *Dark Plus spawn with a **1.25%** (1 in 80) chance when the score is above 750.* »
| Neutrino  | « *Neutrinos spawn with a chance 1/60 when the score is above 1500.* », thus **1.66%** chance. |

## Regular atoms

Deducing the chances of other special atom spawning, we get:
| Score          | Chance of spawning a regular atom |
| -----          | ------   |
| s < 750        | 72%      |
| 750 <= s < 1500 | 70.75%  |
| 1500 <= s       | 69.083% |

The atomic number of the atom is drawn from a shifted Binomial distribution, whose parameters depends on the time step.

The shift and the parameter are computed as follows, where `t` is the current time step (the number of atoms already played):
```
shift(t) = a * t + b = 0.02352571 * t + 2.05071664
n(t)     = c * t + d = 0.02646 * t + 1.5091
p(t)     = e = 0.5
```

## Method

I obtained the numerical value by drawing a sample of atoms (from one game played), then computing the maximum likelihood estimator (MLE). To do so, I defined a log-likelihood function `llikelihood` that computes that likelihood, then minimized it on some coherent domain.

As I didn't obtained a nice result for `b` (about 24, it does not makes sense as it would imply the game to start about that atomic number), I used a linear regression to computes the values for the shift (`a` and `b`), then imposed these values, then recomputed. It turned out that values for `c` and `d` remained the same, whereas `e` became `0`, idk why I'll let it equal to 0.5.

The script that implements this is in the file `estimate.py`.