(defvar nums '(1 2 3))
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

(defun constructor (nums)
  (loop for n in nums
     for i from 0 to (- (length nums) 1)
       collect (list i n))
  )

(defun shuffle (nums)
  (let* ((len (length nums))
         (rendomL (gen-random-num len len)))
    (loop for i in rendomL
         collect (second (nth i nums)))
    ))
