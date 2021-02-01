"""
The atoms are assumed to be drawn from a Binomial distribution, that evolves
lineraly over time, plus a treshold, also linear.
That is:
    X_t ~ s(t) + Bin(n(t), p(t))
where:
    - X_t is the random variable corresponding to the atom drawn at step t
    - s(t) is the treshold
    - n(t) is some linear function of t
    - p(t) is some linear function of t

We have: 
    s(t) = a * t + b
    n(t) = c * t + d
    p(t) = f * t + e

As we expect p not to increase above 1 or under 0, we set e = 0, thus
    p(t) = e
"""

from scipy.special import gamma, factorial, binom
from scipy.optimize import minimize, Bounds
import numpy as np

# Yes, somewhat inaccurate but it will be fine
INFINITY = 10000

data = [2, 2, -1, 0, 2, 1, 1, 2, 1, 0, 2, 3, 2, 2, 2, 0, 2, 3, 3, 3, 3, 0, -1,
        2, 2, 1, 0, 3, 2, 3, 1, 0, 0, 3, 0, 3, 3, 3, 3, 0, 3, 4, 4, 0, 3, 4, 
        4, 0, 4, 4, 4, 4, 0, -1, 3, 2, 0, 0, 4, 0, 4, 2, 4, 2, 3, 0, 0, 2, 2, 
        4, 0, 3, 0, 4, 4, 4, 5, 0, 5, 5, 5, 0, -1, 4, 4, 0, 5, 5, 5, 0, 3, 0,
        5, 5, 0, 3, -1, 3, 3, 4, 0, 4, 4, 5, 0, 5, 5, 4, 5, 0, 5, 5, 5, 6, 0,
        -1, 6, 6, 0, 4, 4, 0, 5, 6, 5, 4, 5, 0, 6, 7, 5, 7, 6, 0, 0, -1, 0, 
        3, 6, 6, 3, 3, 0, 7, -2, 7, 5, 7, 4, 7, 0, 4, 6, 8, 8, -1, 0, 8, 6, 8,
        6, 8, -2, 5, 5, 7, 5, 5, 0, 7, 8, 7, 6, 7, 0, -1, 0, 8, 7, 5, 5, 5, 0,
        7, 6, 0, 7, 0, 5, 0, 5, 6, 0, 6, 7, -1, 7, 8, 0, 7, 5, 7, 8, 7, 0, 6, 
        7, 9, 0, 0, 8, 6, 8, 0, 7, -1, 7, 8, 6, 0, 0, 7, 6, 9, 0, 7, 9, 6, 7,
        0, 9, 7, 6, 9, 6, -1, 0, 9, 9, 8, 7, 7, 0, 8, 7, 7, 8, 7, 0, 7, 7, 8, 
        10, 0, 9, -1, 8, 10, 6, 0, 7, 6, 8, 7, 6, 0, 0, 7, 10, 7, 7, 7, 0, 9,
        8, -1, 10, 0, 7, 0, 10, 8, 7, 10, 0, 9, 0, -2, 9, 8, 11, 0, 9, 7, -1, 
        9, 10, 0, 9, 11, 10, 0, 0, 8, 0, 8, 9, 8, 10, -2, 9, 10, 0, 8, -1, 0, 
        11, 8, 10, 9, 8, 0, 10, 10, 11, 0, 11, 10, 10, 10, 0, 9, 12, 0, -1, 
        10, 9, 12, 0, 12, 11, 9, 11, 0, 9, 12, 0, 12, 11, 12, 10, 0, 11, 10,
        -1, 11, 10, 0, 11, 11, 11, 0, 9, 9, 11, 10, 12, 9, 0, 9, 12, 12, 9, 0,
        0, -1, 10, 11, 13, 0, 10, 0, 13, 11, 0, 0, 0, 10, 11, 11, 12, 0, 12, 
        0, 12, -1, 12, 0, 11, 10, 12, 12, 13, 0, 12, 0, 9, 10, 13, 13, 0, 13,
        12, 12, 12, 10, -1, 0, 0, 10, 0, 11, 11, 11, 14, 11, 0, 0, 0, 12, 11,
        11, 13, 11, 0, 13, -1, 11, 13, -2, 13, 11, 14, -3, 11, 0, 12, 0, 11, 
        12, 0, 11, 12, 11, 0, 11, -1, 11, 13, 0, 12, 14, 13, 10, 12, 0, 13, 0,
        14, 12, 12, 13, 12, 0, 15, 14, -1, 14, 15, 0, 13, 13, 0, 12, 0, 0, 15,
        15, 0, 0, 0, 0, 12, 0, 12, 0, -1, 12, 12, 12, 0, 13, 0, 13, 12, 15,
        14, 12, 0, 13, 13, 0, 12, 13, 15, 14, -1, 0, 15, 16, 13, 0, 0, 16, 0,
        14, 14, 16, 15, 13, 0, 14, 16, 16, 16, 13, -1, 0, 13, 13, 13, 0, 16,
        0, 13, 13, 15, 17, 14, 0, 13, 15, 14, 0, 17, 17, -1, 0, 16, 15, 15,
        16, 14, 0, 16, 18, 16, 18, 18, 0, 18, 16, 0, 16, 16, 15, -1, 17, 0,
        18, 15, 17, 14, -3, 0, 17, 18, 0, 18, -3, -3, 17, 15, 0, 14, 15, -1,
        15, 18, 0, 17, 18, 16, 18, 16, 0, 19, 18, 17, 18, 0, 17, 18, 19, 19,
        18, -1, 0, 0, 16, 0, 16, 18, 0, 16, 17, 0, 19, 16, 17, 17, 16, 0, 19,
        16, 18, -1, 15, 0, 17, 15, 0, 18, 17, 16, 17, 15, 0, 0, 16, 17, 18, 0,
        20, 18, 16, -1, 16, 0, 20, 0, 18, 19, 18, 18, 0, 18, 20, 19, 19, 0, 
        20, 17, 19, 18, 17, -1, 0, 19, 18, 16, 18, 18, 0, -3, 18, 16, 17, 16,
        0, 20, 20, 17, 17, 18, 0, -1, 18, 20, 21, 21, 0, 21, 21, 21, 0, 21,
        20, 21, 20, 20, 0, 20, 17, 20, 20, -1, 0, 17, 21, 21, 20, 19, 0, 21,
        0, 20, 0, -3, 2, 18, 20, 18, 0, 0, 17, -1, 0, 18, 0, 21, 21, 17, 0,
        21, 20, -2, 18, 18, 0, 0, 21, 0, 21, 19, 0, -1, 18, 21, -2, 18, 19, 0,
        20, -2, 22, 21, 19, 21, 19, 0, 0, 18, 22, 1, 21, -1, 0, 22, 20, 20, 22,
        20, 0, 20, -3, 0, 20, 19, 0, 19, 19, 22, 23, 20, 0, -1, 0, 20, 22, 0,
        20, 21, 20, 23, 20, 0, 21, 21, 21, 19, 20, 0, 21, 21, 22, -1, 20, 0, 
        2, 20, 21, 21, 0, 20, 23, 0, 0, 21, 23, 23, 24, 0, 23, 20, 23, -1, 21,
        0, 24, 24, 23, 0, 21, 21, 20, 23, 23, 0, 0, 24, 24, 24, 24, 21, 0, -1,
        21, 22, 22, 23, 0, 24, 20, 0, 24, 0, 24, 22, 20, 22, 21, 0, 20, 0, 0,
        -1, 24, 0, 21, 24, 22, 21, 23, 0, 20, 20, 24, 22, 24, 0, 25, 23, 23, 
        2, 25, -1, 0, 25, 25, 24, 22, 22, 0, 23, 23, 22, 21, 0, 23, 0, -3, 25,
        21, 22, 24, -1, 0, 25, 26, 25, 24, -3, 26, 25, 26, 25, 25, 0, 25, 25,
        24, 24, 24, 0, 24, -1, 0, 26, 0, 22, 27, 22, 26, -3, 0, 27, 0, 0, 24,
        25, 25, 27, 0, 0, 27, -1, 26, 20, 0, 23, 23, 23, 23, 26, 0, 28, 27, 0,
        24, 24, 28, 25, 28, 0, 25, -1, 0, 28, 26, -3, 23, -3, 0, 27, 28, 26, 
        0, 27, 0, 28, 0, 25, 26, 26, 23, -1, 0, 24, 25, 25, -3, 24, 0, 23, 24,
        0, 27, 24, 0, 29, 0, 24, 25, 27, 0, -1, 27, 29, 25, 29, 0, 26, 24, 28,
        0, 0, 24, 29, 26, 0, 26, 24, 25, 26, 0, -1, 27, 28, 28, 27, 0, 0, 28,
        26, 26, 24, 0, 29, 26, 26, 25, 24, 0, 27, 30, -1, 0, 28, 26, 27, 0,
        30, 25, 29, 0, 28, 28, 27, 25, 30, 0, 25, 28, 28, 26, -1, 0, 29, 0,
        28, 30, 30, 29, 26, 0, 29, 28, 29, 27 ]

