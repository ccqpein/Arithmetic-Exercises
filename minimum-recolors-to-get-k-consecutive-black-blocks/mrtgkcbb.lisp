(defun minimum-recolors (blocks k)
  (if (< (length blocks) k) (return-from minimum-recolors 0))
  (let ((w-count (count-if (lambda (c) (char= c #\W)) (subseq blocks 0 k))))
    (loop with min = w-count
          for i from 1 to (- (length blocks) k)
          ;;for c = w-count
          do (if (char= #\W (elt blocks (1- i)))
                 (decf w-count))
             (if (char= #\W (elt blocks (+ i k -1)))
                 (incf w-count))
             (if (<= w-count min) (setf min w-count))
          finally (return min))))

(defun main ()
  (assert (= (minimum-recolors "WBBWWBBWBW" 7) 3))
  (assert (= (minimum-recolors "WBWBBBW" 2) 0))
  (assert (= (minimum-recolors "WWBBBWBBBBBWWBWWWB" 16) 6)))

