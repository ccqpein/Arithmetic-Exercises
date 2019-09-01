(defun shifting-lettert (str l)
  (let ((sum 0))
    (concatenate
     'string
     (reverse
      (loop
         for c across (reverse str)
         for i in (reverse l)
         for this-charcode = (char-code c)
         for this-one = (let ((_ (setf sum (mod (+ sum i) 26))) ;; trick: do something in let block
                              (temp (+ this-charcode sum)))
                          (if (> temp 122)
                              (- temp 26)
                              temp))
         collect (code-char this-one)
           )))))

(shifting-lettert "abc" '(3 5 9))
(shifting-lettert "bad" '(10 20 30))
(shifting-lettert "mkgfzkkuxownxvfvxasy"
                  '(505870226 37526072 66740649 24336793 32917782 11122363 67754492 95798950 81520022 84110326 37742843 75267355 56903962 48291585 19054234 67541837 622939912 16899933 83296461 36563513))
