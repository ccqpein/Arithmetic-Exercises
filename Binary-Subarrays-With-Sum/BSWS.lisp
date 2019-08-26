(defun num-subarrays-with-sum (l s)
  (let ((store-ind (append '(-1)
                           (loop
                              for e in l and ind from 0
                              when (= e 1)
                              collect ind)
                           (list (length l))))
        (sum 0))
    (if (= 0 s)
        (loop
           for ind from 0 to (- (length store-ind) 2)
           for a = (- (nth (+ 1 ind) store-ind) (nth ind store-ind) 1)
           do (setf sum (+ sum
                           (/ (* a (+ a 1)) 2)))
           finally (return-from num-subarrays-with-sum (the integer sum))))
    
    (loop
       for ind from 1 to (- (length store-ind) s 1)
       for x = (+ ind s -1)
       do (setf sum (+ sum
                       (* (- (nth ind store-ind) (nth (- ind 1) store-ind))
                          (- (nth (+ 1 x) store-ind) (nth x store-ind)))))
         )
    (the integer sum)
    ))
