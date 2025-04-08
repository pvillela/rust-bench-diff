# A Model of Time-Dependent Random Noise

Following is a simple model of time-dependent random noise. While this model can be useful as a motivation for the `bench_diff` approach, the test benchmarks discussed previously provide independent validation of the benchmarking approach used in this library.

**Definitions and assumptions:**

1. Let **ln(x)** be the natural logarithm of **x**.
2. Let **L(f, t)** be the latency of function **f** at time **t**.
3. Let **λ1** be the baseline (ideal) latency of function **f1** in the absence of noise; respectively, **λ2** for **f2**.
4. Given a random variable **χ**, let **E(χ)** and **Stdev(χ)** be the expected value and standard deviation of **χ**, respectively.
5. Assume time-dependent noise is **ν(t) = α(t) * β(t)**, where:
   - **α(t)** is a smooth deterministic function of **t**.
   - **β(t)** is a random variable dependent on **t**, with a log-normal distribution, such that **E(ln(β(t))) = 0** and **Stdev(ln(β(t))) = σ**, where **σ** is a constant that does not depend on **t**.
6. Assume **L(f1, t) = λ1 * ν(t)** and **L(f2, t) = λ2 * ν(t)** for all **t**.

**Implications:**

1. When we measure **f1**'s latency at a time **t<sub>1</sub>**, getting **L(f1, t<sub>1</sub>)**, and right after we measure **f2**'s latency, the measurement for **f2** occurs at a time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**, where **Δt<sub>1</sub>** is <u>very close</u> to **L(f1, t<sub>1</sub>)**.

2. Substituting *assumption 5* into *assumption 6* for **f1** at time **t<sub>1</sub>** and **f2** at time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**:

   - **L(f1, t<sub>1</sub>) = λ1 * α(t<sub>1</sub>) * β(t<sub>1</sub>)**
   - **L(f2, t<sub>2</sub>) = λ2 * α(t<sub>1</sub> + Δt<sub>1</sub>) * β(t<sub>2</sub>)**

3. Applying natural logarithms on *implication 2*:

   - **ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))**
   - **ln(L(f2, t<sub>2</sub>)) = ln(λ2) + ln(α(t<sub>1</sub> + Δt<sub>1</sub>)) + ln(β(t<sub>2</sub>))** 

4. Applying a linear approximation with the derivative of **α** (**α'**) to *implication 3*:

   - **ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))**
   - **ln(L(f2, t<sub>2</sub>)) = ln(λ2) + ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>)) + ε<sub>1</sub>(t<sub>1</sub>)**, where **ε<sub>1</sub>(t<sub>1</sub>)** is close to zero

5. Using *implication 1* to replace the **Δ** value in the linear approximation term in the second equation of *implication 4*:

   - **ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))**
   - **ln(L(f2, t<sub>2</sub>)) = ln(λ2) + ln(α(t<sub>1</sub>)) + L(f1, t<sub>1</sub>) * α'(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>)) + ε<sub>1</sub>(t<sub>1</sub>)**, where **ε<sub>1</sub>(t<sub>1</sub>)** is close to zero

6. Using the first equation in *implication 2* and the formula for the expected value of a log-normal distribution:

   - **E(L(f1, t<sub>1</sub>)) = λ1 * α(t<sub>1</sub>) * E(β(t<sub>1</sub>)) = λ1 * α(t<sub>1</sub>) * exp(1/2 * σ^2)**

7. Taking expected values in *implication 5*:

   - **E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>)) + E(ln(β(t<sub>1</sub>)))**
   - **E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + α'(t<sub>1</sub>)/α(t<sub>1</sub>) * E(L(f1, t<sub>1</sub>)) + E(ln(β(t<sub>2</sub>))) + E(ε<sub>1</sub>(t<sub>1</sub>))**

8. Using the second item in *assumption 5*, *implication 7* simplifies to:

   - **E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>))**
   - **E(ln(L(f2, t<sub>2</sub>))) = ln(λ2) + ln(α(t<sub>1</sub>)) + α'(t<sub>1</sub>)/α(t<sub>1</sub>) * E(L(f1, t<sub>1</sub>)) + E(ε<sub>1</sub>(t<sub>1</sub>))**

9. Using *implication 8* and substituting *implication 6* into the second equation:

   - **E(ln(L(f1, t<sub>1</sub>))) = ln(λ1) + ln(α(t<sub>1</sub>))**
   - **E(ln(L(f2, t<sub>2</sub>)))**  
     **= ln(λ2) + ln(α(t<sub>1</sub>)) + α'(t<sub>1</sub>)/α(t<sub>1</sub>) * λ1 * α(t<sub>1</sub>) * exp(1/2 * σ^2) + E(ε<sub>1</sub>(t<sub>1</sub>))**  
     **= ln(λ2) + ln(α(t<sub>1</sub>)) + α'(t<sub>1</sub>) * λ1 * exp(1/2 * σ^2) + E(ε<sub>1</sub>(t<sub>1</sub>))**

