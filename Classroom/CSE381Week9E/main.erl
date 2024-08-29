-module(main).
-export([test/0]).

euclid(A, 0) -> {A, 1, 0};
euclid(A, B) ->
    {G, I, J} = euclid(B, A rem B),
    {G, J, I - ((A div B)*J)}.

genPrivate(P, Q, E) ->
    R = (P-1) * (Q-1),
    {_, I, _} = euclid(E, R),
    case I < 0 of  % Fix Modulo Problem
        true -> (I + R) rem R;
        false -> I rem R
    end.

fast_mod_pow(_, 0, _) -> 
    todo;

fast_mod_pow(X, Y, N) when Y rem 2 == 0 ->
    todo;

fast_mod_pow(X, Y, N) ->
    todo.

encrypt(Data, E, N) ->
    todo.

decrypt(Data, D, N) ->
    todo.

test() ->
    P = 87178291199,
    Q = 22815088913,
    N = P * Q,
    E = 65537,
    D = genPrivate(87178291199, 22815088913, 65537),
    Data = 42,
    EData = encrypt(Data, E, N),
    io:format("Encrypted: ~p~n",[EData]),
    DData = decrypt(EData, D, N),
    io:format("Decrypted: ~p~n",[DData]).