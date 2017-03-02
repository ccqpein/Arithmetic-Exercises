(defn sumRange [nl i j]
  (apply + (-> nl (subvec i (inc j)))))
