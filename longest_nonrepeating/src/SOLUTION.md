# HOW TO SOLVE IT

## Reframe
What is the longest sequence of characters in an array of characters that does not contain any of the previous characters?

## Approach
CREATE max_length: mutable i32 = 0
CREATE store: mutable vector

LOOP over each character
    LOOP over each character in store
        IF character in store == current character
            REMOVE first character from store
        END
    END
    ADD current character to store
    SET max_length = MAX(max_length, LENGTH(store))
END

RETURN max_length


## Understand
Given:
    - a string `s`
    - string is greater than or equal to 0
    - string is less than or equal to 5 * 10^4

Assumptions:
    - a string is an array of characters
    - need to keep the current longest substring to check against
    - need to track the start pointer
    - need to track the end pointer

Approach:
    - start from the first character
    - move the end pointer to the right
    - check if start == end


Unknowns:
  - Are my assumptions correct in Rust?
    - string = array of characters?
    - need a copy of current longest string
    - need start and end pointers?
