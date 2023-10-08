# Semi-Finalist Questions

## a. is_palindrome improvements
```python
def is_palindrone(s):
    r=""
    for c in s:
        r = c +r
    for x in range(0, len(s)):
        if s[x] == r[x]:
            x = True
        else:
            return False
    return x
```
The first way we could improve this code is in the reversing section. For loops in python are notoriously slow, while list comprehension is much faster.
Replacing lines 2-4 with `r=s[::-1]`, which is the short hand for reversing a list, would be an optimization. 
The second optimization that could be made is reducing the amount of characters checked. Currently, the `for x in range(0, len(s))` will iterate through every character in the given word.
This is not necessary. This behavior checks every character twice, which is unnecessary for a palindrome check. The better solution would be a two pointer solution.
This would have a left and right pointer, and walk from each end towards the middle, validating each iteration that the characters are equal to each other. These two optimizations would look like this:
```python
def is_palindrone(s):
    r = s[::-1]
    left, right = 0, len(s) - 1

    while left < right:
        if s[left] != r[right]:
            return False
        else:
            left += 1
            right -= 1
```
However, all of this can be simplified into a one line snippet if that would be more reusable, which would just combine a string comparison and a reversal:
```python
def is_palindrome(s):
    return s == s[::-1]
```
I prefer this version though, as it should be faster (string comparsions still require every single character to be checked).
It is also clearer with it's naming conventions, and adds type hints. These improvements allow for any reader to be able to understand what's happening.
The function name is also updated to fix the typo.
```python
def is_palindrome(input: str):
    reversed = input[::-1]
    left, right = 0, len(input) - 1

    while left < right:
        if input[left] != reversed[right]:
            return False
        else:
            left += 1
            right -= 1

    return True
```

## a. Qualifying Offer
Repo Link: https://github.com/Zenthial/phillies
Online REPL Link: https://www.rustexplorer.com/b/4lpqfp
