(defun find-lucky (arr)
  (let ((cache (make-array 501
                           :element-type 'integer
                           :initial-element 0)))
    (loop
      with cache = (loop
                     for i in arr
                     do (incf (aref cache i))
                     finally (return (reverse cache)))      
      for ind from 500 downto 1
      for i across cache
      when (= ind i)
        do (return i)
      finally (return -1))))


(defun main ()
  (loop
    for case in '((2 2 3 4)
                 (1 2 2 3 3 3)
                 (2 2 2 3 3)
                 (5)
                 (7 7 7 7 7 7 7))
    for result in '(2 3 -1 -1 7)
    for answer = (find-lucky case)
    do (assert (= result answer)
               ()
               "failed test case ~a, expect ~a, but ~a"
               case result answer)))
