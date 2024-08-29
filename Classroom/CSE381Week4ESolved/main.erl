-module(main).
-export([test/0]).

quicksort([]) -> [];
quicksort([First|Rest]) -> 
    quicksort([N || N <- Rest, N < First]) 
    ++ [First] ++ 
    quicksort([N || N <- Rest, N >= First]).

test() ->
    Result = quicksort([3,1,2,6,4,2,3,4,9,5,6,0]),
    io:format("~p~n",[Result]).