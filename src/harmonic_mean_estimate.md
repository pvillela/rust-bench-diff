**Harmonic Mean Estimate**

The time-dependent random noise model motivates another estimate for ln(λ<sub>1</sub>/λ<sub>2</sub>).

1. When f<sub>1</sub> is executed at time t<sub>1</sub> and f<sub>2</sub> is executed right after at time t<sub>2</sub> = t<sub>1</sub> + Δt<sub>1</sub>, using *Assumptions 5, 6, 8*:

   - L(f<sub>1</sub>, t<sub>1</sub>) = λ<sub>1</sub> * α(t<sub>1</sub>) * β(t<sub>1</sub>)
   - L(f<sub>2</sub>, t<sub>2</sub>) = λ<sub>2</sub> * α(t<sub>1</sub> + Δt<sub>1</sub>) * β(t<sub>2</sub>)  **[** _where Δt<sub>1</sub> = L(f<sub>1</sub>, t<sub>1</sub>)_ **]**

2. Applying natural logarithms on *Point 2*:

   - ln(L(f<sub>1</sub>, t<sub>1</sub>)) = ln(λ<sub>1</sub>) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))
   - ln(L(f<sub>2</sub>, t<sub>2</sub>)) = ln(λ<sub>2</sub>) + ln(α(t<sub>1</sub> + Δt<sub>1</sub>)) + ln(β(t<sub>2</sub>))

3. By Lagrange's mean value theorem for ln(α(t)) and the bounds on α(t) and α'(t) from *Assumption 5*:

   - ln(α(t<sub>1</sub> + Δt<sub>1</sub>)) 

     = ln(α(t<sub>1</sub>)) + Δt<sub>1</sub> * α'(t<sub>p</sub>)/α(t<sub>p</sub>)  **[** _for some t<sub>p</sub> between t<sub>1</sub> and t<sub>1</sub> + Δt<sub>1</sub>_ **]**  

     = ln(α(t<sub>1</sub>)) + L(f<sub>1</sub>, t<sub>1</sub>) * α'(t<sub>p</sub>)/α(t<sub>p</sub>)

4. Substituting *Point 3* into the second equation of *Point 2*:

   - ln(L(f<sub>1</sub>, t<sub>1</sub>)) = ln(λ<sub>1</sub>) + ln(α(t<sub>1</sub>)) + ln(β(t<sub>1</sub>))
   - ln(L(f<sub>2</sub>, t<sub>2</sub>)) = ln(λ<sub>2</sub>) + ln(α(t<sub>1</sub>)) + L(f<sub>1</sub>, t<sub>1</sub>) * α'(t<sub>p</sub>)/α(t<sub>p</sub>) + ln(β(t<sub>2</sub>))

5. Subtracting the second equation from the first in the point immediately above:

   - ln(L(f<sub>1</sub>, t<sub>1</sub>)) - ln(L(f<sub>2</sub>, t<sub>2</sub>))  

     = ln(λ<sub>1</sub>) - ln(λ<sub>2</sub>) - L(f<sub>1</sub>, t<sub>1</sub>) * α'(t<sub>p</sub>)/α(t<sub>p</sub>) + ln(β(t<sub>1</sub>)) - ln(β(t<sub>2</sub>))  

     = ln(λ<sub>1</sub>/λ<sub>2</sub>) - L(f<sub>1</sub>, t<sub>1</sub>) * α'(t<sub>p</sub>)/α(t<sub>p</sub>) + ln(β(t<sub>1</sub>)) - ln(β(t<sub>2</sub>))

   Thus:

   - ln(L(f<sub>1</sub>, t<sub>1</sub>)) - ln(L(f<sub>2</sub>, t<sub>2</sub>)) - ln(λ<sub>1</sub>/λ<sub>2</sub>) = -L(f<sub>1</sub>, t<sub>1</sub>) * α'(t<sub>p</sub>)/α(t<sub>p</sub>) + ln(β(t<sub>1</sub>)) - ln(β(t<sub>2</sub>))

6. Dividing both sides of the immediately above equation by L(f<sub>1</sub>, t<sub>1</sub>):

   - (ln(L(f<sub>1</sub>, t<sub>1</sub>)) - ln(L(f<sub>2</sub>, t<sub>2</sub>))) / L(f<sub>1</sub>, t<sub>1</sub>) - ln(λ<sub>1</sub>/λ<sub>2</sub>) / L(f<sub>1</sub>, t<sub>1</sub>)  

     = -α'(t<sub>p</sub>)/α(t<sub>p</sub>) / L(f<sub>1</sub>, t<sub>1</sub>) + ln(β(t<sub>1</sub>)) / L(f<sub>1</sub>, t<sub>1</sub>) - ln(β(t<sub>2</sub>)) / L(f<sub>1</sub>, t<sub>1</sub>)

