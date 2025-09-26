(defmacro alphab (c)
  (let* ((x (concatenate 'list "abcdefghijklmnopqrstuvwxyz"))
         (y (append (subseq x 1) (list (first x)))))
    `(case ,c
       ,@(loop for xx in x
               for yy in y
               collect (list xx yy)))))

(eval-when (:execute :load-toplevel :compile-toplevel)
  (defun one-round (s)
    (loop for c in (concatenate 'list s)
          collect c into res
          collect (alphab c) into res
          finally (return (concatenate 'string res)))))

(defmacro all (k)
  (loop with res = "a"
        for i from 1 to k
        do (setf res (one-round res))
        finally (return res)))

(defun main ()
  (assert (char= (nth 5 (all 5)) #\b))
  (assert (char= (nth 10 (all 10)) #\c))
  (assert (char= (nth 10 (all 30)) #\c))
  ;;(assert (char= (nth 500 (all 500)) #\h))
  ;;(assert (char= (nth 5000 (all 1000)) #\h))
  )

