(defun find-relative-ranks (score)
  (let ((x (make-list 1000001 :initial-element nil))
        (len (length score)))
    
    (loop for i from 0 below len
          do (setf (nth (nth i score) x) i))

    (let ((res (make-list len :initial-element nil))
          (a 1))
      (loop for xx in (reverse x)
            when xx
              do (cond ((= a 1)
                        (setf (nth xx res) "Gold Medal"))
                       ((= a 2)
                        (setf (nth xx res) "Silver Medal"))
                       ((= a 3)
                        (setf (nth xx res) "Bronze Medal"))
                       (t (setf (nth xx res) (format nil "\"~a\"" a))))
              and do (incf a)
            finally (return res)))
    ))

(defun main ())
