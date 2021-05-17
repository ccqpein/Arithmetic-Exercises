(defun is-power-of-4 (n)
  (and (> n 0)
       (= 0 (logand n (1- n)))
       (= n (logand n #x55555555))))
