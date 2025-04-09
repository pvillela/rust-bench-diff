# A Model of Time-Dependent Random Noise

Following is a simple model of time-dependent random noise. While this model can be useful as a motivation for the `bench_diff` approach, the test benchmarks discussed previously provide independent validation of the benchmarking approach used in this library.

**Definitions and assumptions:**

1. Let **ln(x)** be the natural logarithm of **x**.
2. Let **L(f, t)** be the latency of function **f** at time **t**.
3. Let **λ1** be the baseline (ideal) latency of function **f1** in the absence of noise; respectively, **λ2** for **f2**.
4. Given a random variable **χ**, let **E(χ)** and **Stdev(χ)** be the expected value and standard deviation of **χ**, respectively.
5. Assume time-dependent noise is **ν(t) = α(t) * β(t)**, where:
   - **α(t)** > 0 is a smooth deterministic function of **t**, such that:
     - There are constants **A<sub>L</sub>**, **A<sub>U</sub>** such that **A<sub>L</sub> ≤ α(t) ≤ A<sub>U</sub>** for all **t**.
     - There is a constant **A<sub>D</sub>** such that the absolute value of the derivative **d<sub>/dt</sub>ln(α(t))** is bounded by **A<sub>D</sub>**. By the chain rule, **|α'(t)/α(t)| ≤ A<sub>D</sub>**, for all **t**.
   - **β(t)** is a random variable dependent on **t**, with a log-normal distribution, such that **E(ln(β(t))) = 0** and **Stdev(ln(β(t))) = σ**, where **σ** is a constant that does not depend on **t**.
6. Assume **L(f1, t) = λ1 * ν(t)** and **L(f2, t) = λ2 * ν(t)** for all **t**.

**Implications:**

1. When we measure **f1**'s latency at a time **t<sub>1</sub>**, getting **L(f1, t<sub>1</sub>)**, and right after we measure **f2**'s latency, the measurement for **f2** occurs at a time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**, where **Δt<sub>1</sub>** is <u>very close</u> to **L(f1, t<sub>1</sub>)**.

2. Substituting *assumption 5* into *assumption 6* for **f1** at time **t<sub>1</sub>** and **f2** at time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**:

   - L(f1, t<sub>1</sub>) = λ1 * α(t<sub>1</sub>) * β(t<sub>1</sub>)
   - L(f2, t<sub>2</sub>) = λ2 * α(t<sub>1</sub> + Δt<sub>1</sub>) * β(t<sub>2</sub>)

3. Applying natural logarithms on *implication 2*:

   - ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))
   - ln(L(f2, t<sub>2</sub>)) = ln(λ2) + ln(α(t<sub>1</sub> + Δt<sub>1</sub>)) + ln(β(t<sub>2</sub>))

4. Based on *assumption 5*'s smoothness of **α(t)** and bounds on **d<sub>/dt</sub>ln(α(t))**:

   - ln(α(t<sub>1</sub> + Δt<sub>1</sub>))  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>p</sub>)  **[**_for some t<sub>p</sub> between t<sub>1</sub> and Δt<sub>1</sub>_**]**  

     Using the definition ε<sub>1</sub>(t<sub>1</sub>) =<sub>def</sub> α'(t<sub>p</sub>)/α(t<sub>p</sub>):
     
     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>)  **[**_where |ε<sub>1</sub>(t<sub>1</sub>)| ≤ A<sub>D</sub>_**]**

5. Applying *implication 4* to *implication 3*:

   - ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))

   - ln(L(f2, t<sub>2</sub>))  

     = ln(λ2) + ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>) + ln(β(t<sub>2</sub>))   **[**_where |ε<sub>1</sub>(t<sub>1</sub>)| <= A<sub>D</sub>_**]**  

     Using *implication 1* to replace the Δt<sub>1</sub> value above:  

     = lln(λ2) + ln(α(t<sub>1</sub>)) + L(f1, t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>) + ln(β(t<sub>2</sub>))

6. Using the first equation in *implication 2* and the formula for the expected value of a log-normal distribution:

   - E(L(f1, t<sub>1</sub>))  

     = λ1 * α(t<sub>1</sub>) * E(β(t<sub>1</sub>))  

     = λ1 * α(t<sub>1</sub>) * exp(σ<sup>2</sup>/2)

7. Taking expected values in *implication 5*:

   - E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>)) + E(ln(β(t<sub>1</sub>)))
   - E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>)) + E(ln(β(t<sub>2</sub>)))

8. Using the second item in *assumption 5*, *implication 7* simplifies to:

   - E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>))
   - E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>))

9. Defining **δ<sub>1</sub>(t<sub>1</sub>) =<sub>def</sub> E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>))** and by standard expected value inequalities:

   - |δ<sub>1</sub>(t<sub>1</sub>)|  

     = |E(ε<sub>1</sub>(t<sub>1</sub>) * L(f1, t<sub>1</sub>))|  

     ≤ E(|ε<sub>1</sub>(t<sub>1</sub>)| * |L(f1, t<sub>1</sub>)|)  

     ≤ E(|ε<sub>1</sub>(t<sub>1</sub>)|) * E(|L(f1, t<sub>1</sub>)|)  

     = E(|ε<sub>1</sub>(t<sub>1</sub>)|) * E(L(f1, t<sub>1</sub>))  

     Using the  bound|ε<sub>1</sub>(t<sub>1</sub>)| <= A<sub>D</sub> from *implication 4*:

     ≤ A<sub>D</sub> * E(L(f1, t<sub>1</sub>))  

     By *implication 6*:
     
     = A<sub>D</sub> * λ1 * α(t<sub>1</sub>) * exp(σ<sup>2</sup>/2)

