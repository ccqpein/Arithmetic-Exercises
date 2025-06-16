(defun greatest-letter (s)
  (let ((ffl (make-list 26 :initial-element nil))
        (ffu (make-list 26 :initial-element nil))
        (a (concatenate 'list "ABCDEFGHIJKLMNOPQRSTUVWXYZ")))
    (loop for c in (concatenate 'list s)
          do (if (upper-case-p c)
                 (setf (nth (char-upper-to-ind c) ffu) t)
                 (setf (nth (char-low-to-ind c) ffl) t)))
    (loop for i from 25 downto 0
          when (and (nth i ffl) (nth i ffu))
            return (string (nth i a))
          finally (return ""))))

(defun char-low-to-ind (c)
  (- (char-code c) (char-code #\a)))

(defun char-upper-to-ind (c)
  (- (char-code c) (char-code #\A)))

(defun main ()
  (assert (string= "E" (greatest-letter "lEeTcOdE")))
  (assert (string= "R" (greatest-letter "arRAzFif")))
  (assert (string= "" (greatest-letter "AbCdEfGhIjK"))))
