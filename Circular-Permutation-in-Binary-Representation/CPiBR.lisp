(defun circular-permutation (n start)
  (loop
    for i from 0 to (1- (ash 1 n)) ;; left shift
    collect (logxor (logxor start i) (ash i -1))))
