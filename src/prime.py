# this is a coroutine example in python
def prime_coroutine():
    # first we create a list of prime numbers discovered so far
    primes = []
    # let's start with 2
    current = 2
    # loop forever
    while True:
        # check if the current number is prime
        found = True
        for prime in primes:
            # we wish to check all primes which are
            # smaller than the sqrt(current)
            if prime * prime > current:
                break
            # check primality
            if current % prime == 0:
                found = False
                break
        if found:
            primes.append(current)
            # this does the magic
            yield current
        # let's go with the next
        current += 1

# create the coroutine object
coroutine = prime_coroutine()

# each call will restart from the latest yield
print(next(coroutine))
print(next(coroutine))
print(next(coroutine))