7. With `bench_diff`, measurements are done pairs, with one half of the pairs having **f<sub>1</sub>** followed by **f<sub>2</sub>** and the other half having **f<sub>2</sub>** followed by **f<sub>1</sub>**. The equation in *Point 6* above pertains to the first case. The analogous equation for the second case is:

   - (ln(L(f<sub>2</sub>, t<sub>2'</sub>)) - ln(L(f<sub>1</sub>, t<sub>1'</sub>))) / L(f<sub>2</sub>, t<sub>2'</sub>) - ln(λ<sub>2</sub>/λ<sub>1</sub>) / L(f<sub>2</sub>, t<sub>2'</sub>)  

     = -α'(t<sub>p'</sub>)/α(t<sub>p'</sub>) / L(f<sub>2</sub>, t<sub>2'</sub>) + ln(β(t<sub>2'</sub>)) / L(f<sub>2</sub>, t<sub>2'</sub>) - ln(β(t<sub>1'</sub>)) / L(f<sub>2</sub>, t<sub>2'</sub>)

   Or, equivalently:

   - (ln(L(f<sub>1</sub>, t<sub>1'</sub>)) - ln(L(f<sub>2</sub>, t<sub>2'</sub>))) / L(f<sub>2</sub>, t<sub>2'</sub>) - ln(λ<sub>1</sub>/λ<sub>2</sub>) / L(f<sub>2</sub>, t<sub>2'</sub>)  

     = α'(t<sub>p'</sub>)/α(t<sub>p'</sub>) / L(f<sub>2</sub>, t<sub>2'</sub>) - ln(β(t<sub>2'</sub>)) / L(f<sub>2</sub>, t<sub>2'</sub>) + ln(β(t<sub>1'</sub>)) / L(f<sub>2</sub>, t<sub>2'</sub>)

8. Assuming the number of latency observations for each function is **n** and considering the two cases as described in *Point 7*, we can calculate the sample means for the equations in *Points 6 and 7*:

   - (1/(n/2)) * ∑<sub>i:odd</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>1</sub>, t<sub>1,i</sub>) - (1/(n/2)) * ∑<sub>i:odd</sub> ln(λ<sub>1</sub>/λ<sub>2</sub>) / L(f<sub>1</sub>, t<sub>1,i</sub>)  

     = -(1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>1</sub>, t<sub>1,i</sub>) + (1/(n/2)) * ∑<sub>i:odd</sub> ln(β(t<sub>1,i</sub>)) / L(f<sub>1</sub>, t<sub>1,i</sub>) - (1/(n/2)) * ∑<sub>i:odd</sub> ln(β(t<sub>2,i</sub>)) / L(f<sub>1</sub>, t<sub>1,i</sub>)

   - (1/(n/2)) * ∑<sub>i:even</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>2</sub>, t<sub>2,i</sub>) - (1/(n/2)) * ∑<sub>i:even</sub> ln(λ<sub>1</sub>/λ<sub>2</sub>) / L(f<sub>2</sub>, t<sub>2,i</sub>)  

     = (1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>2</sub>, t<sub>2,i</sub>) - (1/(n/2)) * ∑<sub>i:odd</sub> ln(β(t<sub>1,i</sub>)) / L(f<sub>2</sub>, t<sub>2,i</sub>) + (1/(n/2)) * ∑<sub>i:odd</sub> ln(β(t<sub>2,i</sub>)) / L(f<sub>2</sub>, t<sub>2,i</sub>)

9. In *Point 8*, for large n, the sample means for the terms with an ln(β(t)) on the numerator converge to 0 because ln(β(t)) is normally distributed with mean 0. Therefore:

   - (1/(n/2)) * ∑<sub>i:odd</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>1</sub>, t<sub>1,i</sub>) - (1/(n/2)) * ∑<sub>i:odd</sub> ln(λ<sub>1</sub>/λ<sub>2</sub>) / L(f<sub>1</sub>, t<sub>1,i</sub>)  

     ≈ -(1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>1</sub>, t<sub>1,i</sub>)

   - (1/(n/2)) * ∑<sub>i:even</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>2</sub>, t<sub>2,i</sub>) - (1/(n/2)) * ∑<sub>i:even</sub> ln(λ<sub>1</sub>/λ<sub>2</sub>) / L(f<sub>2</sub>, t<sub>2,i</sub>) 

     ≈ (1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>2</sub>, t<sub>2,i</sub>)

   Or, equivalently:  

   - (1/(n/2)) * ∑<sub>i:odd</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>1</sub>, t<sub>1,i</sub>) - ln(λ<sub>1</sub>/λ<sub>2</sub>) * (1/(n/2)) * ∑<sub>i:odd</sub> 1/L(f<sub>1</sub>, t<sub>1,i</sub>)  

     ≈ -(1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>1</sub>, t<sub>1,i</sub>)

   - (1/(n/2)) * ∑<sub>i:even</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>2</sub>, t<sub>2,i</sub>) - ln(λ<sub>1</sub>/λ<sub>2</sub>) * (1/(n/2)) * ∑<sub>i:even</sub> 1/L(f<sub>2</sub>, t<sub>2,i</sub>) 

     ≈ (1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>2</sub>, t<sub>2,i</sub>)

