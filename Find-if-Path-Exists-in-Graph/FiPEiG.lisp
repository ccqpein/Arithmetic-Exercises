(defun valid-path (steps edges source dest)
  (labels ((helper (table cache paths this end length)
             (if (gethash this cache) (return-from helper nil))
             (setf (gethash this paths) t)
             (if (= this end)
                 t
                 (find-if (lambda (v)
                            (if (= v end)
                                t
                                (cond ((= length 0) (remhash this paths) nil)
                                      ((gethash v paths) (remhash this paths) nil)
                                      (t (let ((x (helper table cache paths v end (1- length))))
                                           (setf (gethash this cache) x)
                                           x)))))
                          (gethash this table)))
             ))
    (let ((table (make-hash-table :test 'equal))
          (paths (make-hash-table :test 'equal))
          (cache (make-hash-table :test 'equal)))
      (loop for (a b) in edges
            do (push b (gethash a table))
            do (push a (gethash b table)))
      (helper table cache paths source dest steps))
    ))

(defun main ()
  (assert (valid-path 3 '((0 1) (1 2) (2 0)) 0 2))
  (assert (not (valid-path 6 '((0 1) (0 2) (3 5) (5 4) (4 3)) 0 5)))
  (assert (valid-path 1 '() 0 0))
  (assert (not
           (valid-path
            50
            '(
              (31 5)
              (10 46)
              (19 31)
              (5 1)
              (31 28)
              (28 29)
              (8 26)
              (13 23)
              (16 34)
              (30 1)
              (16 18)
              (33 46)
              (27 35)
              (2 25)
              (49 33)
              (44 19)
              (22 26)
              (30 13)
              (27 12)
              (8 16)
              (42 13)
              (18 3)
              (21 20)
              (2 17)
              (5 48)
              (41 37)
              (39 37)
              (2 11)
              (20 26)
              (19 43)
              (45 7)
              (0 21)
              (44 23)
              (2 39)
              (27 36)
              (41 48)
              (17 42)
              (40 32)
              (2 28)
              (35 38)
              (3 9)
              (41 30)
              (5 11)
              (24 22)
              (39 5)
              (40 31)
              (18 35)
              (23 39)
              (20 24)
              (45 1)
              )
            29
            46)))
  )
