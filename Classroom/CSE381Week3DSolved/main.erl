% CSE 381 REPL 3D Solution
% MergeSort in a functional language

-module(main).
-export([test/0]).

mergesort(Data) when length(Data) =< 1 ->
    Data;

mergesort(Data) ->
    {L,R} = lists:split(length(Data) div 2, Data),
    merge(mergesort(L), mergesort(R)).

merge([],[]) ->
    [];

merge([],Right) ->
    Right;

merge(Left,[]) ->
    Left;

merge(L=[LFirst|LRest], R=[RFirst|RRest]) ->
    case LFirst =< RFirst of
        true -> [LFirst|merge(LRest,R)];
        false -> [RFirst|merge(L,RRest)]
    end.

test() ->
    Result1 = mergesort([3,1,2,6,4,2,3,4,9,5,6,0]),
    io:format("~p~n",[Result1]).