10. Let:

    - **LNH<sub>odd</sub>** =<sub>def</sub> (1/(n/2)) * ∑<sub>i:odd</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>1</sub>, t<sub>1,i</sub>)
    - **LNH<sub>even</sub>** =<sub>def</sub> (1/(n/2)) * ∑<sub>i:even</sub> (ln(L(f<sub>1</sub>, t<sub>1,i</sub>)) - ln(L(f<sub>2</sub>, t<sub>2,i</sub>))) / L(f<sub>2</sub>, t<sub>2,i</sub>)
    - **HM<sub>odd</sub>** =<sub>def</sub> (1/(n/2)) * ∑<sub>i:odd</sub> 1/L(f<sub>1</sub>, t<sub>1,i</sub>)
    - **HM<sub>even</sub>** =<sub>def</sub> (1/(n/2)) * ∑<sub>i:even</sub> 1/L(f<sub>2</sub>, t<sub>2,i</sub>)
    - Residue<sub>odd</sub> =<sub>def</sub> -(1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>1</sub>, t<sub>1,i</sub>)
    - Residue<sub>even</sub> =<sub>def</sub> (1/(n/2)) * ∑<sub>i:odd</sub> α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) / L(f<sub>2</sub>, t<sub>2,i</sub>)

11. Substituting *Point 10* into *Point 9*:

    - LNH<sub>odd</sub> - ln(λ<sub>1</sub>/λ<sub>2</sub>) * HM<sub>odd</sub> ≈ Residue<sub>odd</sub>
    - LNH<sub>even</sub> - ln(λ<sub>1</sub>/λ<sub>2</sub>) * HM<sub>even</sub> = Residue<sub>even</sub>

    Or, equivalently:  

    - LNH<sub>odd</sub> / HM<sub>odd</sub> - ln(λ<sub>1</sub>/λ<sub>2</sub>) ≈ Residue<sub>odd</sub> / HM<sub>odd</sub>
    - LNH<sub>even</sub> / HM<sub>even</sub> - ln(λ<sub>1</sub>/λ<sub>2</sub>) ≈ Residue<sub>even</sub> / HM<sub>even</sub>

12. We assume Residue<sub>odd</sub> / HM<sub>odd</sub> and Residue<sub>even</sub> / HM<sub>even</sub> are both small because:

    - They are averages of α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>).
    - Each term α'(t<sub>p,i</sub>)/α(t<sub>p,i</sub>) by itself is small due to the bounds on α(t) and α'(t).
    - The average of these terms is even smaller as the sign of α'(t<sub>p,i</sub>) can be positive or negative (α(t<sub>p,i</sub>) is always positive).

13. Adding the two equations in *Point 11*, using the assumptions in *Point 12*:

    - LNH<sub>odd</sub> / HM<sub>odd</sub> + LNH<sub>even</sub> / HM<sub>even</sub> - 2 * ln(λ<sub>1</sub>/λ<sub>2</sub>)  

      ≈ Residue<sub>odd</sub> / HM<sub>odd</sub> + Residue<sub>even</sub> / HM<sub>even</sub>  

      ≈ 0

    Or, equivalently:  

    - (LNH<sub>odd</sub> / HM<sub>odd</sub> + LNH<sub>even</sub> / HM<sub>even</sub>) / 2 ≈ ln(λ<sub>1</sub>/λ<sub>2</sub>)

14. Therefore, from *Point 13*, (LNH<sub>odd</sub> / HM<sub>odd</sub> + LNH<sub>even</sub> / HM<sub>even</sub>) / 2 is an estimator of ln(λ<sub>1</sub>/λ<sub>2</sub>) which may have a lower error than mean_diff_ln.


