-module(main).
-export([test/0]).

djikstra(Graph, Start) -> 
    Shortest = dict:from_list(dict:fold(fun(Key, _, AccIn) -> case Key == Start of
                                                                  true -> [{Key, 0}|AccIn];
                                                                  false -> [{Key, 99999}|AccIn] 
                                                              end end, [], Graph)),
    Pred = dict:from_list(dict:fold(fun(Key, _, AccIn) -> [{Key, 99999}|AccIn] end, [], Graph)),
    Queue = sets:from_list(dict:fold(fun(Key, _, AccIn) -> [Key|AccIn] end, [], Graph)),
    djikstra(Graph,Shortest, Pred, Queue).

djikstra(Graph, Shortest, Pred, Queue) ->
    case sets:size(Queue) == 0 of
        true -> {Shortest, Pred};
        false -> {Vertex, Queue2} = find_smallest(Shortest, Queue),
                 {Shortest2, Pred2} = relax(Graph, Vertex, Shortest, Pred),
                 djikstra(Graph, Shortest2, Pred2, Queue2)
    end.

find_smallest(Shortest, Queue) ->
    {Vertex,_} = sets:fold(fun(Key, {Vertex, Dist}) -> {_,DistKey} = dict:find(Key, Shortest),
                                                     case DistKey < Dist of
                                                        true -> {Key, DistKey};
                                                        false -> {Vertex, Dist}
                                                     end
                         end, {none,999999}, Queue),
    Queue2 = sets:del_element(Vertex, Queue),
    {Vertex,Queue2}.

relax(Graph, Vertex, Shortest, Pred) ->
    {_,Edges} = dict:find(Vertex, Graph),
    relax_(Vertex, Shortest, Pred, Edges).

relax_(_, Shortest, Pred, []) ->
    {Shortest, Pred};

relax_(Vertex, Shortest, Pred, [{EdgeV,Weight}|Rest]) ->
    {_,CurrStartWeight} = dict:find(Vertex, Shortest),
    {_,CurrEndWeight} = dict:find(EdgeV, Shortest),
    case CurrStartWeight + Weight =< CurrEndWeight of
        true -> Shortest2 = dict:store(EdgeV, CurrStartWeight+Weight, Shortest),
                Pred2 = dict:store(EdgeV, Vertex, Pred);
        false -> Shortest2 = Shortest,
                 Pred2 = Pred
    end,
    relax_(Vertex, Shortest2, Pred2, Rest).

test() ->
    G = dict:from_list([{'s',[{'t',6},
                              {'y',4}]},
                        {'t',[{'x',3},
                              {'y',2}]},
                        {'x',[{'z',4}]},
                        {'y',[{'t',1},
                              {'x',9},
                              {'z',3}]},
                        {'z',[{'s',7},
                              {'x',5}]}]),
    {Shortest,Pred} = djikstra(G,'s'),
    io:format("~p~n~p~n~p~n",[G,Shortest,Pred]).