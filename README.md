# ransel - a library for succinct rank/select data structures

[Succinct data structures](https://en.wikipedia.org/wiki/Succinct_data_structure) offer a space-efficient way to store bitmaps (or sets of integers, depending on your point of view). They use a simple but powerful API based on 2 basic functions:

* *rank* which given a an element *x* from the domain of the set, returns the number of elements of the set which are less than *x*.
* *select* which given an index *i* from the range of the set, returns the *ith* smallest element of the set.

There are a number of derived operations, but they may all be expressed in terms of *rank* and *select*. In some cases, however, we do use special implementations which may have better runtime performance than the naive implementations.
