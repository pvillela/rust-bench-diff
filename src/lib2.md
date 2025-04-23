# A Model of Time-Dependent Random Noise

Following is a simple mathematical model of time-dependent random noise. This model is useful for reasoning about time-dependent noise and helps in understanding why `bench_diff` is more effective than traditional benchmarking for the comparison of latencies between two functions.

Nonetheless, the model's fit with reality is not necessary to validate `bench_diff` as the test benchmarks discussed previously provide independent validation of the library.

## The Model

The first section defines the model. Subsequent sections develop estimates for some relevant statistics and parameters.

**Definitions and Assumptions**

This section defines the model per se.

1. Let **ln(x)** be the natural logarithm of **x**.
2. Let **L(f, t)** be the latency of function **f** at time **t**.
3. Let **λ<sub>1</sub>** be the baseline (ideal) latency of function **f<sub>1</sub>** in the absence of noise; respectively, **λ<sub>2</sub>** for **f<sub>2</sub>**.
4. Given a random variable **χ**, let **E(χ)** and **Stdev(χ)** be the expectation and standard deviation of **χ**, respectively.
5. Assume time-dependent noise is **ν(t) =<sub>def</sub> α(t) * β(t)**, where:
   - **α(t)** is a differentiable deterministic function of **t**, such that there are positive constants **A<sub>L</sub>**, **A<sub>U</sub>**, and **A<sub>D</sub>** for which **A<sub>L</sub> ≤ α(t) ≤ A<sub>U</sub>** and **|α'(t)| ≤ A<sub>D</sub>**, for all **t**.
   - **β(t)** is a family of mutually independent log-normal random variables indexed by **t**, such that  **E(ln(β(t))) = 0** and **Stdev(ln(β(t))) = σ**, where **σ** is a constant that does not depend on **t**.

6. Assume **L(f<sub>1</sub>, t) = λ<sub>1</sub> * ν(t)** and **L(f<sub>2</sub>, t) = λ<sub>2</sub> * ν(t)** for all **t**.  

   A couple of points to notice about this model:  

   - λ<sub>1</sub> and λ<sub>2</sub> can't be determined independently of α. If we multiply λ<sub>1</sub> and λ<sub>2</sub> by a constant factor r and divide α by the same factor, we end up with an exactly equivalent model. Nonetheless, the ratio λ<sub>1</sub>/λ<sub>2</sub> remains the same regardless of the multiplier r, so the model is useful to reason about the ratio of the function latencies or, equivalently, the difference of the logarithm of the latencies.
   - As a log-normally distributed random variable, β(t) can take arbitrarily small positive values, which implies that the function latencies can take arbitrarily small values. That is clearly not 100% realistic. It is, however, a reasonable approximation of reality, especially when σ is close to 0, in which case the values of β(t) are clustered around 1.

7. Let **mean_diff_ln** be the sample mean difference between the natural logarithms of the observed latencies.

8. When we measure f<sub>1</sub>'s latency at a time **t<sub>1</sub>**, getting L(f<sub>1</sub>, t<sub>1</sub>), and right after we measure f<sub>2</sub>'s latency, the measurement for f<sub>2</sub> occurs at a time **t<sub>2</sub>** that is very close to L(f<sub>1</sub>, t<sub>1</sub>). We assume that **t<sub>2</sub> = t<sub>1</sub> + L(f<sub>1</sub>, t<sub>1</sub>)**.

**Game Plan**

The goal is to obtain a bound on how closely mean_diff_ln estimates ln(λ<sub>1</sub>/λ<sub>2</sub>):

