# cs128honors-final-project
Group-4

Jason Witek (jrwitek2) Alexander Tseng (aetseng2)

Project Introduction:

We will be implementing a Fast Fourier Transform, an algorithm that breaks a sound wave down (or any vector) into its individual sine and cosine wave functions for each frequency. It is also the faster version of the Discrete Fourier Transform (DFT), which is a function that maps each frequency to a complex number that represents its magnitude and phase. It can be used to compress audio signals and apply noise filtering and other effects. We will specifically be implementing the "Cooley-Tukey" Algorithm.

Technical Overview:

Equations we will have to use:

DFT Transformation

$$
X[k] = \sum_{n=0}^{N-1} x[n] \cdot e^{-j2\pi kn/N}, \quad k = 0, 1, \ldots, N-1
$$

"Twiddle" Factors

$$
W_N^k = e^{-j2\pi k/N} = \cos\!\left(\frac{2\pi k}{N}\right) - j\sin\!\left(\frac{2\pi k}{N}\right)
$$

DFT Using Twiddle Factors

$$
X[k] = \sum_{n=0}^{N-1} x[n] \cdot W_N^{kn}
$$

Decimation in Time (DIT)

$$
X[k] = {\sum_{m=0}^{N/2-1} x[2m] \cdot W_N^{2mk}}
     + W_N^k {\sum_{m=0}^{N/2-1} x[2m+1] \cdot W_N^{2mk}}
$$

Twiddle Factor Reduction Identity

$$
W_N^{2mk} = W_{N/2}^{mk}
$$

Butterfly Operation

$$
X[k] = E[k] + W_N^k \cdot O[k]
$$
$$
X[k + N/2] = E[k] - W_N^k \cdot O[k]
$$

1. Use the num-complex crate to use complex numbers
2. Implement a recursive bit-reversal function that takes the vector and shuffles by reordering elements based on the binary representation of their index
3. Implement a function to pre-compute the Twiddle Factors using the equation above
4. First make a recursive version of FFT computation
5. Then make it iterative to save computing time

FFT Computation:

1. Calcualte Twiddle Factors (twiddle factor helper method)
2. Split the input into even / odd indexes (bit-reversal helper method)
3. Apply the Twiddle Factor Reduction Identity (esentially reusing already computed twiddle factors instead of recalculating)
4. Use the butterfly operations to combine the sums of the even and odd outputs

Please list what you plan to have finished by each checkpoint. These are meant to be goals to keep your project on track. We will NOT grade your checkpoints on how much you have completed, but on whether or not you’ve made some progress. 

Checkpoint 1 (4/13):
Have our Helper functions completed and num-complex crate correctly implemented (Steps 1-3)
Start working on the recursive approach to FFT Computation

Checkpoint 2 (4/27):
Have our Recursive implementaion finished and working correctly
Be a decent way into making our iterative approach, should be able to finish by Friday (5/1)

Possible Challenges:

- This is math I have never seen before, so I can forsee just some math errors popping up in the code, especially if we are not careful with syntax

- Never used a Rust crate before (besides in the lesson) and so that will take some adjusting to

- Also using GitHub routinely in our workflow is something we haven't done much before

References:
https://cp-algorithms.com/algebra/fft.html
If you’re basing the project off of some other work or if you received inspiration from an existing project, please list it here!

