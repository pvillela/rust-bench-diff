# A Model of Time-Dependent Random Noise

Following is a simple model of time-dependent random noise. While this model can be useful as a motivation for the `bench_diff` approach, the test benchmarks discussed previously provide independent validation of the benchmarking approach used in this library.

## The Model

**Definitions and assumptions**

1. Let **ln(x)** be the natural logarithm of **x**.
2. Let **L(f, t)** be the latency of function **f** at time **t**.
3. Let **λ1** be the baseline (ideal) latency of function **f1** in the absence of noise; respectively, **λ2** for **f2**.
4. Given a random variable **χ**, let **E(χ)** and **Stdev(χ)** be the expected value and standard deviation of **χ**, respectively.
5. Assume time-dependent noise is **ν(t) = α(t) * β(t)**, where:
   - **α(t)** is a smooth deterministic function of **t**, such that there are positive constants **A<sub>L</sub>** and **A<sub>D</sub>** for which **A<sub>L</sub> ≤ α(t)** and **|α'(t)| ≤ A<sub>D</sub>**, for all **t**.
   - **β(t)** is a family of mutually independent log-normal random variables indexed by **t**, such that  **E(ln(β(t))) = 0** and **Stdev(ln(β(t))) = σ**, where **σ** is a constant that does not depend on **t**.

6. Assume **L(f1, t) = λ1 * ν(t)** and **L(f2, t) = λ2 * ν(t)** for all **t**.

**Implications**

1. When we measure **f1**'s latency at a time **t<sub>1</sub>**, getting **L(f1, t<sub>1</sub>)**, and right after we measure **f2**'s latency, the measurement for **f2** occurs at a time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**, where **Δt<sub>1</sub>** is <u>very close</u> to **L(f1, t<sub>1</sub>)**.

2. Substituting *assumption 5* into *assumption 6* for **f1** at time **t<sub>1</sub>** and **f2** at time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**:

   - L(f1, t<sub>1</sub>) = λ1 * α(t<sub>1</sub>) * β(t<sub>1</sub>)
   - L(f2, t<sub>2</sub>) = λ2 * α(t<sub>1</sub> + Δt<sub>1</sub>) * β(t<sub>2</sub>)

3. Applying natural logarithms on *implication 2*:

   - ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))
   - ln(L(f2, t<sub>2</sub>)) = ln(λ2) + ln(α(t<sub>1</sub> + Δt<sub>1</sub>)) + ln(β(t<sub>2</sub>))

4. Based on the bound on **α'(t)** from *assumption 5*:

   - ln(α(t<sub>1</sub> + Δt<sub>1</sub>))  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>p</sub>)  **[** _for some t<sub>p</sub> between t<sub>1</sub> and Δt<sub>1</sub>_ **]**  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * γ(t<sub>1</sub>)/α(t<sub>p</sub>)  **[** _where γ(t<sub>1</sub>) =<sub>def</sub> α'(t<sub>p</sub>), and thus |γ(t<sub>1</sub>)| ≤ A<sub>D</sub>_ **]**

   Based on the bounds for **α(t)** from *assumption 5*:

   - 1 / α(t<sub>p</sub>) - 1 / α(t<sub>1</sub>)  

     = -α'(t<sub>q</sub>) / α(t<sub>1</sub>)^2  **[** _for some t<sub>q</sub> between t<sub>1</sub> and t<sub>p</sub>_ **]**  

     = δ<sub>0</sub>(t<sub>1</sub>) / α(t<sub>1</sub>)  **[** _where δ<sub>0</sub>(t<sub>1</sub>) =<sub>def</sub>  -α'(t<sub>q</sub>)/α(t<sub>1</sub>); thus |δ<sub>0</sub>(t<sub>1</sub>)| ≤ A<sub>D</sub>/A<sub>L</sub>_ **]**  

     Thus:  

     1 / α(t<sub>p</sub>)  

     = 1 / α(t<sub>1</sub>) + δ<sub>0</sub>(t<sub>1</sub>) / α(t<sub>1</sub>)  

     = (1 + δ<sub>0</sub>(t<sub>1</sub>)) / α(t<sub>1</sub>)  

     = δ(t<sub>1</sub>) / α(t<sub>1</sub>)  **[** _where δ(t<sub>1</sub>) =<sub>def</sub> (1 + δ<sub>0</sub>(t<sub>1</sub>)); thus 1 - A<sub>D</sub>/A<sub>L</sub> ≤ δ(t<sub>1</sub>) ≤ 1 + A<sub>D</sub>/A<sub>L</sub> ⇒ |δ(t<sub>1</sub>)| ≤ 1 + A<sub>D</sub>/A<sub>L</sub>_ **]**

   Therefore, from the first equation in this implication group and the above equation:

   - ln(α(t<sub>1</sub> + Δt<sub>1</sub>))  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * γ(t<sub>1</sub>) * δ(t<sub>1</sub>) / α(t<sub>1</sub>)  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>) / α(t<sub>1</sub>)  **[** _where ε<sub>1</sub>(t<sub>1</sub>) =<sub>def</sub> γ(t<sub>1</sub>)\*δ(t<sub>1</sub>), ε<sub>1</sub><sup>U</sup> =<sub>def</sub> A<sub>D</sub>\*(1+A<sub>D</sub>/A<sub>L</sub>); thus |ε<sub>1</sub>(t<sub>1</sub>)| ≤  ε<sub>1</sub><sup>U</sup>_ **]**

5. Applying *implication 4* to *implication 3*:

   - ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))

   - ln(L(f2, t<sub>2</sub>))  

     = ln(λ2) + ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))   **[** _where |ε<sub>1</sub>(t<sub>1</sub>)| ≤ ε<sub>1</sub><sup>U</sup>_ **]**  

     Using *implication 1* to replace the Δt<sub>1</sub> value above:  

     = ln(L(f2, t<sub>2</sub>)) = ln(λ2) + ln(α(t<sub>1</sub>)) + L(f1, t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))

