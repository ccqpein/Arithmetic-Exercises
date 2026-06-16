#|
Get the map of ballon. Every 2 gonna make around ballon (1) popped in the next step

Get the solution that return if all ballon gonna popped

like below example will return the true

[
  [2 1 0 1]
  [1 1 0 2]
  [1 0 0 0]
]

below will return false, because the 2 point won't affect those three 1 (island)

[
  [2 1 1 0 0]
  [1 1 0 1 1]
  [1 0 0 0 1]
]
|#


(defun helper (m this-two-ind cache)
  (setf (gethash this-two-ind cache) t)
  (let ((r (first this-two-ind))
        (c (second this-two-ind)))
    (loop for (r-o c-o) in '((-1 0) (1 0) (0 -1) (0 1))
          when (and (<= 0 (+ r r-o) (1- (length m)))
                    (<= 0 (+ c c-o) (1- (length (first m)))))
          do (when (and (= 1 (nth (+ c c-o) (nth (+ r r-o) m)))
                        (not (gethash (list (+ r r-o) (+ c c-o)) cache)))
               (helper m (list (+ r r-o) (+ c c-o)) cache )))))

(defun solution (m)
  (let* ((row-len (length m))
         (col-len (length (first m)))
         (all-two-and-ones-count 0)
         (all-twos-inds (loop for r from 0 below row-len
                              append (loop for c from 0 below col-len
                                           when (or (= 2 (nth c (nth r m)))
                                                    (= 1 (nth c (nth r m))))
                                             do (incf all-two-and-ones-count)
                                           if (= 2 (nth c (nth r m)))
                                             collect (list r c))))
         (cache (make-hash-table :test 'equal)))

    (format t "all-twos-inds ~a~%" all-twos-inds)
    (loop for two-ind in all-twos-inds
          do (helper m two-ind cache))

    (format t "cache map count ~a~%" (hash-table-count cache))
    (= all-two-and-ones-count (hash-table-count cache))))

(defun main ()
  (let ((case0 '((2 1 0 1)
                 (1 1 0 2)
                 (1 0 0 0))))
    (assert (solution case0)))

  (let ((case0 '((2 1 1 0 0)
                 (1 1 0 1 1)
                 (1 0 0 0 1))))
    (assert (not (solution case0)))))
