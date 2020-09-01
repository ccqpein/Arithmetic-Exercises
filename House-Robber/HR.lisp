(defun rob (nums)
  (labels ((inner-func (nums start len table)
             (let (v p result)
               (multiple-value-setq (v p) (gethash start table))
               (cond (p (setf result v))

                     ((eql start (1- len))
                      (setf (gethash start table) (nth start nums)
                            result (nth start nums)))

                     ((eql start len)
                      (setf (gethash start table) 0
                            result 0))

                     (t
                      (let (a)
                        (setf a (max (+ (nth start nums)
                                        (inner-func nums (+ 2 start) len table))
                                     (apply #'max
                                            (loop
                                              for x from (1+ start) to (1- len)
                                              collect (inner-func nums x len table))))
                              (gethash start table) a
                              result a))))
               result)))
    (let ((table (make-hash-table))
          (len (length nums)))
      (inner-func nums 0 len table))))

;;:= TODO
;;(defun rob2 (nums &aux))

(defun main ()
  (eql (rob '(2 7 9 3 1)) 12)
  (eql (rob '(1 2 3 1)) 4)
  (eql (rob '(183 219 57 193 94 233 202 154 65 240 97 234 100 249 186 66 90 238 168 128 177 235 50 81 185 165 217 207 88 80 112 78 135 62 228 247 211)) 3365))