6. Using the first equation in *implication 2* and the formula for the expected value of a log-normal distribution:

   - E(L(f1, t<sub>1</sub>))  

     = λ1 * α(t<sub>1</sub>) * E(β(t<sub>1</sub>))  

     = λ1 * α(t<sub>1</sub>) * exp(σ<sup>2</sup>/2)

7. Taking expected values in *implication 5*:

   - E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>)) + E(ln(β(t<sub>1</sub>)))
   - E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>)) / α(t<sub>1</sub>) + E(ln(β(t<sub>2</sub>)))

8. Using the second item in *assumption 5*, *implication 7* simplifies to:

   - E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>))
   - E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>)) / α(t<sub>1</sub>)  

9. Defining **ξ<sub>1</sub>(t<sub>1</sub>) =<sub>def</sub> E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>)) / α(t<sub>1</sub>)** and by standard expected value inequalities:

   - |ξ<sub>1</sub>(t<sub>1</sub>)|  

     = |E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>))| / α(t<sub>1</sub>)  

     ≤ E(|ε<sub>1</sub>(t<sub>1</sub>)| * |L(f1, t<sub>1</sub>)|) / α(t<sub>1</sub>)  

     ≤ E(|ε<sub>1</sub>(t<sub>1</sub>)|) * E(|L(f1, t<sub>1</sub>)|) / α(t<sub>1</sub>)  

     = E(|ε<sub>1</sub>(t<sub>1</sub>)|) * E(L(f1, t<sub>1</sub>)) / α(t<sub>1</sub>)  

     Using the  bound|ε<sub>1</sub>(t<sub>1</sub>)| ≤ ε<sub>1</sub><sup>U</sup> from *implication 4*:  

     ≤ ε<sub>1</sub><sup>U</sup> * E(L(f1, t<sub>1</sub>)) / α(t<sub>1</sub>)  

     By *implication 6*:
     
     = ε<sub>1</sub><sup>U</sup> * λ1 * α(t<sub>1</sub>) * exp(σ<sup>2</sup>/2) / α(t<sub>1</sub>)  
     
     = ε<sub>1</sub><sup>U</sup> * λ1 * exp(σ<sup>2</sup>/2)

10. Using *implication 8* and substituting *implication 9* into the second equation:

       - E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>))
       - E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + ξ<sub>1</sub>(t<sub>1</sub>)  **[** _where |ξ<sub>1</sub>(t<sub>1</sub>)| ≤ ε<sub>1</sub><sup>U</sup> * λ1 * exp(σ<sup>2</sup>/2)_ **]**

11. Subtracting the second equation from the first in *implication 10* and using the linearity of **E()**:

    - E(ln(L(f1, t<sub>1</sub>) - ln(L(f2, t<sub>2</sub>))))  

      = ln(λ1) - ln(λ2) - ξ<sub>1</sub>(t<sub>1</sub>)  

      = ln(λ1 / λ2) - ξ<sub>1</sub>(t<sub>1</sub>)  **[** _where |ξ<sub>1</sub>(t<sub>1</sub>)| ≤ ε<sub>1</sub><sup>U</sup> * λ1 * exp(σ<sup>2</sup>/2)_ **]**

