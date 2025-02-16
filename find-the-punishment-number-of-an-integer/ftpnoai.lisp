(defun punishment-number (n)
  (loop with re = 0
        for i from 1 to n
        for a = (split-it (* i i))
        if (can-sum-to i a)
          do (incf re (* i i))
        finally (return re)))

(defun split-it (n)
  (mapcar #'parse-integer (mapcar #'string (concatenate 'list (format nil "~a" n)))))

(defun can-sum-to (n ns)
  (if (and (= n 0) (not ns))
      (return-from can-sum-to t))

  (if (not ns)
      (return-from can-sum-to nil))

  (if (and (= n (car ns)) (= 1 (length ns)))
      (return-from can-sum-to t))

  (loop for i from 1 to (length ns)
        if (can-sum-to (- n (merge-number ns i)) (subseq ns i))
          return t))

(defun merge-number (ns ind)
  (loop for ii from 0 below ind
        sum (* (nth ii ns)
               (expt 10 (- ind  1 ii)))))

;; (merge-number '(1 2 3 4) 1)
;; (merge-number '(1 2 3 4) 2)
;; (merge-number '(1 2 3 4) 3)
;; (merge-number '(1 2 3 4) 4)

;; (can-sum-to 10 '(1 0 0))

(defun main ()
  (assert (= 182 (punishment-number 10)))
  (assert (= 1478 (punishment-number 37)))
  (assert (= 3503 (punishment-number 45))))
