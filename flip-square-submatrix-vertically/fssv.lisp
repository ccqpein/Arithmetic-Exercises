(defun reverse-submatrix (grid x y k)
  (append
   (subseq grid 0 x)
   (flip-helper (subseq grid x (+ x k)) y k)
   (subseq grid (+ x k))))

(defun flip-helper (grid y k)
  (let ((aa (loop for l in (reverse grid)
                  collect (subseq l y (+ y k)))))
    (loop for l in grid
          for ind from 0
          collect (append (subseq l 0 y) (nth ind aa) (subseq l (+ y k))))))


(defun main ()
  (assert (equal (reverse-submatrix
                  '((1 2 3 4) (5 6 7 8) (9 10 11 12) (13 14 15 16))
                  1 0 3)
                 '((1 2 3 4) (13 14 15 8) (9 10 11 12) (5 6 7 16))))

  (assert (equal (reverse-submatrix
                  '((3 4 2 3) (2 3 4 2))
                  0 2 2)
                 '((3 4 4 2) (2 3 2 3))))

  (assert (equal (reverse-submatrix
                  '((4 20 8 20) (2 16 3 12) (3 12 17 1) (3 13 2 13))
                  1 1 1)
                 '((4 20 8 20) (2 16 3 12) (3 12 17 1) (3 13 2 13)))))
