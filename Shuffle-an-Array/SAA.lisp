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

(defun Shuffle (nums)
  (let* ((len (length nums))
         (random-list (gen-random-num len len)))    
    (loop for i in random-list
       collect (nth i nums)))
  )


