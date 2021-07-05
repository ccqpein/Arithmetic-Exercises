(defun trap (height)
  (let ((left-max (loop
                    with key = 0
                    for n in height
                    for larger = (max n key)
                    do (setf key larger)
                    collect larger
                    ))
        (right-max (reverse
                    (loop
                      with key = 0
                      for n in (reverse height)
                      for larger = (max n key)
                      do (setf key larger)
                      collect larger
                      ))))
    (loop for l in left-max
          for r in right-max
          for h in height
          sum (- (min l r) h))
    ))

(defun main ()
  (assert (= 6 (trap '(0 1 0 2 1 0 1 3 2 1 2 1))))
  (assert (= 9 (trap '(4 2 0 3 2 5))))
  )