12. With `bench_diff`, measurements are done pairs, with one half of the pairs having **f1** followed by **f2** and the other half having **f2** followed by **f1**. The equation in *implication 11* above pertains to the first case. The analogous equation for the second case is:  

    - E(ln(L(f2, t<sub>2'</sub>) - ln(L(f1, t<sub>1'</sub>)))) = ln(λ2 / λ1) - ξ<sub>2</sub>(t<sub>2'</sub>)  

    Or, equivalently:

    - E(ln(L(f1, t<sub>1'</sub>)) - ln(L(f2, t<sub>2'</sub>))) = ln(λ1 / λ2) + ξ<sub>2</sub>(t<sub>2'</sub>)  **[** _where |ξ<sub>2</sub>(t<sub>2'</sub>)| ≤ ε<sub>2</sub><sup>U</sup> * λ2 * exp(σ<sup>2</sup>/2)_ **]**

13. Assuming the number of latency observations for each function is **n** and considering the two cases as described in *implication 12*, we can calculate the sample mean difference between the natural logarithms of the observed latencies:
    - mean_diff_ln  
      
      =<sub>def</sub> (1/n) * ∑<sub>i=1..n</sub> (ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>))))  
      
      = (1/n) * (∑<sub>i:odd</sub> (ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)))) + ∑<sub>i:even</sub> (ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)))))

