(defun alphabet-board-path (target)
  (let ((board-raw '("abcde" "fghij" "klmno" "pqrst" "uvwxy" "z"))
        (board (make-hash-table :test 'equal)))
    ;; make hashtable
    (loop
       for r from 0 to (1- (length board-raw))
       for row = (coerce (nth r board-raw) 'list)
       do (loop
             for c from 0 to (1- (length row))
             do (setf (gethash (nth c row) board) (list r c))))
    
    (do* ((now '(0 0))
          (target (coerce target 'list) (cdr target))
          (result '())
          )
         ((not target) result)
      (let* ((tg (gethash (car target) board))
             (row-diff (- (car tg) (car now)))
             (col-diff (- (cadr tg) (cadr now))))
        (setf result
              (append result
                      (cond 
                        ((and (>= row-diff 0) (>= col-diff 0))
                         (append (loop
                                    repeat row-diff
                                    collect #\D)
                                 (loop
                                    repeat col-diff
                                    collect #\R)
                                 ))
                        ((and (>= row-diff 0) (< col-diff 0))
                         (append (loop
                                    repeat row-diff
                                    collect #\D)
                                 (loop
                                    repeat (abs col-diff)
                                    collect #\L)
                                 ))
                        ((and (< row-diff 0) (>= col-diff 0))
                         (append (loop
                                    repeat (abs row-diff)
                                    collect #\U)
                                 (loop
                                    repeat col-diff
                                    collect #\R)
                                 ))
                        ((and (< row-diff 0) (< col-diff 0))
                         (append (loop
                                    repeat (abs row-diff)
                                    collect #\U)
                                 (loop
                                    repeat (abs col-diff)
                                    collect #\L)
                                 ))
                        )
                      '(#\!)))
        
        (setf now tg)
        )
      )
    ))
