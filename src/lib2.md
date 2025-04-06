# A Model of Time-Dependent Random Noise

Following is a simple model of time-dependent random noise. While this model can be useful as a motivation for the `bench_diff` approach, the test benchmarks discussed previously provide independent validation of the benchmarking approach used in this library.

**Definitions and assumptions:**

1. Let **ln(x)** be the natural logarithm of **x**.
2. Let **L(f, t)** be the latency of function **f** at time **t**.
3. Let **λ1** be the baseline (ideal) latency of function **f1** in the absence of noise; respectively, **λ2** for **f2**.
4. Given a random variable **χ**, let **E(χ)** and **Stdev(χ)** be the expected value and standard deviation of **χ**, respectively.
5. Assume time-dependent noise is **ν(t) = α(t) * β(t)**, where:
   - **α(t)** is a smooth deterministic function of **t** such that, given a positive duration **Δt** that is no greater than the measured latencies of **f1** and **f2**, then **ln(α(t + Δt)) ≈ ln(α(t))**. Essentially, we are assuming that **α(t)** doesn't fluctuate too wildly.
   - **β(t)** is a random variable dependent on **t**, with a log-normal distribution, such that **E(ln(β(t))) = 0** and **Stdev(ln(β(t))) = σ**, where **σ** is a constant that does not depend on **t**.
6. Assume **L(f1, t) = λ1 * ν(t)** and **L(f2, t) = λ2 * ν(t)** for all **t**.

**Implications:**

1. Substituting *assumption 5* into *assumption 6* for **f1** at time **t** and **f2** at time **t + Δt**, we get:
   - **L(f1, t) = λ1 * α(t) * β(t)**
   - **L(f2, t + Δt) = λ2 * α(t + Δt) * β(t + Δt)**
2. Applying natural logarithms on *implication 1*:
   - **ln(L(f1, t)) = ln(λ1) + ln(α(t)) + ln(β(t))**
   - **ln(L(f2, t + Δt)) = ln(λ2) + ln(α(t + Δt)) + ln(β(t + Δt))**
3. When we measure **f1**'s latency at time **t**, getting **L(f1, t)**, and right after we measure **f2**'s latency, the measurement for **f2** occurs at a time **t + Δt**, where **Δt ≈ L(f1, t)**.
4. By *implication 3* and the first point of *assumption 5*, **ln(α(t + Δt)) ≈ ln(α(t))**.
5. Substituting *implication 4* into the second equation of *implication 2*, we get:
   - **ln(L(f1, t)) = ln(λ1) + ln(α(t)) + ln(β(t))**
   - **ln(L(f2, t + Δt)) ≈ ln(λ2) + ln(α(t)) + ln(β(t + Δt))**
6. Subtracting the second equation from the first equation in *implication 5* from the first equation, we get:
   - **ln(L(f1, t)) - ln(L(f2, t + Δt)) ≈ ln(λ1) - ln(λ2) + ln(β(t)) - ln(β(t + Δt))**
7. Taking the expected values of both sides of *implication 6*, we get:
   - **E(ln(L(f1, t)) - ln(L(f2, t + Δt))) ≈ ln(λ1) - ln(λ2) + E(ln(β(t))) - E(ln(β(t + Δt)))**
8. Simplifying *implication 7*, given the second item of *assumption 5*:
   - **E(ln(L(f1, t)) - ln(L(f2, t + Δt))) ≈ ln(λ1) - ln(λ2) = ln(λ1 / λ2)**
9. Thus, for an execution of `bench_diff` with **f1** and **f2**, the difference between the sample means of the natural logarithms of the observed latencies is an approximately unbiased estimator of **ln(λ1 / λ2)**.

# Limitations

This library works well for latencies at the microseconds or millisecons order of magnitude, but not for latencies at the nanoseconds order of magnitude.

