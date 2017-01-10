(defn diff-ham  [x y]
  (let [re 0
        xin x
        yin y]
    (while (or (pos? @xin) (pos? @yin))
      (do 
        (cond (not= (mod xin 2) (mod yin 2))
              (swap! re inc))
        (update-in xin - (mod xin 2))
        (update-in xin / 2)
        (update-in yin - (mod yin 2))
        (update-in yin / 2)))
    re))


















