(defun find-unsorted-subarray (nums)
  (let ((newNums (sort (copy-seq nums) #'<)))
    (loop for ind from 0 to (1- (length nums))
       count (/= (nth ind nums)
                 (nth ind newNums)))))

(defun main ()
  (let  ((test1 '(1 2 3))
         (test2 '(1))
         (test3 '(2 6 4 8 10 9 15))
         (test4 '(2 6 4 8 10 15 9))
         (test5 '())
         (test6 '(3 2 1)))
    (print (find-unsorted-subarray test1))
    (print (find-unsorted-subarray test2))
    (print (find-unsorted-subarray test3))
    (print (find-unsorted-subarray test4))
    (print (find-unsorted-subarray test5))
    (print (find-unsorted-subarray test6))
    ))