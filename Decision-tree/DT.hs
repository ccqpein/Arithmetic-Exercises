type PlayList = [Int]
data Leaf = Leaf {tag      :: String
                 ,playList :: PlayList
                 } deriving (Ord, Eq, Show, Read)
data DTree = DTree Leaf [DTree] deriving (Ord, Eq, Show, Read)

root = Leaf "root" [9,5]
overcast = Leaf "over" [4,0]
sunny = Leaf "sun" [2,3]
rain = Leaf "rain" [3,2]
windy = Leaf "Windy" [0,2]
no_windy = Leaf "NoWindy" [3,0]
hum_large_70 = Leaf ">70" [0,3]
hum_less_70 = Leaf "<70" [2,0]

b1 = DTree sunny [(DTree hum_large_70 [])
                 ,(DTree hum_less_70 [])]
b2 = DTree overcast []
b3 = DTree rain [(DTree windy [])
                ,(DTree no_windy [])]

a1 = DTree root [b1,b2,b3]
