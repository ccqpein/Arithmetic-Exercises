(defun is-toeplitz-matrix (m)
  (let* ((row-num (array-dimension m 0))
         (col-num (array-dimension m 1))
         compare-list)
    ;;guard 
    (if (or (< row-num 2) (< col-num 2))
        (return-from is-toeplitz-matrix t)
        )

    (setf compare-list
          (append '((0 0))
                  (loop for c from 1 to (- col-num 2) collect (list 0 c))
                  (loop for r from 1 to (- row-num 2) collect (list r 0))))

    ;;inner function
    (labels ((match-each (m ind)
               (let ((first-v (aref m (car ind) (cadr ind)))
                     (row_ (1+ (car ind)))
                     (col_ (1+ (cadr ind))))
                 (loop
                    if (and (< row_ row-num)
                            (< col_ col-num))
                    do (progn 
                         (if (/= first-v (aref m row_ col_))
                             (return nil)
                             (progn (incf row_)
                                    (incf col_))))
                    else do (return t)))
               ))
      (loop for ele in compare-list
         when (not (match-each m ele))
         return (return nil)
         finally (return t)))
    ))

(defvar *a* (make-array '(3 4) :initial-contents '((1 2 3 4) (5 1 2 3) (9 5 1 2)))) ;; true
(defvar *b* (make-array '(2 2) :initial-contents '((1 2) (2 2)))) ;; false
(defvar *c* (make-array '(1 1) :initial-contents '((84)))) ;; true
(defvar *d* (make-array '(2 4) :initial-contents '((11 74 0 93) (40 11 74 7)))) ;; false

(print (is-toeplitz-matrix *a*))
(print (is-toeplitz-matrix *b*))
(print (is-toeplitz-matrix *c*))
(print (is-toeplitz-matrix *d*))