- Obtain an upper bound BE on |E(mean_diff_ln) - ln(λ<sub>1</sub>/λ<sub>2</sub>)|.
- Obtain an upper bound BA on E(|mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)|).
- mean_diff_ln is approximately normal, so its median and mean are approximately the same.
- The median minimizes the mean absolute deviation, so E(|mean_diff_ln - E(mean_diff_ln))|) ≤ BA.
- A normal distribution with a mean absolute deviation from its mean ≤ BA has stdev ≤ BA * √(π/2).
- The 99% confidence interval around the mean for a normal distribution is stdev * 2.58.
- |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)| ≤ |mean_diff_ln - E(mean_diff_ln)| + |E(mean_diff_ln) - ln(λ<sub>1</sub>/λ<sub>2</sub>)|.
- With 99% confidence, |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)| ≤ BA * √(π/2) * 2.58 + BE.

**Expansion of mean_diff_ln**

This section expands **mean_diff_ln** (see *Definition 7*) to facilitate subsequent upper bound calculations.

1. When f<sub>1</sub> is executed at time t<sub>1</sub> and f<sub>2</sub> is executed right after at time t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>, using *Assumptions 5, 6, 8*:

   - L(f<sub>1</sub>, t<sub>1</sub>) = λ<sub>1</sub> * α(t<sub>1</sub>) * β(t<sub>1</sub>)
   - L(f<sub>2</sub>, t<sub>2</sub>) = λ<sub>2</sub> * α(t<sub>1</sub> + Δt<sub>1</sub>) * β(t<sub>2</sub>)  **[** _where Δt<sub>1</sub> = L(f<sub>1</sub>, t<sub>1</sub>)_ **]**

2. Applying natural logarithms on *Point 2*:

   - ln(L(f<sub>1</sub>, t<sub>1</sub>)) = ln(λ<sub>1</sub>) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))
   - ln(L(f<sub>2</sub>, t<sub>2</sub>)) = ln(λ<sub>2</sub>) + ln(α(t<sub>1</sub> + Δt<sub>1</sub>)) + ln(β(t<sub>2</sub>))

3. By Lagrange's mean value theorem for ln(α(t)) and the bounds on α(t) and α'(t) from *Assumption 5*:

   - ln(α(t<sub>1</sub> + Δt<sub>1</sub>))  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>p</sub>)  **[** _for some t<sub>p</sub> between t<sub>1</sub> and t<sub>1</sub> + Δt<sub>1</sub>_ **]**  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>1</sub>) * α(t<sub>1</sub>)/α(t<sub>p</sub>)  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>1</sub>) * (1 + α(t<sub>1</sub>)/α(t<sub>p</sub>) - α(t<sub>1</sub>)/α(t<sub>1</sub>))  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>1</sub>) * (1 + α(t<sub>1</sub>) * 1/(α(t<sub>p</sub>) - 1/α(t<sub>1</sub>)))  

     By Lagrange's mean value theorem for 1/α(t):

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>1</sub>) * (1 + α(t<sub>1</sub>) * 1/(α(t<sub>p</sub>) - 1/α(t<sub>1</sub>)))  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>1</sub>) * (1 + α(t<sub>1</sub>) * -α'(t<sub>q</sub>)/α(t<sub>q</sub>)<sup>2</sup> * (t<sub>p</sub> - t<sub>1</sub>))  **[** _for some t<sub>p</sub> between t<sub>1</sub> and t<sub>p</sub>_ **]**  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>)  

     **[** _where ε<sub>1</sub>(t<sub>1</sub>) =<sub>def</sub> α'(t<sub>p</sub>)\*(1\-α(t<sub>1</sub>)\*α'(t<sub>q</sub>)/α(t<sub>q</sub>)<sup>2</sup>\*(t<sub>p</sub>\-t<sub>1</sub>)), and thus |ε<sub>1</sub>(t<sub>1</sub>)| ≤ A<sub>D</sub>\*(1\+A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub><sup>2</sup>\*Δt<sub>1</sub>)_ **]**

