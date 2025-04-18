# A Model of Time-Dependent Random Noise

Following is a simple mathematical model of time-dependent random noise. This model provides additional corroboration for the `bench_diff` approach. Nonetheless, the model's fit with reality is not necessary to validate `bench_diff` as the test benchmarks discussed previously provide independent validation of the library.

## The Model

The first section defines the model. Subsequent sections develop estimates for some relevant statistics and parameters.

**Definitions and Assumptions**

This section defines the model per se.

1. Let **ln(x)** be the natural logarithm of **x**.
2. Let **L(f, t)** be the latency of function **f** at time **t**.
3. Let **λ1** be the baseline (ideal) latency of function **f1** in the absence of noise; respectively, **λ2** for **f2**.
4. Given a random variable **χ**, let **E(χ)** and **Stdev(χ)** be the expectation and standard deviation of **χ**, respectively.
5. Assume time-dependent noise is **ν(t) =<sub>def</sub> α(t) * β(t)**, where:
   - **α(t)** is a differentiable deterministic function of **t**, such that there are positive constants **A<sub>L</sub>**, **A<sub>U</sub>**, and **A<sub>D</sub>** for which **A<sub>L</sub> ≤ α(t) ≤ A<sub>U</sub>** and **|α'(t)| ≤ A<sub>D</sub>**, for all **t**.
   - **β(t)** is a family of mutually independent log-normal random variables indexed by **t**, such that  **E(ln(β(t))) = 0** and **Stdev(ln(β(t))) = σ**, where **σ** is a constant that does not depend on **t**.

6. Assume **L(f1, t) = λ1 * ν(t)** and **L(f2, t) = λ2 * ν(t)** for all **t**.  

   A couple of points to notice about this model:  

   - λ1 and λ2 can't be determined independently of α. If we multiply λ1 and λ2 by a constant factor r and divide α by the same factor, we end up with an exactly equivalent model. Nonetheless, the ratio λ1/λ2 remains the same regardless of the multiplier r, so the model is useful to reason about the ratio of the function latencies or, equivalently, the difference of the logarithm of the latencies.
   - As a log-normally distributed random variable, β(t) can take arbitrarily small positive values, which implies that the function latencies can take arbitrarily small values. That is clearly not 100% realistic. It is, however, a reasonable approximation of reality, especially when σ is close to 0, in which case the values of β(t) are clustered around 1.

7. Let **mean_diff_ln** be the sample mean difference between the natural logarithms of the observed latencies.

**Mean Absolute Error of mean_diff_ln**

This section calculates the expected absolute error of **mean_diff_ln** (see *Definition 7*) as an estimator of **ln(λ1/λ2)**.

1. When we measure **f1**'s latency at a time **t<sub>1</sub>**, getting **L(f1, t<sub>1</sub>)**, and right after we measure **f2**'s latency, the measurement for **f2** occurs at a time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**, where **Δt<sub>1</sub>** is <u>very close</u> to **L(f1, t<sub>1</sub>)**.

2. Substituting *Assumption 5* into *Assumption 6* for **f1** at time **t<sub>1</sub>** and **f2** at time **t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>**:

   - L(f1, t<sub>1</sub>) = λ1 * α(t<sub>1</sub>) * β(t<sub>1</sub>)
   - L(f2, t<sub>2</sub>) = λ2 * α(t<sub>1</sub> + Δt<sub>1</sub>) * β(t<sub>2</sub>)

3. Applying natural logarithms on *Point 2*:

   - ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))
   - ln(L(f2, t<sub>2</sub>)) = ln(λ2) + ln(α(t<sub>1</sub> + Δt<sub>1</sub>)) + ln(β(t<sub>2</sub>))

