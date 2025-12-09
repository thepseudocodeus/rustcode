# HOW TO SOLVE IT

## Reframed Problem

What is the longest unique sequence of characters in an array of characters?

## Initial Approach

CREATE max_length: mutable i32 = 0
CREATE store: mutable vector

LOOP over each character: O(n)
    LOOP over each character in store: O(n)
        IF character in store == current character
            REMOVE first character from store
        END
    END
    ADD current character to store
    SET max_length = MAX(max_length, LENGTH(store))
END

RETURN max_length

## Understanding

*Given:*
    - a string `s`
    - string is greater than or equal to 0
    - string is less than or equal to 5 * 10^4

*Assumptions:*
    - a string is an array of characters
    - need to keep the current longest substring to check against
    - need to track the start pointer
    - need to track the end pointer

*Approach:*
    - start from the first character
    - move the end pointer to the right
    - check if start == end

*Unknowns:*
    - Are assumptions valid?
      - string = array of characters?
      - need a copy of current longest string
      - need start and end pointers?
    - What is the mathematical optimal solution?
        - What is the path of least action?
    - Are there edge cases I have not considered?
    - Is the optimal solution necessary?

## Improve

Solution beats 63% of submissions. Use a hashset to create an O(n) solution leveraging O(1) lookups and inherent uniqueness of data structure.

    - Current solution uses a dynamic array and loops twice which worst case can be O(n^2)
    - A hashmap could be used to gain O(1) lookups - unsure of benefit and optimal implementation in Rust
    - Hashmap solution is more complicated so for now I will keep the vector solution