4. Applying *Point 3* to *Point 2*:

   - ln(L(f<sub>1</sub>, t<sub>1</sub>)) = ln(λ<sub>1</sub>) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))

   - ln(L(f<sub>2</sub>, t<sub>2</sub>))  

     = ln(λ<sub>2</sub>) + ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))  

     Since Δt<sub>1</sub> = L(f<sub>1</sub>, t<sub>1</sub>):  

     = ln(λ<sub>2</sub>) + ln(α(t<sub>1</sub>)) + L(f<sub>1</sub>, t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))  

     Using *Assumptions 5 and 6* to expand L(f<sub>1</sub>, t<sub>1</sub>):  

     = ln(λ<sub>2</sub>) + ln(α(t<sub>1</sub>)) + (λ<sub>1</sub> * α(t<sub>1</sub>) * β(t<sub>1</sub>)) * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))  

     = ln(λ<sub>2</sub>) + ln(α(t<sub>1</sub>)) + λ<sub>1</sub> * β(t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>) + ln(β(t<sub>2</sub>))

5. Subtracting the second equation from the first in *Point 5*:

   - ln(L(f<sub>1</sub>, t<sub>1</sub>) - L(f<sub>2</sub>, t<sub>2</sub>))  

     = ln(λ<sub>1</sub>) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>)) - ln(λ<sub>2</sub>) - ln(α(t<sub>1</sub>)) - λ<sub>1</sub> * β(t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>) - ln(β(t<sub>2</sub>))  

     = ln(λ<sub>1</sub>/λ<sub>2</sub>) - λ<sub>1</sub> * β(t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>) + ln(β(t<sub>1</sub>)) - ln(β(t<sub>2</sub>))

6. Using *Assumptions 5, 6, 8* to expand L(f<sub>1</sub>, t<sub>1</sub>) and rewrite the bound on |ε<sub>1</sub>(t<sub>1</sub>)|:

   - |ε<sub>1</sub>(t<sub>1</sub>)|  

     ≤ A<sub>D</sub>\*(1\+A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub><sup>2</sup>\*Δt<sub>1</sub>)  

     = A<sub>D</sub>\*(1\+A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub><sup>2</sup>\*L(f<sub>1</sub>, t<sub>1</sub>))  

     = A<sub>D</sub>\*(1\+A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub><sup>2</sup>\*(λ<sub>1</sub> * α(t<sub>1</sub>) * β(t<sub>1</sub>)))  

     ≤ A<sub>D</sub>\*(1\+A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub><sup>2</sup>\*(λ<sub>1</sub> * A<sub>U</sub> * β(t<sub>1</sub>)))  

     = A<sub>D</sub>\*(1\+λ<sub>1</sub>\*A<sub>D</sub>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*β(t<sub>1</sub>))

