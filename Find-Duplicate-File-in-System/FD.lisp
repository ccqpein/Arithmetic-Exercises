(ql:quickload 'split-sequence)

(defun find-duplicate (input)
  (let ((table (make-hash-table :test 'equal))
        )
    (do* ((all-files input (cdr all-files))
          (this-path (car all-files) (car all-files))
          )
         ((not this-path) table)
      (let* ((split-path-and-files (split-sequence:split-sequence #\Space this-path))
             (path (car split-path-and-files))
             (files (cdr split-path-and-files))
             )
        (loop
           for f in files
           do (let ((content (split-sequence:split-sequence-if
                              #'(lambda (x) (or (eql x #\()
                                                (eql x #\))))
                              f))
                    )
                (setf (gethash (cadr content) table)
                      (append (gethash (cadr content) table '())
                              (list (concatenate 'string path "/" (car content)))))))
        ))))

(defvar result (find-duplicate '("root/a 1.txt(abcd) 2.txt(efgh)"
                                 "root/c 3.txt(abcd)"
                                 "root/c/d 4.txt(efgh)"
                                 "root 4.txt(efgh)")))

;; (loop
;;    for k being the hash-keys of result
;;    using (hash-value v)
;;    do (print (list k v)))

(loop
   for v being each hash-value in result
   do (print v))
