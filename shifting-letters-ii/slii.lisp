;; shifts is ((0 0 0)...)
(defun shifting-letters (s shifts)
  (let ((table (make-hash-table :test 'equal))
        (bucket (make-list (length s) :initial-element 0))
        (s (concatenate 'list s)))
    (loop for shift in shifts
          do (if (= 0 (car (last shift)))
                 (decf (gethash (butlast shift) table 0))
                 (incf (gethash (butlast shift) table 0))))

    (loop for k being each hash-key of table
            using (hash-value v)
          do (loop for ind from (first k) to (second k)
                   do (incf (nth ind bucket) v)))

    (concatenate 'string
                 (loop for ind from 0 below (length s)
                       collect (char-convert (nth ind s) (nth ind bucket))))))

(defun char-convert (c offset)
  (code-char (+ (mod (+ (- (char-code c) 97) (mod offset 26)) 26) 97)))

(assert (string= "ace" (shifting-letters "abc" '((0 1 0) (1 2 1) (0 2 1)))))
(assert (string= "catz" (shifting-letters "dztz" '((0 0 0) (1 1 1)))))
