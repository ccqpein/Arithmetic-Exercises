;; borrow code from https://github.com/ccqpein/CLisp-toolbox-ccQ/blob/a731384e7fb7ffcf02283b8ae90caf7c26ac3bc3/general-tool.lisp#L38
(defun update-hash (key hash-table fun &rest args)
  "update hash value conveniently"
  (setf (gethash key hash-table)
        (apply fun (gethash key hash-table) args)
        ))

;; borrow code from https://github.com/ccqpein/CLisp-toolbox-ccQ/blob/a731384e7fb7ffcf02283b8ae90caf7c26ac3bc3/general-tool.lisp#L44
(defun pprint-hash-table (ht)
  (loop for key being the hash-keys of ht
     using (hash-value value)
     do (format t "~S => ~S~%" key value)))

(defun find-LHS (nums)
  (let ((dic (make-hash-table)))
    (loop for num in nums
       if (multiple-value-bind (val bool) (gethash num dic) bool)
       do (update-hash num dic #'1+)
       else
       do (setf (gethash num dic) 1))
    
    (let ((sortedDic (sort (loop for key being the hash-keys of dic
                              using (hash-value value)
                              collect (list key value))
                           #'< :key #'car))
          (result 0))
      (loop for ind from 1 to (1- (length sortedDic))
         when (and (= 1 (- (car (nth ind sortedDic))
                           (car (nth (1- ind) sortedDic))))
                   (< result (+ (cadr (nth ind sortedDic))
                                (cadr (nth (1- ind) sortedDic)))))
         do (setf result (+ (cadr (nth ind sortedDic))
                            (cadr (nth (1- ind) sortedDic)))))
      result)))

(print (find-LHS '(1 3 2 2 5 2 3 7)))