4. Based on the bound on **α'(t)** from *Assumption 5*:

   - ln(α(t<sub>1</sub> + Δt<sub>1</sub>))  

     By Lagrange's mean value theorem:

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>p</sub>)  **[** _for some t<sub>p</sub> between t<sub>1</sub> and Δt<sub>1</sub>_ **]**  

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>1</sub>) * α(t<sub>1</sub>)/α(t<sub>p</sub>)

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>)  **[** _where ε<sub>1</sub>(t<sub>1</sub>) =<sub>def</sub> α'(t<sub>p</sub>)\*α(t<sub>1</sub>)/α(t<sub>p</sub>), and thus |ε<sub>1</sub>(t<sub>1</sub>)| ≤ ε<sub>U</sub> =<sub>def</sub> A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub>_ **]**

5. Applying *Point 4* to *Point 3*:

   - ln(L(f1, t<sub>1</sub>)) = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))

   - ln(L(f2, t<sub>2</sub>))  

     = ln(λ2) + ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))   **[** _where |ε<sub>1</sub>(t<sub>1</sub>)| ≤ ε<sub>U</sub>_ **]**  

     Using *Point 1* to replace the Δt<sub>1</sub> value above:  

     = ln(λ2) + ln(α(t<sub>1</sub>)) + L(f1, t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))  
     
     Using *Assumption 5* and *Assumption 6* to expand L(f1, t<sub>1</sub>):  
     
     = ln(λ2) + ln(α(t<sub>1</sub>)) + (λ1 * α(t<sub>1</sub>) * β(t<sub>1</sub>)) * ε<sub>1</sub>(t<sub>1</sub>)/α(t<sub>1</sub>) + ln(β(t<sub>2</sub>))  
     
     = ln(λ2) + ln(α(t<sub>1</sub>)) + λ1 * β(t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>) + ln(β(t<sub>2</sub>))

6. Subtracting the second equation from the first in *Point 5*:

   - ln(L(f1, t<sub>1</sub>) - L(f2, t<sub>2</sub>))  

      = ln(λ1) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>)) - ln(λ2) - ln(α(t<sub>1</sub>)) - λ1 * β(t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>) - ln(β(t<sub>2</sub>))  

      = ln(λ1/λ2) - λ1 * β(t<sub>1</sub>) * ε<sub>1</sub>(t<sub>1</sub>) + ln(β(t<sub>1</sub>)) - ln(β(t<sub>2</sub>))