14. Taking expected values in *implication 13* and using the linearity of **E()**:

    - E(mean_diff_ln)  

      = (1/n) * (∑<sub>i:odd</sub> E(ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)))) + ∑<sub>i:even</sub> E(ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)))))  

      By *implication 11* and *implication 12*:

      = (1/n) * (∑<sub>i:odd</sub> (ln(λ1 / λ2) - ξ<sub>1</sub>(t<sub>1,i</sub>)) + ∑<sub>i:even</sub> (ln(λ1 / λ2) + ξ<sub>2</sub>(t<sub>2,i</sub>)))  

      = ln(λ1 / λ2) + (1/n) * ∑<sub>i:odd</sub> (ξ<sub>2</sub>(t<sub>2,i+1</sub>) - ξ<sub>1</sub>(t<sub>1,i</sub>))  

      **[** _where |ξ<sub>1</sub>(t<sub>1</sub>)| ≤ ε<sub>1</sub><sup>U</sup> * λ1 * exp(σ<sup>2</sup>/2) and |ξ<sub>2</sub>(t<sub>2'</sub>)| ≤ ε<sub>2</sub><sup>U</sup> * λ2 * exp(σ<sup>2</sup>/2)_ **]**

15. The equation in *implication 14* shows that, with `bench_diff`, the difference between the sample means of the natural logarithms of the observed latencies is a biased estimator of **ln(λ1 / λ2)**, with a bias of:

    - (1/n) * ∑<sub>i:odd</sub> (ξ<sub>2</sub>(t<sub>2,i+1</sub>) - ξ<sub>1</sub>(t<sub>1,i</sub>))  

      ≤ (1/n) * ∑<sub>i:odd</sub> (|ξ<sub>2</sub>(t<sub>2,i+1</sub>)| + |ξ<sub>1</sub>(t<sub>1,i</sub>)|)  

      ≤ (1/n) * ∑<sub>i:odd</sub> ((ε<sub>2</sub><sup>U</sup> * λ2 * exp(σ<sup>2</sup>/2)) + (ε<sub>1</sub><sup>U</sup> * λ1 * exp(σ<sup>2</sup>/2)))  

      = (1/2) * (ε<sub>2</sub><sup>U</sup> * λ2 + ε<sub>1</sub><sup>U</sup> * λ1) * exp(σ<sup>2</sup>/2)  

      = (1/2) * (A<sub>D</sub>\*(1 + A<sub>D</sub>/A<sub>L</sub>)\*λ2 + A<sub>D</sub>\*(1 + A<sub>D</sub>/A<sub>L</sub>)\*λ1) * exp(σ<sup>2</sup>/2)
      
      = A<sub>D</sub> * (1 +  A<sub>D</sub>/A<sub>L</sub>) * (λ1+λ2)/2 * exp(σ<sup>2</sup>/2)
    
16. Thus, assuming the above product is sufficiently small, the estimates of the ratio of latency medians produced by `bench_diff` should be sufficiently accurate. Notice that while the variability of the statistic mean_diff_ln varies with the sample size (exec_count), the bias estimate does not change.

## Comparative Example

We will define an example of the above model and compare how `bench_diff` and the *traditional* benchmarking method fare with respect to the model. The example is admittedly contrived in order to facilitate approximate calculations and also to highlight the potential disparity of results between the two benchmarking methods.

**Model parameters**

- The two functions, **f1** and **f2** are identical, with **λ1 = λ2 = 12 ms**.

- The number of executions of each function is **exec_count = 2500**. So, the total execution time, ignoring warm-up, is 1 minute.

- **α(t) = 1 + 1/2 * sin(t * 2*π / 60000)**, where **t** is the number of milliseconds elapsed since the start of the benchmark.  

  -  α'(t)  

    = 1/2 * 2\*π / 60000 * cos(t * 2\*π / 60000)  

    = π / 60000 * cos(t * 2\*π / 60000)

  Therefore:

  - |α'(t)| ≤ π / 60000.

  And we have the following bounds for α(t):

  - A<sub>L</sub> = 1/2
  - A<sub>D</sub> = π / 60000

- **β(t)** has **σ = 0.28**.

  - Therefore, E(β(t)) = exp(σ<sup>2</sup>/2) ≈ 1.04.

**`bench_diff` calculations**

1. Given exec_count = 2500, the noise contributed by β(t) is effectively eliminated.

2. From the model (*implication*), the bias of E(mean_diff_ln) is:

   - A<sub>D</sub> * (1 +  A<sub>D</sub>/A<sub>L</sub>) * (λ1+λ2)/2 * exp(σ<sup>2</sup>/2)  

     ≈ π / 60000 * (1 + π / 60000 / (1/2)) * (12 + 12)/2 * 1.04  

     = π / 60000 * (1 + π / 30000) * 12 * 1.04  

     ≈ 0.0006535

3. So, the multiplicative bias on the estimate of λ1/λ2 (which is 1 in our example) is:

   - exp(0.0006535) = 1.0006537, i.e., less than 1/10 of 1%.

4. Recall that bias does not depend on the number of executions, so it is the same with only half the number of executions. Also, given the high exec_count assumed, the `bench_diff` results during the first half should be very close to those obtained during the second half.

***Traditional* method calculations**

1. With the traditional method, we benchmark f1 with exec_count = 2500 and then benchmark f2 (which is the same as f1 in our example) with exec_count = 2500.

2. Given exec_count = 2500, the noise contributed by β(t) is effectively eliminated.

3. The first benchmark of f1 takes place during the first 30 seconds. The calculated mean latency is approximately:

   - λ1 / 30000 * **∫**<sub>0</sub><sup>30000</sup> α(t) dt  

     = 12 / 30000 * (30000 + 1/2 * (-cos(30000 * 2\*π / 60000) + cos(0)) / (2\*π / 60000))  

     = 12 + 12/30000 * 1/2 * (-cos(π) + cos(0)) / (2\*π / 60000)  

     = 12 + 12 * (-cos(π) + cos(0)) / (2\*π)  

     = 12 + 12 * (1 + 1) / (2\*π)  

     = 12 * (1 + 1/π)  

     ≈ 12 * 1.3183

   - This is an upward error of more than 30%.

4. The second benchmark of f1 takes place during the second 30 seconds. The calculated mean latency is approximately:

   - λ1 / 30000 * **∫**<sub>30000</sub><sup>60000</sup> α(t) dt  

     = 12 / 30000 * (30000 + 1/2 * (-cos(60000 * 2\*π / 60000) + cos(30000 * 2\*π / 60000)) / (2\*π / 60000))  

     = 12 + 12/30000 * 1/2 * (-cos(2\*π) + cos(π)) / (2\*π / 60000)  

     = 12 + 12 * (-cos(2\*π) + cos(π)) / (2\*π)  

     = 12 + 12 * (-1 + -1) / (2\*π)  

     = 12 * (1 - 1/π)  

     ≈ 12 * 0.6817

   - This is an downward error of more than 30%.

5. The estimated ratio λ1 / λ1 is approximately 1.3183 / 0.6817 ≈ 1.9338, an estimating error of close to !00%.

# Limitations

This library works well for latencies at the microseconds or milliseconds order of magnitude, but not for latencies at the nanoseconds order of magnitude.