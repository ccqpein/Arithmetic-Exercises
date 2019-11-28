(defun min-add-to-make-valid (s)
  (loop
     with cache = 0
     with result = 0
     for c across s
     do (ccase c
          (#\( (incf cache))
          (#\) (if (= 0 cache) (incf result) (decf cache)))
          )
     finally (return (+ result cache))))