7. With `bench_diff`, measurements are done pairs, with one half of the pairs having **f1** followed by **f2** and the other half having **f2** followed by **f1**. The equation in *Point 6* above pertains to the first case. The analogous equation for the second case is:  

   - ln(L(f2, t<sub>2'</sub>) - ln(L(f1, t<sub>1'</sub>)) = ln(λ2/λ1) - λ2 * β(t<sub>2'</sub>) * ε<sub>2</sub>(t<sub>2'</sub>) + ln(β(t<sub>1'</sub>)) - ln(β(t<sub>2'</sub>))  

   Or, equivalently:

   - ln(L(f1, t<sub>1'</sub>)) - ln(L(f2, t<sub>2'</sub>) = ln(λ1/λ2) + λ2 * β(t<sub>2'</sub>) * ε<sub>2</sub>(t<sub>2'</sub>) - ln(β(t<sub>1'</sub>)) + ln(β(t<sub>2'</sub>))  **[** _where |ε<sub>2</sub>(t<sub>2'</sub>)| ≤ ε<sub>U</sub>_ **]**

8. Assuming the number of latency observations for each function is **n** and considering the two cases as described in *Point 7*, we can calculate the sample mean difference between the natural logarithms of the observed latencies (see *Definition 7*):  

   - mean_diff_ln = (1/n) * ∑<sub>i=1..n</sub> (ln(L(f1, t<sub>1,i</sub>)) - ln(L(f2, t<sub>2,i</sub>)))  

     = (1/n) * (∑<sub>i:odd</sub> (ln(L(f1, t<sub>1,i</sub>)) - ln(L(f2, t<sub>2,i</sub>))) + ∑<sub>i:even</sub> (ln(L(f1, t<sub>1,i</sub>)) - ln(L(f2, t<sub>2,i</sub>))))  

     = (1/n) * ∑<sub>i:odd</sub> (ln(λ1/λ2) - λ1 * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) +  
     
        (1/n) * ∑<sub>i:even</sub> (ln(λ1/λ2) + λ2 * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>) - ln(β(t<sub>1,i</sub>)) + ln(β(t<sub>2,i</sub>)))  
     
     = ln(λ1/λ2) +  
     
        (1/n) * ∑<sub>i:odd</sub> (-λ1 * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) +  
     
        (1/n) * ∑<sub>i:even</sub> (λ2 * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>) - ln(β(t<sub>1,i</sub>)) + ln(β(t<sub>2,i</sub>)))  

9. Thus:

   - **absolute_error(mean_diff_ln)** =<sub>def</sub> |mean_diff_ln - ln(λ1/λ2)|  

     = |(1/n) * ∑<sub>i:odd</sub> (-λ1 * β(t<sub>1,i</sub>) * ε<sub>1</sub>(t<sub>1,i</sub>) + ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) +  

     ​     (1/n) * ∑<sub>i:even</sub> (λ2 * β(t<sub>2,i</sub>) * ε<sub>2</sub>(t<sub>2,i</sub>) - ln(β(t<sub>1,i</sub>)) + ln(β(t<sub>2,i</sub>)))|  

     ≤ (1/n) * ∑<sub>i:odd</sub> λ1 * β(t<sub>1,i</sub>) * |ε<sub>1</sub>(t<sub>1,i</sub>)|  

        \+ (1/n) * ∑<sub>i:even</sub> λ2 * β(t<sub>2,i</sub>) * |ε<sub>2</sub>(t<sub>2,i</sub>)|  

        \+ (1/n) * |∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|

     ≤ (1/n) * ∑<sub>i:odd</sub> (λ1 * β(t<sub>1,i</sub>) * ε<sub>U</sub>  

        \+ (1/n) * ∑<sub>i:even</sub> (λ2 * β(t<sub>2,i</sub>) * ε<sub>U</sub>  

        \+ (1/n) * |∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|

     = (1/n)\*λ1\*ε<sub>U</sub> * ∑<sub>i:odd</sub> β(t<sub>1,i</sub>)  

        \+ (1/n)\*λ2\*ε<sub>U</sub> * ∑<sub>i:even</sub> β(t<sub>2,i</sub>)  
     
        \+ (1/n) * |∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|

10. Thus, defining the mean absolute error:

    - **mean_absolute_error(mean_diff_ln)** =<sub>def</sub> E(absolute_error(mean_diff_ln))  

        ≤ (1/n)\*λ1\*ε<sub>U</sub> * ∑<sub>i:odd</sub> E(β(t<sub>1,i</sub>))  

           \+ (1/n)\*λ2\*ε<sub>U</sub> * ∑<sub>i:even</sub> E(β(t<sub>2,i</sub>))  
        
           \+ (1/n) * E(|∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|)

11. Calculating the expectation of each term in *Point 10*:

     - E(β(t<sub>1,i</sub>)) = exp(σ<sup>2</sup>/2)  **[** _expectation of a log-normal distribution_ **]**

     - E(β(t<sub>2,i</sub>)) = exp(σ<sup>2</sup>/2)  **[** _same as above_ **]**

     - E(|∑<sub>i:odd</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>))) - ∑<sub>i:even</sub> (ln(β(t<sub>1,i</sub>)) - ln(β(t<sub>2,i</sub>)))|)  

         The term inside the absolute value is the sum of 2*n independently distributed normal random variables with mean 0 and standard deviation σ. It is, therefore, a normal distribution with mean 0 and standard deviation σ\*√(2\*n). From the formula for the expectation of the absolute value of a normal random variable, the above expected value is:  

         = σ\*√(2\*n) * √(2/π)