7. With `bench_diff`, measurements are done pairs, with one half of the pairs having **f<sub>1</sub>** followed by **f<sub>2</sub>** and the other half having **f<sub>2</sub>** followed by **f<sub>1</sub>**. The equation in *Point 5* above pertains to the first case. The analogous equation for the second case is:  

   - ln(L(f<sub>2</sub>, t<sub>2'</sub>) - ln(L(f<sub>1</sub>, t<sub>1'</sub>)) = ln(λ<sub>2</sub>/λ<sub>1</sub>) - λ<sub>2</sub> * β(t<sub>2'</sub>) * ε<sub>2</sub>(t<sub>2'</sub>) + ln(β(t<sub>1'</sub>)) - ln(β(t<sub>2'</sub>))  

   Or, equivalently:

   - ln(L(f<sub>1</sub>, t<sub>1'</sub>)) - ln(L(f<sub>2</sub>, t<sub>2'</sub>) = ln(λ<sub>1</sub>/λ<sub>2</sub>) + λ<sub>2</sub> * β(t<sub>2'</sub>) * ε<sub>2</sub>(t<sub>2'</sub>) - ln(β(t<sub>1'</sub>)) + ln(β(t<sub>2'</sub>))  

     **[** _where ε<sub>2</sub>(t<sub>2'</sub>) =<sub>def</sub> α'(t<sub>p'</sub>)\*(1\-α(t<sub>2'</sub>)\*α'(t<sub>q'</sub>)/α(t<sub>q'</sub>)<sup>2</sup>\*(t<sub>p'</sub>\-t<sub>2'</sub>)), and thus |ε<sub>2</sub>(t<sub>2'</sub>)| ≤ A<sub>D</sub>\*(1\+λ<sub>2</sub>\*A<sub>D</sub>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*β(t<sub>2'</sub>))) (see Point 6)_ **]**

8. Assuming the number of latency observations for each function is **n** and considering the two cases as described in *Point 7*, we can calculate the sample mean difference between the natural logarithms of the observed latencies (see *Definition 7*):  

   - mean_diff_ln = (1/n) * ∑<sub>i=1..n</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>)))  

     = (1/n) * (∑<sub>i:odd</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) + ∑<sub>i:even</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))))  

     = (1/n) * ∑<sub>i:odd</sub> (ln(λ<sub>1</sub>/λ<sub>2</sub>) - λ<sub>1</sub> * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) +  

        (1/n) * ∑<sub>i:even</sub> (ln(λ<sub>1</sub>/λ<sub>2</sub>) + λ<sub>2</sub> * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>) - ln(β(t<sub>1,i</sub>)) + ln(β(t<sub>2,i</sub>)))  

     = ln(λ<sub>1</sub>/λ<sub>2</sub>) +  

        (1/n) * ∑<sub>i:odd</sub> (-λ<sub>1</sub> * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) +  

        (1/n) * ∑<sub>i:even</sub> (λ<sub>2</sub> * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>) - ln(β(t<sub>1,i</sub>)) + ln(β(t<sub>2,i</sub>)))

**Expectation of mean_diff_ln**

This section calculates a bound on |E(mean_diff_ln) - ln(λ<sub>1</sub>/λ<sub>2</sub>)| (see *Definition 7*).

1. From *Expansion Point 8*:

   - E(mean_diff_ln) - ln(λ<sub>1</sub>/λ<sub>2</sub>)  

      Due to the linearity of E() and the fact that E(β(t)) = 0:

      = (1/n) * -λ<sub>1</sub> * ∑<sub>i:odd</sub>  E(β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>)) + (1/n) * λ<sub>2</sub> * ∑<sub>i:even</sub> E(β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>))

2. From *Point 1* and the bounds on ε<sub>1</sub>(t<sub>1,i</sub>) and ε<sub>2</sub>(t<sub>2,i</sub>) from *Expansion Points 6 and 7*:

   - |E(mean_diff_ln) - ln(λ<sub>1</sub>/λ<sub>2</sub>)|  

      ≤ (1/n) * λ<sub>1</sub> * ∑<sub>i:odd</sub>  E(β(t<sub>1,i</sub>) * |ε<sub>1</sub>(t<sub>1,i</sub>)|) + (1/n) * λ<sub>2</sub> * ∑<sub>i:even</sub> E(β(t<sub>2,i</sub>) * |ε<sub>2</sub>(t<sub>2,i</sub>)|)  

      ≤ (1/n) * λ<sub>1</sub> * ∑<sub>i:odd</sub>  E(β(t<sub>1,i</sub>) * A<sub>D</sub>\*(1\+λ<sub>1</sub>\*A<sub>D</sub>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*β(t<sub>1,i</sub>))) +  
      (1/n) * λ<sub>2</sub> * ∑<sub>i:even</sub> E(β(t<sub>2,i</sub>) * A<sub>D</sub>\*(1\+λ<sub>2</sub>\*A<sub>D</sub>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*β(t<sub>2,i</sub>)))  

      = (1/n) * λ<sub>1</sub> * ∑<sub>i:odd</sub>  (A<sub>D</sub>\*E(β(t<sub>1,i</sub>) + λ<sub>1</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*E(β(t<sub>1,i</sub>)<sup>2</sup>)) + 
      (1/n) * λ<sub>2</sub> * ∑<sub>i:even</sub> (A<sub>D</sub>\*E(β(t<sub>2,i</sub>) + λ<sub>2</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*E(β(t<sub>2,i</sub>)<sup>2</sup>))  

      By the expectation of log-normal distributions:  

      = (1/n) * λ<sub>1</sub> * ∑<sub>i:odd</sub>  (A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + λ<sub>1</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp((2\*σ)<sup>2</sup>/2)) +  
      (1/n) * λ<sub>2</sub> * ∑<sub>i:even</sub> (A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + λ<sub>2</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp((2\*σ)<sup>2</sup>/2))  

      = (1/n) * λ<sub>1</sub> * ∑<sub>i:odd</sub>  (A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + λ<sub>1</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>)) +  
      (1/n) * λ<sub>2</sub> * ∑<sub>i:even</sub> (A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + λ<sub>2</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>))  

      = λ<sub>1</sub> / 2 * (A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + λ<sub>1</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>)) +  
      λ<sub>2</sub> / 2 * (A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + λ<sub>2</sub>\*A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>))  

      = (λ<sub>1</sub>\+λ<sub>2</sub>)/2 * A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + (λ<sub>1</sub><sup>2</sup>+λ<sub>2</sub><sup>2</sup>)/2 * A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>)