10. Subtracting the second equation from the first in *implication 9* and using the linearity of **E()**:

    - **E(ln(L(f1, t<sub>1</sub>) - ln(L(f2, t<sub>2</sub>))))**  
      **= ln(λ1) - ln(λ2) - α'(t<sub>1</sub>) * λ1 * exp(1/2 * σ^2) - E(ε<sub>1</sub>(t<sub>1</sub>)) =**  
      **= ln(λ1 / λ2) - α'(t<sub>1</sub>) * λ1 * exp(1/2 * σ^2) - E(ε<sub>1</sub>(t<sub>1</sub>))**

11. When we measure **f1**'s latency with `bench_diff` at time **t<sub>1'</sub>**, getting **L(f1, t<sub>1'</sub>)**, it happens right after we measure **f2**'s latency at a time **t<sub>2'</sub>**, so **t<sub>1'</sub> = t<sub>2'</sub> + Δt<sub>2'</sub>**, where **Δt<sub>2'</sub>** is <u>very close</u> to **L(f2, t<sub>2'</sub>)**.

12. Based on *implication 11*, an equation analogous to that of *implication 10* can be derived:

    - **E(ln(L(f2, t<sub>2'</sub>) - ln(L(f1, t<sub>1'</sub>)))) = ln(λ2 / λ1) - α'(t<sub>2'</sub>) * λ2 * exp(1/2 * σ^2) - E(ε<sub>2</sub>(t<sub>2'</sub>))**  
      or, equivalently: 
    - **E(ln(L(f1, t<sub>1'</sub>)) - ln(L(f2, t<sub>2'</sub>))) = ln(λ1 / λ2) + α'(t<sub>2'</sub>) * λ2 * exp(1/2 * σ^2) + E(ε<sub>2</sub>(t<sub>2'</sub>))**

13. Using the linearity of **E()** and the definition of the sample mean **M(i, x<sub>i</sub>)** for a sample **x** with observations **x<sub>1</sub>, ..., x<sub>i</sub>, ..., x<sub>n</sub>**:

    - **E(M(i, ln(L(f1, t<sub>1<sub>i</sub></sub>) - ln(L(f2, t<sub>2<sub>i</sub></sub>))))) = M(i, E(ln(L(f1, t<sub>1<sub>i</sub></sub>) - ln(L(f2, t<sub>2<sub>i</sub></sub>)))))**

14. Using *implication 13* and taking the sample mean **M()** of each side in *implication 10* and *implication 12* over all measured latencies, we get:

    - **E(M(i, ln(L(f1, t<sub>1<sub>i</sub></sub>) - ln(L(f2, t<sub>2<sub>i</sub></sub>)))) = ln(λ1 / λ2) - M(i, α'(t<sub>1<sub>i</sub></sub>)) * λ1 * exp(1/2 * σ^2) - M(i, E(ε<sub>1</sub>(t<sub>1<sub>i</sub></sub>))))**
    - **E(M(i, ln(L(f1, t<sub>1'<sub>i</sub></sub>) - ln(L(f2, t<sub>2'<sub>i</sub></sub>)))) = ln(λ1 / λ2) + M(i, α'(t<sub>2'<sub>i</sub></sub>)) * λ2 * exp(1/2 * σ^2) + M(i, E(ε<sub>2</sub>(t<sub>2'<sub>i</sub></sub>))))**

15. Both equations in *implication 14* show that the difference between the sample means of the natural logarithms of the observed latencies is an approximately unbiased estimator of **ln(λ1 / λ2)**. The bias is equal to the following equivalent values:

    - **- M(i, α'(t<sub>1<sub>i</sub></sub>)) * λ1 * exp(1/2 * σ^2) - M(i, E(ε<sub>1</sub>(t<sub>1<sub>i</sub></sub>)))**
    - **M(i, α'(t<sub>2'<sub>i</sub></sub>)) * λ2 * exp(1/2 * σ^2) + M(i, E(ε<sub>2</sub>(t<sub>2'<sub>i</sub></sub>)))**

16. Thus, assuming the rate of change of **α(t)** is sufficiently small for all **t** during the measurement process, the estimates of the ratio of latency medians produced by `bench_diff` should be sufficiently accurate.


# Limitations

This library works well for latencies at the microseconds or millisecodns order of magnitude, but not for latencies at the nanoseconds order of magnitude.