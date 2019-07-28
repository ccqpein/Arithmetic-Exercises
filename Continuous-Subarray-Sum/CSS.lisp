(defun check-subarray-sum (nums k)
  (if (= 0 (length nums)) (return-from check-subarray-sum nil))
  (loop
     for count from 1 to (length nums)
     do (loop  for start from 0 to (- (length nums) 1 count)
           do (format t "count: ~a, start: ~a~%" count start)
           do (case k
                (0
                 (if (= 0 (apply #'+ (subseq nums start (+ start count))))
                     (return-from check-subarray-sum t)))
                (otherwise
                 (if (= 0 (mod (apply #'+ (subseq nums start (+ start count))) (abs k)))
                     (return-from check-subarray-sum t))))))
  nil)


(check-subarray-sum '(23 2 4 6 7) 6) ;;t
(check-subarray-sum '(23 2 6 4 7) -6) ;;t
(check-subarray-sum '(0 0) -1) ;;t
(check-subarray-sum '(1 2 3) 4) ;;nil
(check-subarray-sum '(0 0) 0) ;;t
(check-subarray-sum '(0) 0) ;;nil
