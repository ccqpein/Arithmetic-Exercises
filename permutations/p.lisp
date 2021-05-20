(defun permutations (num)
  (if (= 1 (length num)) (return-from permutations (list (list (car num)))))
  (the (cons (cons fixnum *) *)
       (loop
             with result = '()
             for n from 0 below (length num)
             for all_rest = (multiple-value-bind (head tail)
                                (delete-x num n)
                              (append head tail))
             do (setf result (append result
                                     (loop for nn in (permutations all_rest)
                                           collect (append (list (nth n num)) nn))))
             finally (return result))))

(defun delete-x (num ind)
  (values (subseq num 0 ind)
          (subseq num (1+ ind))))