12. Substituting *Point 11* into *Point 10*:

      - mean_absolute_error(mean_diff_ln)  

          ≤ (1/n)\*λ1\*ε<sub>U</sub> * ∑<sub>i:odd</sub> exp(σ<sup>2</sup>/2)  

             \+ (1/n)\*λ2\*ε<sub>U</sub> * ∑<sub>i:even</sub> exp(σ<sup>2</sup>/2)  

             \+ (1/n) * σ\*√(2\*n) * √(2/π)

          = (1/2)\*λ1\*ε<sub>U</sub> * exp(σ<sup>2</sup>/2)  

             \+ (1/2)\*λ2\*ε<sub>U</sub> * exp(σ<sup>2</sup>/2)  
          
             \+ 2 * σ / √(n\*π)
          
          = (λ1+λ2)/2 * ε<sub>U</sub> * exp(σ<sup>2</sup>/2) + 2 * σ / √(n\*π)  
          
          = (λ1+λ2)/2 * A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub> * exp(σ<sup>2</sup>/2) + 2 * σ / √(n\*π)

13. Thus, for a large sample size n, the above upper bound on the mean absolute error of mean_diff_ln becomes:

     - **mean_absolute_error(mean_diff_ln) ≲ (λ1+λ2)/2 * A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub> * exp(σ<sup>2</sup>/2)**


**Mean Square Error of mean_diff_ln**

This section calculates the mean square error of **mean_diff_ln** (see *Definition 7*) as an estimator of **ln(λ1/λ2)**. In this section, we will assume, without loss of generality, that  **A<sub>L</sub> = 1** (refer to the comments under *Assumption 6*).

1. Defining the mean square error:

   - **square_error(mean_diff_ln)** =<sub>def</sub> (mean_diff_ln - ln(λ1/λ2))<sup>2</sup>  

     = ((mean_diff_ln - E(mean_diff_ln)) + (E(mean_diff_ln) - ln(λ1/λ2)))<sup>2</sup>  

     = (mean_diff_ln - E(mean_diff_ln))<sup>2</sup>  

        \+ (E(mean_diff_ln) - ln(λ1/λ2))<sup>2</sup>  

        \+ 2 * (mean_diff_ln - E(mean_diff_ln)) * (E(mean_diff_ln) - ln(λ1/λ2))  

2. Taking expectations on the above equation:

   - **mean_square_error(mean_diff_ln)** =<sub>def</sub> E(square_error(mean_diff_ln))  

     = E((mean_diff_ln - E(mean_diff_ln))<sup>2</sup>)  

        \+ (E(mean_diff_ln) - ln(λ1/λ2))<sup>2</sup>  

        \+ 2 * E(mean_diff_ln - E(mean_diff_ln)) * (E(mean_diff_ln) - ln(λ1/λ2))  

     Using the definition of variance **Var(X)** =<sub>def</sub> E((X - E(X))<sup>2</sup>) and the fact that E(mean_diff_ln - E(mean_diff_ln)) = 0:

     = Var(mean_diff_ln) + (E(mean_diff_ln) - ln(λ1/λ2))<sup>2</sup>  

     = Var(mean_diff_ln) + |E(mean_diff_ln) - ln(λ1/λ2)|<sup>2</sup>  

     ≤ Var(mean_diff_ln) + E(|mean_diff_ln - ln(λ1/λ2)|)<sup>2</sup>  

     By *Mean Absolute Error Point 12*:  

     ≤ Var(mean_diff_ln) + ((λ1+λ2)/2 * A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub> * exp(σ<sup>2</sup>/2) + 2 * σ / √(n\*π))<sup>2</sup>  