"""
Real generalization of nchoosek.
n: real
k: real
"""
# def nchoosek2(n, k):
#     if n - k + 1 < 0:
#         # print("ouin (", n, ", ", k, ")")
#         return 0
#     a = gamma(n + 1)
#     b = gamma(k + 1)
#     c = gamma(n - k + 1)
#     return a / (b * c)
def nchoosek2(n, k):
    return binom(round(n), round(k))

"""
x: a vector containing the data

theta: the vector of parameters of the model
theta = [a, b, c, d, e]

l=\sum [ D_t * ( log(e) + log(1 - e)) + log(nchoosek(n(t), D_t)) ]
"""
def llikelihood(theta, x):
    l = 0
    (a, b, c, d, e) = theta
    t_1 = np.log(e) * np.log(1 - e) if e != 0 else -INFINITY
    for t in range(len(x)):
        D_t = abs(x[t] - a * t - b)
        m = nchoosek2(c * t + d, D_t)
        # if m < 0:
            # print(c * t + d, "  ", D_t, "  -> ", m)
        if m == 0:
            l = l - INFINITY
            # print("bim")
            continue
        l = l + D_t * t_1 + np.log(m)
    
    return l

def optimize(i, theta0, tol):
    prev = 0.0001
    curr = 0.8
    next = 0.9999
    theta = theta0.copy()
    while abs(prev - curr) > tol or abs(curr - next) > tol:
        theta[i] = prev
        l_prev = llikelihood(data, theta)
        theta[i] = curr
        l_curr = llikelihood(data, theta)
        theta[i] = next
        l_next = llikelihood(data, theta)
        if l_prev < l_curr and l_curr < l_next:
            tmp = 3 * curr - 2 * next
            prev = curr
            curr = next
            next = tmp
            print("up")
        elif l_prev > l_curr and l_curr > l_next:
            tmp = 3 * prev - 2 * curr
            if tmp < 0:
                tmp = 1e-5
            next = curr
            curr = prev
            prev = tmp
            print("down")
        else:
            prev = curr + 0.5 * (prev - curr)
            next = curr + 0.5 * (next - curr)
        
        print(f"({prev:.5}, {curr:.5}, {next:.5}) ->", end="")
        print(f"({l_prev:.5}, {l_curr:.5}, {l_next:.5})")
    print(f"{i}--> {curr}")
    return theta

def gd(theta0, lr, lr2, tol, dx):
    theta = theta0
    dtheta = [1, 1, 1, 1, 1]
    while sum(np.square(dtheta)) > tol:
        print(theta)
        l0 = llikelihood(data, theta)
        for i in range(len(theta)):
            thetap = theta.copy()
            thetap[i] = thetap[i] + dx
            dtheta[i] = llikelihood(data, thetap) - l0
        
        for i in range(len(theta)):
            theta[i] += dtheta[i] * lr[i] * lr2
            if theta[i] < 0:
                theta[i] = 0
    return theta

    # s(t) = a * t + b
    # n(t) = c * t + d
    # p(t) = e

x_0 = [0.01251, 0.9945, 0.02646, 1.5091, 0.4999]
lb = [0,    0, 0,  0, 0]
ub = [1, 1000, 1, 10, 1]
bounds = Bounds(lb, ub)

y = minimize(fun=llikelihood,
             x0=x_0,
             args=(data),
             method='TNC',
             bounds=bounds)
print(f"->{y}")