;;; lru-cache inspired by python version.
(defmacro lru-cache ((&rest keys) (keyword name vars &body body))
  "maybe screw up &rest keyword in lambda list"
  (declare (ignore keyword))
  (let ((table (gensym))
        (val (gensym))
        (call-vars-chain (call-lambda-list
                          (multiple-value-list
                           (parse-lambda-list vars)))))
    `(let ((,table (make-hash-table :test 'equal)))
       (defun ,name ,vars
         (let ((,val (gethash (list ,@keys) ,table)))
           (if ,val (return-from ,name ,val)))
         (labels ((inner-fake ,vars
                    ,@body))
           (let ((result (inner-fake ,@call-vars-chain)))
             (setf (gethash (list ,@keys) ,table) result)
             result))))))

(defun parse-lambda-list (l)
  "parse lambda list of function"
  (let (args optionals keys)
    (do ((flag 'args)
         (l l (cdr l)))
        ((not l) (values
                  (reverse args)
                  (reverse optionals)
                  (reverse keys)))
      (tagbody
         (case (car l)
           ('&optional (setf flag 'optionals) (go end))
           ('&key (setf flag 'keys) (go end))
           ('&rest (setf l (cdr l)) (go end)))
         
         (case flag
           ('args (push (car l) args))
           ('optionals (push (car l) optionals))
           ('keys (push (car l) keys)))
       end))))

(defun call-lambda-list (lambda-plist)
  "input result which multiple-value-list from parse-lambda-list"
  (let ((args (car lambda-plist))
        (optionals (cadr lambda-plist))
        (keys (caddr lambda-plist))
        (result '()))
    (append result
            args  ;; normal args
            (loop ;; optionals
                  for i in optionals
                  if (eq 'cons (type-of i))
                    collect (car i) into cache
                  else
                    collect i into cache
                  finally (return cache))
            (loop
              with cache = '()
              for i in keys
              if (eq 'cons (type-of i))
                do (push (intern (symbol-name (car i)) "KEYWORD") cache)
                and do (push (car i) cache)
              else
                do (push (intern (symbol-name i) "KEYWORD") cache)
                and do (push i cache)
              finally (return (reverse cache))))))

;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;
;;;;;;;;;;;;;;;;;;;

(defun max-size-slices (slice)
  (max-inner slice
             0
             (1- (length slice))
             (floor (/ (length slice) 3))
             1))


(defun max-inner (slice i j left &optional (cycle 0))
  (declare (list slice))
  (cond
    ((= 1 left)
     (return-from max-inner
       (apply #'max
              (subseq slice i (1+ j)))))
    ((zerop left)
     (return-from max-inner 0))

    ((< (- j i -1) (1- (* 2 left)))
     (return-from max-inner most-negative-fixnum)))
  
  (the fixnum
       (max (+ (nth j slice)
               (max-inner slice (+ i cycle) (- j 2) (1- left)))
            (max-inner slice i (- j 1) left))))


(lru-cache (i j left)
    (defun max-inner (slice i j left &optional (cycle 0))
      (declare (list slice))
      (cond
        ((= 1 left)
         (return-from max-inner
           (apply #'max
                  (subseq slice i (1+ j)))))
        ((zerop left)
         (return-from max-inner 0))

        ((< (- j i -1) (1- (* 2 left)))
         (return-from max-inner most-negative-fixnum)))
      (the fixnum
           (max (+ (nth j slice)
                   (max-inner slice (+ i cycle) (- j 2) (1- left)))
                (max-inner slice i (- j 1) left)))
      ))

(defun main ()
  (= 150
     (max-size-slices '(7 8 5 6 9 10 1 6 5 10 8 10 5 4 7 2 8 5 9 7 5 9 3 7 7 2 2 10 7 6 4 6 5 7 7 9 6 8 10 7 5 7 2 5 4 9 6 10 10 2 10))))
