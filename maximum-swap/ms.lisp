(defun number-to-digits (num)
  (reverse
   (loop while (> num 0)
         collect (mod num 10)
         do (setf num (floor (/ num 10))))))

(defun digits-to-number (digits)
  (let ((i 1))
    (reduce (lambda (a d)
              (prog1
                  (+ d (* a i))
                (setf i (* 10 i))))
            digits
            :from-end t :initial-value 0)))

(defun maximum-swap (num)
  (let* ((rdigits (reverse (number-to-digits num)))
         (st (make-list (length rdigits) :initial-element -1)))
    (loop with i = 0
          for j from 0 below (length rdigits)
          if (> (nth i rdigits) (nth j rdigits))
            do (setf (nth j st) i)
          else if (< (nth i rdigits) (nth j rdigits))
                 do (setf i j))

    (loop for j from (1- (length rdigits)) downto 0
          when (>= (nth j st) 0)
            do (let ((a (nth (nth j st) rdigits)))
                 (setf (nth (nth j st) rdigits) (nth j rdigits)
                       (nth j rdigits) a)
                 (return-from maximum-swap (digits-to-number (reverse rdigits)))))
    ))

(defun main ())
