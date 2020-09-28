(defun num-subseq (nums target)
  (let* ((nums (sort nums #'<))
         (head-ind 0)
         (tail-ind (or (position-if (lambda (x)
                                      (<= (+ x (car nums)) target))
                                    nums
                                    :from-end t)
                       0))
         (cache (loop
                  for i from 0 to tail-ind
                  collect (mod (expt 2 i) 1000000007))))

    (do ((result 0))
        ((> head-ind tail-ind) result)
      (do ()
          ((<= (+ (nth tail-ind nums) (nth head-ind nums)) target))
        (if (or (= 0 tail-ind) (<= tail-ind head-ind))
            (return-from num-subseq result))
        
        (decf tail-ind))
      
      (setf result (+ (mod result 1000000007)
                      (nth (- tail-ind head-ind) cache)
                      (if (<= (* 2 (nth 0 nums)) target) 1 0)
                      -1))
      
      (incf head-ind)
      )))

(defun main ()
  (assert (= 0 (num-subseq '(1) 1)))
  (assert (= 4 (num-subseq '(3 5 6 7) 9)))
  (assert (= 6 (num-subseq '(3 3 6 8) 10)))
  (assert (= 127 (num-subseq '(5 2 4 1 7 6 8) 16)))
  (assert (= 272187084 (num-subseq '(14 4 6 6 20 8 5 6 8 12 6 10 14 9 17 16 9 7 14 11 14 15 13 11 10 18 13 17 17 14 17 7 9 5 10 13 8 5 18 20 7 5 5 15 19 14) 22))))
