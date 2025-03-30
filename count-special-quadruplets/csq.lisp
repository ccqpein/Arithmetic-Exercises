(defun count-quadruplets (nums)
  (labels ((helper (nums left already)
             (cond ((< (length nums) left)
                    0)
                   ((= 1 left)
                    (count already nums))
                   (t (loop for i from 0 below (length nums)
                            sum (helper (subseq nums (1+ i)) (1- left) (+ already (nth i nums))))))))
    (helper nums 4 0))) 


(assert (= 1 (count-quadruplets '(1 2 3 6))))
(assert (= 0 (count-quadruplets '(3 3 6 4 5))))
(assert (= 4 (count-quadruplets '(1 1 1 3 5))))
