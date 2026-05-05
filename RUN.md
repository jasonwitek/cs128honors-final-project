How to run:

1. Clone the repository

2. Navigate to the project directory and build and run with:

cd cs182honors-final-project/fft

cargo run

Usage:

Now that the program is running, you will be prompted to enter in a array of numbers that you would like to perform the FFT on.
Enter in each entry seperated by a single space or each on their own line, when done, enter "done" on a new line.

eg.

1 2 3 4

done

OR

1

2

3

4

done

**Note that the array length MUST be a power of two (2,4,8,16,etc.) for the FFT algorithm to work.

The program will spit out the results of the FFT, one using a recursive approach, and the other using an iterative approach.

Both of the results should be the same.