3. Define **BE** as the right-hand-side of the immediately above equality. Thus:

   - |E(mean_diff_ln) - ln(λ<sub>1</sub>/λ<sub>2</sub>)| ≤ BE.

**Mean Absolute Error of mean_diff_ln**

This section calculates the expected absolute error of **mean_diff_ln** (see *Definition 7*) as an estimator of **ln(λ<sub>1</sub>/λ<sub>2</sub>)**.

1. From *Expansion Point 8*::

   - **absolute_error(mean_diff_ln)** =<sub>def</sub> |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)|  

     = |(1/n) * ∑<sub>i:odd</sub> (-λ<sub>1</sub> * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) +  

     ​     (1/n) * ∑<sub>i:even</sub> (λ<sub>2</sub> * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>) - ln(β(t<sub>1,i</sub>)) + ln(β(t<sub>2,i</sub>)))|

     = |(1/n) * ∑<sub>i:odd</sub> -λ<sub>1</sub> * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + (1/n) * ∑<sub>i:even</sub> λ<sub>2</sub> * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>) +  

     ​     (1/n) * ∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|  

     ≤ (1/n) * |∑<sub>i:odd</sub> -λ<sub>1</sub> * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + (1/n) * ∑<sub>i:even</sub> λ<sub>2</sub> * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>)| +  

        (1/n) * |∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|

2. From the above point, defining the mean absolute error:

   - **mean_absolute_error(mean_diff_ln)** =<sub>def</sub> E(absolute_error(mean_diff_ln))  

       ≤ (1/n) * λ<sub>1</sub> * ∑<sub>i:odd</sub>  E(β(t<sub>1,i</sub>) * |ε<sub>1</sub>(t<sub>1,i</sub>)|) + (1/n) * λ<sub>2</sub> * ∑<sub>i:even</sub> E(β(t<sub>2,i</sub>) * |ε<sub>2</sub>(t<sub>2,i</sub>)|) +  
      (1/n) * E(|∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|)  
      
      By *Expectation Point 2*, the first line on the right hand side of the above inequality is bounded by the last line of *Expectation Point 2*. Thus:  
       
       ≤ (λ<sub>1</sub>\+λ<sub>2</sub>)/2 * A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + (λ<sub>1</sub><sup>2</sup>+λ<sub>2</sub><sup>2</sup>)/2 * A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>) + 
       
      (1/n) * E(|∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|)  
       
       By *Expectation Point 3*:  
       
       = BE +   
      (1/n) * E(|∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|)  
       
       On the line immediately above, term inside the absolute value is the sum of 2*n independently distributed normal random variables with mean 0 and standard deviation σ. It is, therefore, a normal distribution with mean 0 and standard deviation σ\*√(2\*n). From the formula for the expectation of the absolute value of a normal random variable, we get:  
       
       = BE + (1/n) * σ\*√(2\*n) * √(2/π)  
       
       = BE + 2 * σ / √(n\*π) 

