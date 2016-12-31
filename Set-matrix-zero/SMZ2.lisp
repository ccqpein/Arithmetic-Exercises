(defvar *m1 '((1 2 0 3 4 5) (0 4 5 6 7 8) (1 2 3 4 5 6)))
(defvar *m2 '((1 2 3 3 4 5) (0 4 5 6 7 8) (1 2 3 4 5 6)))
(defvar *m3 '((1 0 0 0 4 5) (0 4 5 6 7 8) (1 2 3 4 5 6)))

(defun set-matrix-zero (m)
  (let ((zRow '())
        (zCol '()))
    (loop with rowNum = 0
       for row in m
       do (loop for colNum from 0 to (1- (length row))
             for col in row
             do (if (= col 0)
                    (progn (push colNum zCol)
                           (push rowNum zRow)))
             finally (setf rowNum (1+ rowNum))))
    (delete-duplicates zRow)
    (delete-duplicates zCol)
    (print zRow) (print zCol)
    
    ))
