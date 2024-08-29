-module(main).
-export([test/0]).

% List Comprehension:
% [expression || generator, condition]

quicksort([]) -> [];
quicksort([First|Rest]) -> 
    todo.

test() ->
    Result = quicksort([3,1,2,6,4,2,3,4,9,5,6,0]),
    io:format("~p~n",[Result]).