3. Calculating the variance of mean_diff_ln from *Mean Absolute Error Point 8*:

   - Var(mean_diff_ln)  

     = Var( (1/n) * (∑<sub>i:odd</sub> (ln(L(f1, t<sub>1,i</sub>)) - ln(L(f2, t<sub>2,i</sub>))) + ∑<sub>i:even</sub> (ln(L(f1, t<sub>1,i</sub>)) - ln(L(f2, t<sub>2,i</sub>)))) )  

     = (1/n<sup>2</sup>) * Var( ∑<sub>i:odd</sub> (ln(L(f1, t<sub>1,i</sub>)) - ln(L(f2, t<sub>2,i</sub>))) + ∑<sub>i:even</sub> (ln(L(f1, t<sub>1,i</sub>)) - ln(L(f2, t<sub>2,i</sub>))) )  

     = (1/n<sup>2</sup>) * Var(  

     ​     ∑<sub>i:odd</sub> (ln(λ1) + ln(α(t<sub>1,i</sub>)) + ln(β(t<sub>1,i</sub>)) - ln(λ2) - ln(α(t<sub>2,i</sub>)) - ln(β(t<sub>2,i</sub>)))  

        \+ ∑<sub>i:even</sub> (ln(λ1) + ln(α(t<sub>1,i</sub>)) + ln(β(t<sub>1,i</sub>)) - ln(λ2) - ln(α(t<sub>2,i</sub>)) - ln(β(t<sub>2,i</sub>))) )  

     = (1/n<sup>2</sup>) * ( ∑<sub>i:odd</sub> (Var(ln(α(t<sub>1,i</sub>))) + Var(ln(α(t<sub>2,i</sub>))) + Var(ln(β(t<sub>1,i</sub>))) + Var(ln(β(t<sub>2,i</sub>))) - 2 * Cov(ln(α(t<sub>1,i</sub>)), ln(α(t<sub>2,i</sub>))) - 2 * Cov(ln(β(t<sub>1,i</sub>)), ln(α(t<sub>2,i</sub>))))  

     ​                \+ ∑<sub>i:even</sub> (Var(ln(α(t<sub>1,i</sub>))) + Var(ln(α(t<sub>2,i</sub>))) + Var(ln(β(t<sub>1,i</sub>))) + Var(ln(β(t<sub>2,i</sub>))) - 2 * Cov(ln(α(t<sub>1,i</sub>)), ln(α(t<sub>2,i</sub>))) - 2 * Cov(ln(β(t<sub>2,i</sub>)), ln(α(t<sub>1,i</sub>)))) )  
     
     *Notice that other covariances are zero. For example, Cov(ln(α(t<sub>1,i</sub>)), ln(β(t<sub>1,i</sub>))) and Cov(ln(α(t<sub>1,i</sub>)), ln(β(t<sub>2,i</sub>))) are both zero when i is odd because in that case t<sub>1,i</sub> is determined before either β(t<sub>1,i</sub>) or β(t<sub>2,i</sub>) are generated and all β(t)s are mutually independent.*

4. Calculating values and bounds for the variance and covariance terms above:

   - Var(ln(α(t<sub>1,i</sub>))) ≤ (ln(A<sub>U</sub>))<sup>2</sup>
   - Var(ln(α(t<sub>2,i</sub>))) ≤ (ln(A<sub>U</sub>))<sup>2</sup>
   - Var(ln(β(t<sub>1,i</sub>))) = σ<sup>2</sup>
   - Var(ln(β(t<sub>2,i</sub>))) = σ<sup>2</sup>
   - |Cov(ln(α(t<sub>1,i</sub>)), ln(α(t<sub>2,i</sub>)))| ≤ (ln(A<sub>U</sub>))<sup>2</sup>
   - |Cov(ln(β(t<sub>1,i</sub>)), ln(α(t<sub>2,i</sub>)))| ≤ ln(A<sub>U</sub>) * σ / √(2\*π)
   - |Cov(ln(β(t<sub>2,i</sub>)), ln(α(t<sub>1,i</sub>)))| ≤ ln(A<sub>U</sub>) * σ / √(2\*π)