3. Define **BA** as the right-hand-side of the immediately above equality. Thus:

   - mean_absolute_error(mean_diff_ln) ≤ BA.

4. For a large sample size n, the above upper bound becomes, approximately:

    - BA  

      ≈ BE  

      = (λ<sub>1</sub>\+λ<sub>2</sub>)/2 * A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + (λ<sub>1</sub><sup>2</sup>+λ<sub>2</sub><sup>2</sup>)/2 * A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>)

**Confidence Interval**

1. mean_diff_ln is approximately normal, so its median and mean are approximately the same.

2. The median minimizes the mean absolute deviation. From *Point 1* and *Mean Absolute Error Point 3*:

   - E(|mean_diff_ln - E(mean_diff_ln))|)  

     ≈ E(|mean_diff_ln - median(mean_diff_ln))|)

     ≤ mean_absolute_error(mean_diff_ln)  

     ≤ BA

3. A normal distribution with a mean absolute deviation from its mean ≤ BA has stdev ≤ BA * √(π/2). Thus, from *Points 1 and 2*:

   - Stdev(mean_diff_ln) ≤ BA * √(π/2)

4. The 99% confidence interval around the mean for a normal distribution is stdev * 2.58. Thus:

   - |mean_diff_ln - E(mean_diff_ln))| ≤ BA * √(π/2) * 2.58  **[** _with 99% confidence_ **]**

5. 99% confidence bound for |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)|:

   - |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)|  

     ≤ |mean_diff_ln - E(mean_diff_ln)| + |E(mean_diff_ln) - ln(λ<sub>1</sub>/λ<sub>2</sub>)|  

     From *Point 4* and *Expectation Point 3*:

     ≤ BA * √(π/2) * 2.58 + BE **[** _with 99% confidence_ **]**  

     ≤ (BE + 2 * σ / √(n\*π)) * √(π/2) * 2.58 + BE  **[** _with 99% confidence_ **]**  

     = BE * (√(π/2) * 2.58 + 1) + 2 * σ / √(n\*π) * √(π/2) * 2.58  **[** _with 99% confidence_ **]**  

     = BE * (√(π/2) * 2.58 + 1) + √(2/n) * σ * 2.58  **[** _with 99% confidence_ **]**

6. Define **BC** as the right-hand-side of the immediately above equality. Thus:

   - |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)| ≤ BC  **[** _with 99% confidence_ **]**

7. For a large sample size n, the above confidence bound becomes, approximately:

   - BC  

     ≈ BE * (√(π/2) * 2.58 + 1) 

     = ((λ<sub>1</sub>\+λ<sub>2</sub>)/2 * A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + (λ<sub>1</sub><sup>2</sup>+λ<sub>2</sub><sup>2</sup>)/2 * A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>)) * (√(π/2) * 2.58 + 1)

**Reasonable Parameter Values**

Assume, without loss of generality, that the lowest value of α(t) is 1 = A<sub>L</sub>.

In practice, one could reasonably expect the time-dependent random noise parameters to be no greater (and likely substantially lower) than the values below:

- Rate of change of α(t) < 5% per second, which means A<sub>D</sub> < .00005 when time is measured in milliseconds and A<sub>D</sub> < .00000005 when time is measured in microseconds.
- A<sub>U</sub>/A<sub>L</sub> < 2
- σ < 0.3

From the above, given λ<sub>1</sub> and λ<sub>2</sub>, we can compute BC (assume a large sample size):

- λ<sub>1</sub> = λ<sub>2</sub> = 12 ⇒ BC < 0.003
- λ<sub>1</sub> = λ<sub>2</sub> = 120 ⇒ BC < 0.028

