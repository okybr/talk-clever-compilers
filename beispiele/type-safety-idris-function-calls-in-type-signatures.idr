isSingleton : Bool -> Type
isSingleton True = Nat
isSingleton False = List Nat

makeSingleton : (singleAsBool : Bool) -> isSingleton singleAsBool
makeSingleton True = 0
makeSingleton False = []

sum : (single : Bool) -> isSingleton single -> Nat
sum True x = x
sum False [] = 0
sum False (x :: xs) = x + sum False xs

main : IO ()
main = putStrLn "hi"
