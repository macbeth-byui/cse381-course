% CSE 381 REPL 3D
% MergeSort in a functional language

-module(main).
-export([test/0]).

mergesort(Data) when length(Data) =< 1 ->
    todo;

mergesort(Data) ->
    {L,R} = lists:split(length(Data) div 2, Data),
    todo.

merge([],[]) ->
    todo;

merge([],Right) ->
    todo;

merge(Left,[]) ->
    todo;

merge(L=[LFirst|LRest], R=[RFirst|RRest]) ->
    case LFirst =< RFirst of
        true -> todo;
        false -> todo
    end.

test() ->
    Result1 = mergesort([3,1,2,6,4,2,3,4,9,5,6,0]),
    io:format("~p~n",[Result1]).