(defun calculate-score (instructions values)
  (let ((visited (make-hash-table :test 'equal)))
    (setf (gethash 0 visited) t)
    (do* ((index 0)
          (score 0)
          (ins (nth index instructions) (nth index instructions))
          (vv (nth index values) (nth index values)))
         ((or (< index 0) (>= index (length instructions)))
          score)
      (cond ((string= "add" ins)
             (incf score vv)
             (incf index))
            ((string= "jump" ins)
             (incf index vv)))
      (if (gethash index visited)
          (return-from calculate-score score)
          (setf (gethash index visited) t)))))

(assert (= 1 (calculate-score '("jump" "add" "add" "jump" "add" "jump") '(2 1 3 1 -2 -3))))
(assert (= 0 (calculate-score '("jump" "add" "add") '(3 1 1))))
(assert (= 0 (calculate-score '("jump") '(0))))
