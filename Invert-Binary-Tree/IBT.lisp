(defun invert-tree (l)
  (if (eql nil l)
      nil
      (list (car l) (invert-tree (caddr l)) (invert-tree (cadr l)))))

(defun main ()
  (let ((a '(4
             (2
              (1)
              (3))
             (7
              (6)
              (9)))))
    (equal (invert-tree a)
           '(4
             (7
              (9)
              (6))
             (2
              (3)
              (1))
             ))
    (invert-tree a)))
