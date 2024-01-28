(defun find-longest-word (s dicts)
  (let ((dict (sort dicts (lambda (a b) (> (length a) (length b)))))
        (largest-len 0)
        flag
        re)
    (setf re
          (loop for d in dict
                when (and flag (< (length d) largest-len))
                  return re
                when (check s d)
                  do (setf flag t
                           largest-len (length d))
                  and collect d into re
                finally (return re)))

    (car (sort re #'lexi<))))

(defun check (a b)
  (loop with bi = 0
        for ai from 0 below (length a)
        when (char= (elt a ai) (elt b bi))
          do (incf bi)
          and if (= bi (length b))
                return t
        finally (return nil)))

(defun lexi-compare (s1 s2 &optional (f #'char<))
  (loop for c1 across s1
        for c2 across s2
        when (char/= c1 c2)
          return (if (funcall f c1 c2) s1 s2)
        finally (return s1)))

(defun lexi< (s1 s2)
  (string= s1 (lexi-compare s1 s2)))

(defun lexi<= (s1 s2)
  (string= s1 (lexi-compare s1 s2 #'char<=)))

(defun lexi> (s1 s2)
  (string= s1 (lexi-compare s1 s2 #'char>)))

(defun lexi>= (s1 s2)
  (string= s1 (lexi-compare s1 s2 #'char>=)))

(defun main ()
  (assert (string= "apple"
                   (find-longest-word "abpcplea"
                                      '("ale" "apple" "monkey" "plea"))))
  (assert (string= "a"
                   (find-longest-word "abpcplea"
                                      '("a" "b" "c"))))
  (assert (string= "ewaf"
                   (find-longest-word "aewfafwafjlwajflwajflwafj"
                                      '("apple" "ewaf" "awefawfwaf" "awef" "awefe" "ewafeffewafewf"))))
  )