5. Substituting the right hand sides of *Point 4* above into *Point 3*:

   - Var(mean_diff_ln)  

     ≤ (1/n<sup>2</sup>) * ( ∑<sub>i:odd</sub> ((ln(A<sub>U</sub>))<sup>2</sup> + (ln(A<sub>U</sub>))<sup>2</sup> + σ<sup>2</sup> + σ<sup>2</sup> + 2 * (ln(A<sub>U</sub>))<sup>2</sup> + 2 * ln(A<sub>U</sub>) * σ / √(2\*π))  

     ​                \+ ∑<sub>i:even</sub> ((ln(A<sub>U</sub>))<sup>2</sup> + (ln(A<sub>U</sub>))<sup>2</sup> + σ<sup>2</sup> + σ<sup>2</sup> + 2 * (ln(A<sub>U</sub>))<sup>2</sup> + 2 * ln(A<sub>U</sub>) * σ / √(2\*π)) )  

     = (1/n) * ((ln(A<sub>U</sub>))<sup>2</sup> + (ln(A<sub>U</sub>))<sup>2</sup> + σ<sup>2</sup> + σ<sup>2</sup> + 2 * (ln(A<sub>U</sub>))<sup>2</sup> + 2 * ln(A<sub>U</sub>) * σ / √(2\*π))  

     = (2/n) * ((ln(A<sub>U</sub>))<sup>2</sup> + σ<sup>2</sup> + (ln(A<sub>U</sub>))<sup>2</sup> + ln(A<sub>U</sub>) * σ / √(2\*π))

6. Therefore, substituting *Point 5* into *Point 2*:

   - mean_square_error(mean_diff_ln)  

     ≤ (2/n) * ((ln(A<sub>U</sub>))<sup>2</sup> + σ<sup>2</sup> + (ln(A<sub>U</sub>))<sup>2</sup> + ln(A<sub>U</sub>) * σ / √(2\*π)) + ( (λ1+λ2)/2 * A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub> * exp(σ<sup>2</sup>/2) + 2 * σ / √(n\*π) )<sup>2</sup>

7. So, for a large sample size n, the above upper bound on the mean square error of mean_diff_ln becomes:  

   - **mean_square_error(mean_diff_ln) ≲ ((λ1+λ2)/2 * A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub> * exp(σ<sup>2</sup>/2))<sup>2</sup>**

   And:

   - √(mean_square_error(mean_diff_ln)) ≲ (λ1+λ2)/2 * A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub> * exp(σ<sup>2</sup>/2)

**Parameter Estimates**

Rate of change of α(t) < 20% per second, which means A<sub>D</sub> < .0002 when time is measured in milliseconds and A<sub>D</sub> < .0000002 when time is measured in microseconds.

In practice, A<sub>U</sub>/A<sub>L</sub> < 2 and σ < 0.3, so the product A<sub>U</sub>/A<sub>L</sub> * exp(σ<sup>2</sup>/2) < 2.1.

Notice that while the variability of the statistic mean_diff_ln depends on sample size (exec_count), this bias upper bound does not change.

## Comparative Example

We will define an example of the above model and compare how `bench_diff` and the *traditional* benchmarking method fare with respect to the model. The example is admittedly contrived in order to facilitate approximate calculations and also to highlight the potential disparity of results between the two benchmarking methods.

**Model parameters**

- The two functions, **f1** and **f2** are identical (call it **f**), with **λ1 = λ2 = λ = 12 ms**.

- The number of executions of each function is **exec_count = 2500**. So, the total execution time, ignoring warm-up, is 1 minute.

- **α(t) = 1 + 1/2 * sin(t * 2*π / 60000)**, where **t** is the number of milliseconds elapsed since the start of the benchmark.  

  - α'(t)  

    = 1/2 * 2\*π / 60000 * cos(t * 2\*π / 60000)  

    = π / 60000 * cos(t * 2\*π / 60000)

  Therefore:

  - |α'(t)| ≤ π / 60000.

  And we have the following bounds for α(t):

  - A<sub>L</sub> = 1/2
  - A<sub>U</sub> = 3/2
  - A<sub>D</sub> = π / 60000

- **β(t)** has **σ = 0.28**.

  - Therefore, E(β(t)) = exp(σ<sup>2</sup>/2) ≈ 1.0400.

- The **baseline median latency** of f is the median latency when α(t) = 1. Its value is **λ = λ1 = λ2** = 12 ms.

- The **baseline mean latency** of f is the mean latency when α(t) = 1. Its value  **μ = μ1 = μ2** = λ * exp(σ<sup>2</sup>/2) ≈ 12 * 1.0400 = 12.48 ms.

- The ratio of baseline medians is always equal to the ratio of baseline means even when f1 ≠ f2 because σ does not depend on f1 or f2.

