(def chars [\1 \2 \3 \4 \5 \6 \7 \8 \9 \0])
(def ints [1 2 3 4 5 6 7 8 9 0])
(def dict (merge (zipmap chars ints)
                 (zipmap ints chars)))

