(defun largest-combination (candidates)
  (loop with cc = (mapcar #'to-bit candidates)
        for ind from 0 below 24
        maximize (count #\1 (mapcar (lambda (c) (nth ind c)) cc))
        ))

(defun to-bit (n)
  (let ((re (make-list 24 :initial-element #\0)))
    (reverse (subseq (append (reverse (concatenate 'list (format nil "~B" n))) re) 0 24))))

(defun main ())
