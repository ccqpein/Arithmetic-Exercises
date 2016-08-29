;;; switch the design

(setf *random-state* (make-random-state t))

(defun gen-random-num (n times)
  "return result is list. n is number limit, times is number of results. No same number in the result list"
  (let ((result '())
        (x))
    (dotimes (i times result)
      (tagbody
         (setf x (random n *random-state*))
         (go tag-b)
       tag-a
         (setf x (random n *random-state*))
         (go tag-b)
       tag-b
         (if (find x result)
             (go tag-a)
             (go tag-c))
       tag-c
         (setf result (append result (list x)))))
    ))

(defvar solution (make-hash-table))

(defun constructor (nums)
  (loop for i from 0 to (- (length nums) 1)
     do (setf (gethash i solution) (nth i nums))))

(defun reset (solution)
  (loop for i from 0 to (- (hash-table-count solution) 1)
     collect (gethash i solution)))

(defun shuffle (solution)
  (loop for i in (gen-random-num (hash-table-count solution) (hash-table-count solution))
     collect (gethash i solution)))