**`bench_diff` calculations**

- Given exec_count = 2500, the noise contributed by β(t) is effectively eliminated.

- From the model (*Absolute Error 15*), the bias of E(mean_diff_ln) is at most:

  - A<sub>D</sub>\*A<sub>U</sub>/A<sub>L</sub> * (λ1+λ2)/2 * exp(σ<sup>2</sup>/2)  

    ≈ π / 60000 * 3 * (12 + 12)/2 * 1.0400  

    = π / 60000 * 3 * 12 * 1.0400  

    ≈ 0.001960

- So, the multiplicative bias on the estimate of λ1/λ2 (= λ/λ = 1 in our example) is at most:

  - exp(0.001960) ≈ 1.001962, i.e., less than 2/10 of 1%.

- Recall that bias does not depend on the number of executions, so it is the same with only half the number of executions. Also, given the high exec_count assumed, the `bench_diff` results during the first half should be very close to those obtained during the second half.

***Traditional* method calculations**

- With the traditional method, we benchmark f1 with exec_count = 2500 and then benchmark f2 (which is the same as f1 in our example) with exec_count = 2500.

- Given exec_count = 2500, the noise contributed by β(t) is effectively eliminated.

- The first benchmark of f takes place during the first 30 seconds.

- The calculated sample mean latency is approximately:

  - μ / 30000 * **∫**<sub>0</sub><sup>30000</sup> α(t) dt  

    = μ / 30000 * (30000 + 1/2 * (-cos(30000 * 2\*π / 60000) + cos(0)) / (2\*π / 60000))  

    = μ + μ/30000 * 1/2 * (-cos(π) + cos(0)) / (2\*π / 60000)  

    = μ + μ * (-cos(π) + cos(0)) / (2\*π)  

    = μ + μ * (1 + 1) / (2\*π)  

    = μ * (1 + 1/π)  

    ≈ μ * 1.3183

  - This is an upward error of more than 30%.

- The second benchmark of f takes place during the second 30 seconds.

- The calculated sample mean latency is approximately:

  - μ / 30000 * **∫**<sub>30000</sub><sup>60000</sup> α(t) dt  

    = μ / 30000 * (30000 + 1/2 * (-cos(60000 * 2\*π / 60000) + cos(30000 * 2\*π / 60000)) / (2\*π / 60000))  

    = μ + μ/30000 * 1/2 * (-cos(2\*π) + cos(π)) / (2\*π / 60000)  

    = μ + μ * (-cos(2\*π) + cos(π)) / (2\*π)  

    = μ + μ * (-1 + -1) / (2\*π)  

    = μ * (1 - 1/π)  

    ≈ μ * 0.6817

  - This is a downward error of more than 30%.

- The estimated ratio of mean latencies is approximately 1.3183 / 0.6817 ≈ 1.9338, an estimating error of 93% in comparison with the baseline ratio of means μ1/μ2 = 1 (which is also equal to the baseline ratio of medians λ1/λ2).


**Closing comments for the example**

- In this example, the estimate of λ1/λ2 (or equivalently μ1/μ2) provided by `bench_diff` is accurate to within 2/10 of 1% of the true value of 1. By contrast, the estimate of μ1/μ2 provided by the traditional method is off by 93%.
- If we "run" `bench_diff` during the model's first 30 seconds, we still get a very accurate estimate of λ1/λ2 but the individual (sample mean) estimates of μ1 and μ2 are both deflated by more than 30%, just like with the *traditional* method.
- Likewise, if we "run" `bench_diff` during the model's last 30 seconds, we still get a very accurate estimate of λ1/λ2 but the individual (sample mean) estimates of μ1 and μ2 are both inflated by more than 30%, just like with the *traditional* method.
- The key point of `bench_diff` is to repeatedly run both functions in close time proximity to each other so that the *ratios* of the two functions' latencies are close to the baseline even if the individual latencies themselves are distorted by time-dependent noise.

# Limitations

This library works well for latencies at the microseconds or milliseconds order of magnitude, but not for latencies at the nanoseconds order of magnitude.