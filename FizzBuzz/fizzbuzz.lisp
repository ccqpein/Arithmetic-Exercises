(defun fizz-buzz (num)
  (do ((this 1 (1+ this))
       (three 1 (1+ three))
       (five 1 (1+ five))
       (result '())
       (flag nil nil))
      ((> this num) (reverse result))

    (when (= 3 three)
      (push "fizz" result)
      (setf three 0
            flag t))

    (when (= 5 five)
      (push "buzz" result)
      (setf five 0
            flag t))

    (when (not flag) 
        (push this result))
    ))

(defun make-fizz-buzz ()
  (let ((this 0)
        (three 0)
        (five 0)
        result)
    (lambda ()
      (setf result '())
      (incf this)
      (incf three)
      (incf five)
      (when (= 3 three)
        (push "fizz" result)
        (setf three 0))

      (when (= 5 five)
        (push "buzz" result)
        (setf five 0))

      (when (not result)
        (push this result))

      result
      )))