10. Using *implication 8* and substituting *implication 9* into the second equation:

       - E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>))
       - E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + δ<sub>1</sub>(t<sub>1</sub>)  **[**_where |δ<sub>1</sub>(t<sub>1</sub>)| ≤ A<sub>D</sub> * λ1 * α(t<sub>1</sub>) * exp(σ<sup>2</sup>/2)_**]**  

11. Subtracting the second equation from the first in *implication 10* and using the linearity of **E()**:

    - E(ln(L(f1, t<sub>1</sub>) - ln(L(f2, t<sub>2</sub>))))  

      = ln(λ1) - ln(λ2) - δ<sub>1</sub>(t<sub>1</sub>)  

      = ln(λ1 / λ2) - δ<sub>1</sub>(t<sub>1</sub>)  **[**_where |δ<sub>1</sub>(t<sub>1</sub>)| ≤ A<sub>D</sub> * λ1 * α(t<sub>1</sub>) * exp(σ<sup>2</sup>/2)_**]**

12. With `bench_diff`, measurements are done pairs, with one half of the pairs having **f1** followed by **f2** and the other half having **f2** followed by **f1**. The equation in *implication 11* above pertains to the first case. The analogous equation for the second case is:  

    - E(ln(L(f2, t<sub>2'</sub>) - ln(L(f1, t<sub>1'</sub>)))) = ln(λ2 / λ1) - δ<sub>2</sub>(t<sub>2'</sub>)  

    Or, equivalently:

    - E(ln(L(f1, t<sub>1'</sub>)) - ln(L(f2, t<sub>2'</sub>))) = ln(λ1 / λ2) + δ<sub>2</sub>(t<sub>2'</sub>)  **[**_where |δ<sub>2</sub>(t<sub>2'</sub>)| ≤ A<sub>D</sub> * λ2 * α(t<sub>2'</sub>) * exp(σ<sup>2</sup>/2)_**]**

13. Assuming the number of latency observations for each function is **n** and considering the two cases as described in *implication 12*, we can calculate the sample mean difference between the natural logarithms of the observed latencies:
    - mean_diff_ln  
      
      =<sub>def</sub> (1/n) * ∑<sub>i=1..n</sub> (ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>))  
      
      = (1/n) * (∑<sub>i:odd</sub> (ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)) + ∑<sub>i:even</sub> (ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)))

14. Taking expected values in *implication 13* and using the linearity of **E()**:

    - E(mean_diff_ln)  

      = (1/n) * (∑<sub>i:odd</sub> E(ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)) + ∑<sub>i:even</sub> E(ln(L(f1, t<sub>1,i</sub>) - ln(L(f2, t<sub>2,i</sub>)))  

      By *implication 11* and *implication 12*:

      = (1/n) * (∑<sub>i:odd</sub> (ln(λ1 / λ2) - δ<sub>1</sub>(t<sub>1,i</sub>)) + ∑<sub>i:even</sub> (ln(λ1 / λ2) + δ<sub>2</sub>(t<sub>2,i</sub>)))  

      = ln(λ1 / λ2) + (1/n) * ∑<sub>i:odd</sub> (δ<sub>2</sub>(t<sub>2,i+1</sub>) - δ<sub>1</sub>(t<sub>1,i</sub>))  

      **[**_where where |δ<sub>1</sub>(t<sub>1,i</sub>)| ≤ A<sub>D</sub> * λ1 * α(t<sub>1,i</sub>) * exp(σ<sup>2</sup>/2) and|δ<sub>2</sub>(t<sub>2,i</sub>)| ≤ A<sub>D</sub> * λ2 * α(t<sub>2,i</sub>) * exp(σ<sup>2</sup>/2)_**]**

15. The equation in *implication 14* shows that, with `bench_diff`, the difference between the sample means of the natural logarithms of the observed latencies is biased estimator of **ln(λ1 / λ2)**, with a bias of:

    - (1/n) * ∑<sub>i:odd</sub> (δ<sub>2</sub>(t<sub>2,i+1</sub>) - δ<sub>1</sub>(t<sub>1,i</sub>))  

      ≤ (1/n) * ∑<sub>i:odd</sub> (|δ<sub>2</sub>(t<sub>2,i+1</sub>)| + |δ<sub>1</sub>(t<sub>1,i</sub>)|)  

      ≤ (1/n) * ∑<sub>i:odd</sub> ((A<sub>D</sub> * λ2 * α(t<sub>2,i</sub>) * exp(σ<sup>2</sup>/2)) + (A<sub>D</sub> * λ1 * α(t<sub>1,i</sub>) * exp(σ<sup>2</sup>/2))  

      = A<sub>D</sub> * (λ1 + λ2)/2 * exp(σ<sup>2</sup>/2) * (1/n) * ∑<sub>i:odd</sub> (α(t<sub>2,i</sub>) + α(t<sub>1,i</sub>))  

      ≤ A<sub>D</sub> * (λ1 + λ2)/2 * exp(σ<sup>2</sup>/2) * A<sub>U</sub>

16. Thus, assuming the above product is sufficiently small, the estimates of the ratio of latency medians produced by `bench_diff` should be sufficiently accurate.


# Limitations

This library works well for latencies at the microseconds or milliseconds order of magnitude, but not for latencies at the nanoseconds order of magnitude.