Notice that while the variability of the statistic mean_diff_ln depends on the sample size (exec_count), the BC upper bound does not, provided that the sample size is large enough (see *Confidence Interval Item 7*).

## Comparative Example

We will define an example of the above model and compare how `bench_diff` and the *traditional* benchmarking method fare with respect to the model. The example is admittedly contrived in order to facilitate approximate calculations and also to highlight the potential disparity of results between the two benchmarking methods.

**Model parameters**

- The two functions, **f<sub>1</sub>** and **f<sub>2</sub>** are identical (call it **f**), with **λ<sub>1</sub> = λ<sub>2</sub> = λ = 12 ms**.

- The number of executions of each function is **exec_count = 2500**. So, the total execution time, ignoring warm-up, is 1 minute.

- **α(t) = 1.5 + 1/2 * sin(t * 2*π / 60000)**, where **t** is the number of milliseconds elapsed since the start of the benchmark.  

  - α'(t)  

    = 1/2 * 2\*π / 60000 * cos(t * 2\*π / 60000)  

    = π / 60000 * cos(t * 2\*π / 60000)

  Therefore:

  - |α'(t)| ≤ π / 60000.

  And we have the following bounds for α(t):

  - A<sub>L</sub> = 1
  - A<sub>U</sub> = 2
  - A<sub>D</sub> = π / 60000

- **β(t)** has **σ = 0.28**.

  - Therefore, E(β(t)) = exp(σ<sup>2</sup>/2) ≈ 1.0400.

- The **baseline median latency** of f is the median latency when α(t) = 1. Its value is **λ = λ<sub>1</sub> = λ<sub>2</sub>** = 12 ms.

- The **baseline mean latency** of f is the mean latency when α(t) = 1. Its value  **μ = μ1 = μ2** = λ * exp(σ<sup>2</sup>/2) ≈ 12 * 1.0400 = 12.48 ms.

- The ratio of baseline medians is always equal to the ratio of baseline means even when f<sub>1</sub> ≠ f<sub>2</sub> because σ does not depend on f<sub>1</sub> or f<sub>2</sub>.

**`bench_diff` calculations**

- Given exec_count = 2500, the large sample size approximation for the BC bound can be used:

  - BC ≈ ((λ<sub>1</sub>\+λ<sub>2</sub>)/2 * A<sub>D</sub>\*exp(σ<sup>2</sup>/2) + (λ<sub>1</sub><sup>2</sup>+λ<sub>2</sub><sup>2</sup>)/2 * A<sub>D</sub><sup>2</sup>\*A<sub>U</sub><sup>2</sup>/A<sub>L</sub><sup>2</sup>\*exp(2\*σ<sup>2</sup>)) * (√(π/2) * 2.58 + 1)  

    ​      ≈ 0.00277

- From the model (*Confidence Interval Point 7*):

  - |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)| ≤ 0.00277  **[** _with 99% confidence_ **]**

- So, the multiplicative error on the estimate of λ<sub>1</sub>/λ<sub>2</sub> (= λ/λ = 1 in our example) is at most, with 99% confidence:

  - exp(0.00277) ≈ 1.00278, i.e., less than 3/10 of 1%

- Recall that the error bound does not depend on the number of executions, so it is the same with only half the number of executions. Also, given the high exec_count assumed, the `bench_diff` results during the first half should be very close to those obtained during the second half.

- If instead of λ = λ<sub>1</sub> = λ<sub>2</sub> = 12 ms and exec_count = 2500 we assume λ = λ<sub>1</sub> = λ<sub>2</sub> = 120 ms and exec_count = 250, then, with 99% confidence:

  - |mean_diff_ln - ln(λ<sub>1</sub>/λ<sub>2</sub>)| ≤ 0.0284
  - Multiplicative error ≤ 1.0289, i.e., about 3%.


***Traditional* method calculations**

