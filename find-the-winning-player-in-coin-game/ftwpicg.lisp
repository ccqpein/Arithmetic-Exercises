(defun losing-player (x y)
  (if (>= (/ y 4) x)
      (if (zerop (mod x 2)) "Bob" "Alice")
      (if (zerop (floor (mod (/ y 4) 2))) "Bob" "Alice")))