- With the traditional method, we benchmark f<sub>1</sub> with exec_count = 2500 and then benchmark f<sub>2</sub> (which is the same as f<sub>1</sub> in our example) with exec_count = 2500.

- The first benchmark of f takes place during the first 30 seconds.

- The calculated sample mean latency is approximately:

  - μ * 1/30000 * **∫**<sub>0</sub><sup>30000</sup> α(t) dt  

    = μ / 30000 * (45000 + 1/2 * (-cos(30000 * 2\*π / 60000) + cos(0)) / (2\*π / 60000))  

    = 1.5\*μ + μ/30000 * 1/2 * (-cos(π) + cos(0)) / (2\*π / 60000)  

    = 1.5\*μ + μ * (-cos(π) + cos(0)) / (2\*π)  

    = 1.5\*μ + μ * (1 + 1) / (2\*π)  

    = μ * (1.5 + 1/π)  

    ≈ μ * 1.8183

- The second benchmark of f takes place during the second 30 seconds.

- The calculated sample mean latency is approximately:

  - μ * 1/30000 * **∫**<sub>30000</sub><sup>60000</sup> α(t) dt  

    = μ / 30000 * (45000 + 1/2 * (-cos(60000 * 2\*π / 60000) + cos(30000 * 2\*π / 60000)) / (2\*π / 60000))  

    = 1.5\*μ + μ/30000 * 1/2 * (-cos(2\*π) + cos(π)) / (2\*π / 60000)  

    = 1.5\*μ + μ * (-cos(2\*π) + cos(π)) / (2\*π)  

    = 1.5\*μ + μ * (-1 + -1) / (2\*π)  

    = μ * (1.5 - 1/π)  

    ≈ μ * 1.1817

- The estimated ratio of mean latencies is approximately 1.8183 / 1.1817 ≈ 1.5387, an estimating error of 54% in comparison with the baseline ratio of means μ1/μ2 = 1 (which is also equal to the baseline ratio of medians λ<sub>1</sub>/λ<sub>2</sub>).

- If instead of λ = λ<sub>1</sub> = λ<sub>2</sub> = 12 ms and exec_count = 2500 we assume λ = λ<sub>1</sub> = λ<sub>2</sub> = 120 ms and exec_count = 250, then the results are roughly the same, i.e., an estimating error of 54%.


**Closing comments for the example**

- In this example, the estimate of λ<sub>1</sub>/λ<sub>2</sub> (or equivalently μ1/μ2) provided by `bench_diff` is accurate to within 3/10 of 1% of the true value of 1. By contrast, the estimate of μ1/μ2 provided by the traditional method is inflated by 54%.
- If we "run" `bench_diff` during the model's first 30 seconds, we still get a very accurate estimate of λ<sub>1</sub>/λ<sub>2</sub> but the individual (sample mean) estimates of μ1 and μ2 are both close to μ * 1.8183, just as with the *traditional* method. Likewise, if we "run" `bench_diff` during the model's last 30 seconds, we still get a very accurate estimate of λ<sub>1</sub>/λ<sub>2</sub> but the individual (sample mean) estimates of μ1 and μ2 are both close to μ * 1.1817, just as with the *traditional* method.
- The key point of `bench_diff` is to repeatedly run both functions in close time proximity to each other so that the *ratios* of the two functions' latencies are close to the baseline even if the individual latencies themselves are distorted by time-dependent noise.
- If instead of λ = λ<sub>1</sub> = λ<sub>2</sub> = 12 ms and exec_count = 2500 we assume λ = λ<sub>1</sub> = λ<sub>2</sub> = 120 ms and exec_count = 250, then the accuracy of the `bench_diff` results is worse, but it is still much better (at around a 3% error) than the *traditional* result (which remains about the same at around a 54% error).

# Limitations

This library works well for latencies at the microseconds or milliseconds order of magnitude, but not for latencies at the nanoseconds order of magnitude.